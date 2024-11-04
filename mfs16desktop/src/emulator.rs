use std::time::Instant;

use color_eyre::eyre::{self, eyre};
use mfs16core::{Computer, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{event::Event, pixels::PixelFormatEnum, rect::Rect, render::TextureAccess};
use text_io::read;

use crate::{arg_parser::Cli, config::UserConfig, palette::Rgb24Palette};

const SCALE: u32 = 2;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

const BYTES_PER_RGB24_PIXEL: usize = 3;
const PIXELS_PER_VRAM_INDEX: usize = 2;
const BYTES_PER_VRAM_INDEX: usize = BYTES_PER_RGB24_PIXEL * PIXELS_PER_VRAM_INDEX;

/// Run the [Emulator].
pub fn run_emulator(mut computer: Computer, args: &Cli, config: &UserConfig) -> eyre::Result<()> {
    // Set up sdl2
    let sdl_context = match sdl2::init() {
        Ok(sdlc) => sdlc,
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

    let mut event_pump = match sdl_context.event_pump() {
        Ok(ep) => ep,
        Err(e) => return Err(eyre!(e)),
    };

    // Get colour palette
    // TODO load this palette from config instead
    let palette = Rgb24Palette::default();

    // Create pixel array
    let mut pixels = vec![0_u8; DISPLAY_WIDTH * DISPLAY_HEIGHT * BYTES_PER_RGB24_PIXEL];
    // Create destination rectangle that is the size of the window
    let dest_rect = Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut frame_cycles = 0;
    let mut last_second = Instant::now();
    let mut cps = 0;
    let mut fps = 0;
    'main_loop: loop {
        if last_second.elapsed().as_secs() >= 1 {
            println!("-------");
            println!("FPS: {fps}\nCPS: {cps}");
            println!("-------");
            fps = 0;
            cps = 0;
            last_second = Instant::now();
        }
        // Handle keypress events.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'main_loop;
                }
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    // TODO match keypress
                    if &sc == config.exit_scancode() {
                        break 'main_loop;
                    }
                }
                _ => {}
            }
        }

        computer.cycle();
        cps += 1;
        frame_cycles += 1;
        if args.step {
            breakpoint();
        }

        // Render a new frame every 10k cycles.
        // TODO make this less hack-y
        if frame_cycles >= 10_000 {
            frame_cycles %= 10_000;

            let frame_start = Instant::now();

            // Fill the pixel array from VRAM, 6 bytes at a time
            for vram_offset in 0..(pixels.len() / BYTES_PER_VRAM_INDEX) {
                let pixels_offset = vram_offset * BYTES_PER_VRAM_INDEX;
                let vram_byte = computer.mmu.gpu.vram[vram_offset];

                // Set bytes 0, 1, & 2 from upper nibble
                pixels[pixels_offset] = palette.r(vram_byte, true);
                pixels[pixels_offset + 1] = palette.g(vram_byte, true);
                pixels[pixels_offset + 2] = palette.b(vram_byte, true);

                // Set bytes 3, 4, & 5 from lower nibble
                pixels[pixels_offset + 3] = palette.r(vram_byte, false);
                pixels[pixels_offset + 4] = palette.g(vram_byte, false);
                pixels[pixels_offset + 5] = palette.b(vram_byte, false);
            }

            // Update the texture
            if let Err(e) = texture.with_lock(None, |buffer: &mut [u8], _| {
                buffer.copy_from_slice(&pixels);
            }) {
                return Err(eyre!(e.to_string()));
            }

            sdl_canvas.clear();
            if let Err(e) = sdl_canvas.copy(&texture, None, Some(dest_rect)) {
                return Err(eyre!(e.to_string()));
            }
            sdl_canvas.present();
            fps += 1;
        }
    }
    Ok(())
}

// /// Run the [Emulator].
// pub fn run_emulator(mut computer: Computer, args: &Cli, config: &UserConfig) -> eyre::Result<()> {
//     // SDL2 setup
//     let sdl_context = match sdl2::init() {
//         Ok(sdlc) => sdlc,
//         Err(e) => return Err(eyre!(e)),
//     };
//     let video_subsystem = match sdl_context.video() {
//         Ok(vs) => vs,
//         Err(e) => return Err(eyre!(e)),
//     };
//     let window = video_subsystem
//         .window("MFS-16", WINDOW_WIDTH, WINDOW_HEIGHT)
//         .position_centered()
//         .opengl()
//         .build()?;
//     let mut canvas = window.into_canvas().present_vsync().build()?;
//
//     canvas.clear();
//     canvas.present();
//
//     let mut event_pump = match sdl_context.event_pump() {
//         Ok(ep) => ep,
//         Err(e) => return Err(eyre!(e)),
//     };
//
//     // Get the colour palette
//     let palette = Rgb24Palette::default();
//
//     let mut frame_cycles = 0;
//     'main_loop: loop {
//         // Handle keypress events.
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. } => {
//                     break 'main_loop;
//                 }
//                 Event::KeyDown {
//                     scancode: Some(sc), ..
//                 } => {
//                     // TODO match keypress
//                     if &sc == config.exit_scancode() {
//                         break 'main_loop;
//                     }
//                 }
//                 _ => {}
//             }
//         }
//
//         computer.cycle();
//         frame_cycles += 1;
//         if args.step {
//             breakpoint();
//         }
//
//         // TODO make this less hack-y
//         if frame_cycles >= 10000 {
//             frame_cycles %= 10000;
//
//             let frame_start = Instant::now();
//
//             // Clear the canvas
//             canvas.set_draw_color(Color::RGB(0, 0, 0));
//             canvas.clear();
//
//             // Update the canvas pixels from VRAM
//             for (i, byte) in computer.mmu.gpu.vram.iter().enumerate() {
//                 // Higher nibble
//                 let mut index = i * 2;
//                 canvas.set_draw_color(palette.as_sdl2_color(*byte, true));
//                 let x = (index % DISPLAY_WIDTH) as u32;
//                 let y = (index / DISPLAY_WIDTH) as u32;
//                 let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
//                 canvas.fill_rect(rect).unwrap();
//
//                 // Lower nibble
//                 index += 1;
//                 canvas.set_draw_color(palette.as_sdl2_color(*byte, false));
//                 let x = (index % DISPLAY_WIDTH) as u32;
//                 let y = (index / DISPLAY_WIDTH) as u32;
//                 let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
//                 canvas.fill_rect(rect).unwrap();
//             }
//
//             canvas.present();
//         }
//     }
//     Ok(())
// }

fn breakpoint() {
    println!("Enter any text to continue...");
    let _: String = read!();
}
