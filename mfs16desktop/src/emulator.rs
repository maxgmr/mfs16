use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use color_eyre::eyre::{self, eyre};
use mfs16core::{Computer, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event, keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

use crate::config::UserConfig;
use Colour::*;

const SCALE: u32 = 4;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

const FPS: u32 = 60;

// fn computer_thread(state: Arc<RwLock<Computer>>, tx: Sender<()>) {
//     // TODO temp soln: update every 10k cycles
//     let mut cycle_counter: usize = 0;
//     loop {
//         {
//             let mut computer = state.write().unwrap();
//             computer.cycle();
//             cycle_counter += 1;
//         }
//
//         if cycle_counter > 10000 {
//             // Notify rendering thread that state has been updated
//             tx.send(()).unwrap();
//             cycle_counter = 0;
//         }
//     }
// }

/// Run the [Emulator].
pub fn run_emulator(mut computer: Computer, config: &UserConfig) -> eyre::Result<()> {
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
        .opengl()
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;

    canvas.clear();
    canvas.present();

    let mut event_pump = match sdl_context.event_pump() {
        Ok(ep) => ep,
        Err(e) => return Err(eyre!(e)),
    };

    // SINGLETHREADED ATTEMPT
    let mut frame_cycles = 0;
    'main_loop: loop {
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
        frame_cycles += 1;

        // TODO make this less hack-y
        if frame_cycles >= 10000 {
            frame_cycles %= 10000;

            // Clear the canvas
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // Update the canvas pixels from VRAM
            for (i, byte) in computer.mmu.gpu.vram.iter().enumerate() {
                // Higher nibble
                let mut index = i * 2;
                let colour = Colour::from_nibble(*byte, true);
                canvas.set_draw_color(colour.into_sdl_rgb());
                let x = (index % DISPLAY_WIDTH) as u32;
                let y = (index / DISPLAY_WIDTH) as u32;
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();

                // Lower nibble
                index += 1;
                let colour = Colour::from_nibble(*byte, false);
                canvas.set_draw_color(colour.into_sdl_rgb());
                let x = (index % DISPLAY_WIDTH) as u32;
                let y = (index / DISPLAY_WIDTH) as u32;
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();
            }

            canvas.present();
        }
    }

    // MULTITHREADED ATTEMPT

    // let state = Arc::new(RwLock::new(computer));
    // let (tx, rx) = mpsc::channel();
    //
    // let computer_state = Arc::clone(&state);
    // let computer_handle = thread::spawn(move || computer_thread(computer_state, tx));
    //
    // 'frame_loop: loop {
    //     let frame_start = Instant::now();
    //     // Handle keypress events.
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. } => {
    //                 break 'frame_loop;
    //             }
    //             Event::KeyDown {
    //                 scancode: Some(sc), ..
    //             } => {
    //                 // TODO match keypress
    //                 if &sc == config.exit_scancode() {
    //                     println!("ping!");
    //                     break 'frame_loop;
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //
    //     // Check for render update
    //     if rx.try_recv().is_ok() {
    //         // Get the state
    //         let computer = state.read().unwrap();
    //
    //         // Clear the canvas
    //         canvas.set_draw_color(Color::RGB(0, 0, 0));
    //         canvas.clear();
    //
    //         // Update the canvas pixels from VRAM
    //         for (i, byte) in computer.mmu.gpu.vram.iter().enumerate() {
    //             // Higher nibble
    //             let mut index = i * 2;
    //             let colour = Colour::from_nibble(*byte, true);
    //             canvas.set_draw_color(colour.into_sdl_rgb());
    //             let x = (index % DISPLAY_WIDTH) as u32;
    //             let y = (index / DISPLAY_WIDTH) as u32;
    //             let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
    //             canvas.fill_rect(rect).unwrap();
    //
    //             // Lower nibble
    //             index += 1;
    //             let colour = Colour::from_nibble(*byte, false);
    //             canvas.set_draw_color(colour.into_sdl_rgb());
    //             let x = (index % DISPLAY_WIDTH) as u32;
    //             let y = (index / DISPLAY_WIDTH) as u32;
    //             let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
    //             canvas.fill_rect(rect).unwrap();
    //         }
    //     }
    //
    //     let frame_time_diff = (1.0 / (FPS as f32)) - frame_start.elapsed().as_secs_f32();
    //     if frame_time_diff > 0.0 {
    //         thread::sleep(Duration::from_millis((frame_time_diff * 1000.0) as u64));
    //     } else {
    //         println!(
    //             "warning: can't maintain desired fps! ({:?})",
    //             frame_start.elapsed()
    //         );
    //     }
    // }
    //
    // computer_handle.join().unwrap();

    Ok(())
}

// TODO make this more flexible
enum Colour {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
    BrightBlack = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15,
}
impl Colour {
    fn from_nibble(byte: u8, is_high_nibble: bool) -> Self {
        match if is_high_nibble {
            byte >> 4
        } else {
            byte & 0x0F
        } {
            0 => Black,
            1 => Red,
            2 => Green,
            3 => Yellow,
            4 => Blue,
            5 => Magenta,
            6 => Cyan,
            7 => White,
            8 => BrightBlack,
            9 => BrightRed,
            10 => BrightGreen,
            11 => BrightYellow,
            12 => BrightBlue,
            13 => BrightMagenta,
            14 => BrightCyan,
            15 => BrightWhite,
            _ => unreachable!("Value is too big for a nibble!"),
        }
    }

    // TODO make this work with custom config palette
    fn into_sdl_rgb(self) -> Color {
        match self {
            Black => Color::RGB(0x00, 0x00, 0x00),
            Red => Color::RGB(0xDF, 0x00, 0x00),
            Green => Color::RGB(0x00, 0xDF, 0x00),
            Yellow => Color::RGB(0xDF, 0xDF, 0x00),
            Blue => Color::RGB(0x00, 0x00, 0xDF),
            Magenta => Color::RGB(0xDF, 0x00, 0xDF),
            Cyan => Color::RGB(0x00, 0xDF, 0xDF),
            White => Color::RGB(0xDF, 0xDF, 0xDF),
            BrightBlack => Color::RGB(0x40, 0x40, 0x40),
            BrightRed => Color::RGB(0xFF, 0x40, 0x40),
            BrightGreen => Color::RGB(0x40, 0xFF, 0x40),
            BrightYellow => Color::RGB(0xFF, 0xFF, 0x40),
            BrightBlue => Color::RGB(0x40, 0x40, 0xFF),
            BrightMagenta => Color::RGB(0xFF, 0x40, 0xFF),
            BrightCyan => Color::RGB(0x40, 0xFF, 0xFF),
            BrightWhite => Color::RGB(0xFF, 0xFF, 0xFF),
        }
    }
}
