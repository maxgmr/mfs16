use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::{self, eyre};
use crossbeam::channel;
use mfs16core::{Computer, Interrupt, CLOCK_FREQ, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event,
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Canvas, Texture, TextureAccess},
    video::Window,
};

use crate::{arg_parser::Cli, config::UserConfig, debug::Debugger, palette::Rgb24Palette};

const SCALE: u32 = 2;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

const BYTES_PER_RGB24_PIXEL: usize = 3;
const PIXELS_PER_VRAM_INDEX: usize = 2;
const BYTES_PER_VRAM_INDEX: usize = BYTES_PER_RGB24_PIXEL * PIXELS_PER_VRAM_INDEX;

const DEBUG_LOG_NAME: &str = "debug.log";

/// Run the [Emulator].
pub fn run_emulator(
    computer: Computer,
    args: &Cli,
    config: &UserConfig,
    data_dir: &Utf8Path,
) -> eyre::Result<()> {
    let s_per_frame = 1.0 / args.fps;
    let cycles_per_frame: u32 = (s_per_frame * (CLOCK_FREQ as f32)) as u32;
    let frame_duration = Duration::from_secs_f32(s_per_frame);
    let emu_frame_duration = frame_duration;

    let debug = args.debug;
    let strong_debug = args.strong_debug;
    let cpu_debug = args.cpu_debug;

    let break_criteria = config.debugger_settings.break_criteria.clone();
    let mem_ranges = config.debugger_settings.mem_ranges.clone();
    let history_size = config.debugger_settings.history_size;

    // Channel to send graphics data to the renderer thread
    let (vram_sender, vram_receiver) = channel::bounded(2);
    // Channel to signal frame updates to the emulation thread
    let (frame_sender, frame_receiver): (
        channel::Sender<Option<()>>,
        channel::Receiver<Option<()>>,
    ) = channel::bounded(2);

    // Atomic flag to signal program quit
    let should_quit = Arc::new(AtomicBool::new(false));
    let emu_should_quit = Arc::clone(&should_quit);

    // Start the emulation thread
    let emu_thread = std::thread::spawn(move || {
        // Move the computer into the thread
        let mut computer = computer;

        // Don't want to spam console if too slow
        let mut too_slow_printed = false;

        // Set up debugger
        let mut debugger = Debugger::new(break_criteria, mem_ranges, cpu_debug, history_size);

        while !emu_should_quit.load(Ordering::SeqCst) {
            // Check if stopped
            if computer.cpu.is_stopped {
                emu_should_quit.store(true, Ordering::SeqCst);
                break;
            }

            // Wait for frame signal from main thread
            match frame_receiver.recv() {
                // Frame signal received, continue
                Ok(Some(())) => {}
                // Program execution finished, break
                Ok(None) => {
                    emu_should_quit.store(true, Ordering::SeqCst);
                    break;
                }
                // Error; print error then break
                Err(e) => {
                    emu_should_quit.store(true, Ordering::SeqCst);
                    eprintln!("{}", eyre!(e));
                    break;
                }
            }

            let cycles_start = Instant::now();

            // Perform the CPU cycles for this frame
            for _ in 0..cycles_per_frame {
                computer.cycle();

                // Do debugging stuff if the instruction is done
                if (debug || cpu_debug) && computer.cpu.instr_is_done() {
                    debugger.add_state(&computer);
                    if debugger.criteria.is_satisfied(&computer) {
                        emu_should_quit.store(true, Ordering::SeqCst);
                        break;
                    }
                    if strong_debug {
                        println!("{}", computer.cpu)
                    }
                }
            }

            // Set the frame interrupt
            computer.mmu.set_interrupt(Interrupt::Frame);

            // Send the new VRAM state
            if let Err(e) = vram_sender.send(computer.mmu.gpu.vram.to_vec()) {
                emu_should_quit.store(true, Ordering::SeqCst);
                eprintln!("{}", eyre!("{e}"));
                break;
            }

            if !too_slow_printed && (cycles_start.elapsed() >= emu_frame_duration) {
                println!(
                    "Warning: emulator is unable to keep up with FPS!\nTime limit \
                    for {} cycles: {:?}\nActual time: {:?}",
                    cycles_per_frame,
                    emu_frame_duration,
                    cycles_start.elapsed()
                );
                too_slow_printed = true;
            }
        }

        // Execution done, send debug results (if any)
        debugger
    });

    // Set up sdl2
    let sdl_context = match sdl2::init() {
        Ok(sdlc) => sdlc,
        Err(e) => return Err(eyre!(e)),
    };

    let mut event_pump = match sdl_context.event_pump() {
        Ok(ep) => ep,
        Err(e) => return Err(eyre!(e)),
    };

    let video_subsystem = match sdl_context.video() {
        Ok(vs) => vs,
        Err(e) => return Err(eyre!(e)),
    };

    let window = video_subsystem
        .window("MFS-16", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;
    let mut sdl_canvas = window.into_canvas().build()?;

    let texture_creator = sdl_canvas.texture_creator();
    let mut texture = texture_creator.create_texture(
        PixelFormatEnum::RGB24,
        TextureAccess::Streaming,
        DISPLAY_WIDTH.try_into()?,
        DISPLAY_HEIGHT.try_into()?,
    )?;

    // Get colour palette
    // TODO load this palette from config instead
    let palette = Rgb24Palette::default();

    // Create pixel array
    let mut pixels = vec![0_u8; DISPLAY_WIDTH * DISPLAY_HEIGHT * BYTES_PER_RGB24_PIXEL];

    let mut last_second = Instant::now();
    let mut fps = 0;

    // Main thread for event handling and rendering
    while !should_quit.load(Ordering::SeqCst) {
        let frame_start = Instant::now();

        if last_second.elapsed() >= Duration::from_secs(1) {
            last_second = Instant::now();
            if args.debug {
                println!("FPS: {fps}");
            }
            fps = 0;
        }

        // Handle keypress events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    let _ = frame_sender.send(None);
                    should_quit.store(true, Ordering::SeqCst);
                    // Flush event pump
                    let _ = event_pump.poll_iter();
                    break;
                }
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    // TODO match keypress
                    if &sc == config.exit_scancode() {
                        let _ = frame_sender.send(None);
                        should_quit.store(true, Ordering::SeqCst);
                        // Flush event pump
                        let _ = event_pump.poll_iter();
                        break;
                    }
                }
                _ => {}
            }
        }

        // Render the current frame
        if let Ok(vram) = vram_receiver.try_recv() {
            render_graphics(&mut sdl_canvas, &mut pixels, &mut texture, &palette, vram);
            fps += 1;
        }

        // Send a signal to the CPU thread for this frame
        let _ = frame_sender.send(Some(()));

        // Maintain target FPS
        let current_frame_time = frame_start.elapsed();
        if current_frame_time < frame_duration {
            thread::sleep(frame_duration - current_frame_time);
        }
    }

    // Cleanup
    let _ = frame_sender.send(None);
    should_quit.store(true, Ordering::SeqCst);
    drop(frame_sender);
    match emu_thread.join() {
        Ok(debugger) => {
            if args.debug || args.cpu_debug {
                let mut debug_log_path = Utf8PathBuf::from(data_dir);
                debug_log_path.push(DEBUG_LOG_NAME);
                debugger.write_to_file(debug_log_path)?;
            }
        }
        Err(_) => return Err(eyre!("Failed to join emulation thread,")),
    }

    Ok(())
}

fn render_graphics(
    sdl_canvas: &mut Canvas<Window>,
    pixels: &mut [u8],
    texture: &mut Texture,
    palette: &Rgb24Palette,
    vram: Vec<u8>,
) {
    // Create destination rectangle that is the size of the window
    let dest_rect = Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    // Fill the pixel array from VRAM, 6 bytes at a time
    for (vram_offset, vram_byte) in vram
        .iter()
        .enumerate()
        .take(pixels.len() / BYTES_PER_VRAM_INDEX)
    {
        let pixels_offset = vram_offset * BYTES_PER_VRAM_INDEX;

        // Set bytes 0, 1, & 2 from upper nibble
        pixels[pixels_offset] = palette.r(*vram_byte, true);
        pixels[pixels_offset + 1] = palette.g(*vram_byte, true);
        pixels[pixels_offset + 2] = palette.b(*vram_byte, true);

        // Set bytes 3, 4, & 5 from lower nibble
        pixels[pixels_offset + 3] = palette.r(*vram_byte, false);
        pixels[pixels_offset + 4] = palette.g(*vram_byte, false);
        pixels[pixels_offset + 5] = palette.b(*vram_byte, false);
    }

    // Update the texture
    texture
        .with_lock(None, |buffer: &mut [u8], _| {
            buffer.copy_from_slice(pixels);
        })
        .unwrap();
    sdl_canvas.clear();
    sdl_canvas.copy(texture, None, Some(dest_rect)).unwrap();
    sdl_canvas.present();
}
