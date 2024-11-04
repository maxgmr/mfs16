use std::{
    cell::RefCell,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use color_eyre::{
    eyre::{self, eyre},
    owo_colors::OwoColorize,
};
use mfs16core::{Computer, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event,
    keyboard::Scancode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas as SdlCanvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};
use text_io::read;

use crate::{arg_parser::Cli, config::UserConfig, palette::Rgb24Palette};

const SCALE: u32 = 2;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

const FPS: u32 = 60;

// /// Run the [Emulator].
// pub fn run_emulator(mut computer: Computer, args: &Cli, config: &UserConfig) -> eyre::Result<()> {
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
//         .build()?;
//     let mut canvas = window.into_canvas().build()?;
//     let creator = canvas.texture_creator();
//     let texture =
//         creator.create_texture_target(PixelFormatEnum::RGB24, WINDOW_WIDTH, WINDOW_HEIGHT)?;
//     //
//     // canvas.clear();
//     // canvas.present();
//     //
//     // let mut event_pump = match sdl_context.event_pump() {
//     //     Ok(ep) => ep,
//     //     Err(e) => return Err(eyre!(e)),
//     // };
//     //
//     // let mut frame_cycles = 0;
//     // 'main_loop: loop {
//     //     // Handle keypress events.
//     //     for event in event_pump.poll_iter() {
//     //         match event {
//     //             Event::Quit { .. } => {
//     //                 break 'main_loop;
//     //             }
//     //             Event::KeyDown {
//     //                 scancode: Some(sc), ..
//     //             } => {
//     //                 // TODO match keypress
//     //                 if &sc == config.exit_scancode() {
//     //                     break 'main_loop;
//     //                 }
//     //             }
//     //             _ => {}
//     //         }
//     //     }
//     //
//     //     computer.cycle();
//     //     frame_cycles += 1;
//     //     if args.step {
//     //         breakpoint();
//     //     }
//     //
//     //     // TODO make this less hack-y
//     //     if frame_cycles >= 10000 {
//     //         frame_cycles %= 10000;
//     //
//     //         let frame_start = Instant::now();
//     //
//     //         // Clear the canvas
//     //         canvas.set_draw_color(Color::RGB(0, 0, 0));
//     //         canvas.clear();
//     //
//     //         // Update the canvas pixels from VRAM
//     //         for (i, byte) in computer.mmu.gpu.vram.iter().enumerate() {
//     //             // Higher nibble
//     //             let mut index = i * 2;
//     //             let colour = Colour::from_nibble(*byte, true);
//     //             canvas.set_draw_color(colour.into_sdl_rgb());
//     //             let x = (index % DISPLAY_WIDTH) as u32;
//     //             let y = (index / DISPLAY_WIDTH) as u32;
//     //             let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
//     //             canvas.fill_rect(rect).unwrap();
//     //
//     //             // Lower nibble
//     //             index += 1;
//     //             let colour = Colour::from_nibble(*byte, false);
//     //             canvas.set_draw_color(colour.into_sdl_rgb());
//     //             let x = (index % DISPLAY_WIDTH) as u32;
//     //             let y = (index / DISPLAY_WIDTH) as u32;
//     //             let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
//     //             canvas.fill_rect(rect).unwrap();
//     //         }
//     //
//     //         canvas.present();
//     //     }
//     // }
//     Ok(())
// }

/// Run the [Emulator].
pub fn run_emulator(mut computer: Computer, args: &Cli, config: &UserConfig) -> eyre::Result<()> {
    // SDL2 setup
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

    // Get the colour palette
    let palette = Rgb24Palette::default();

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
        if args.step {
            breakpoint();
        }

        // TODO make this less hack-y
        if frame_cycles >= 10000 {
            frame_cycles %= 10000;

            let frame_start = Instant::now();

            // Clear the canvas
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // Update the canvas pixels from VRAM
            for (i, byte) in computer.mmu.gpu.vram.iter().enumerate() {
                // Higher nibble
                let mut index = i * 2;
                canvas.set_draw_color(palette.as_sdl2_color(*byte, true));
                let x = (index % DISPLAY_WIDTH) as u32;
                let y = (index / DISPLAY_WIDTH) as u32;
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();

                // Lower nibble
                index += 1;
                canvas.set_draw_color(palette.as_sdl2_color(*byte, false));
                let x = (index % DISPLAY_WIDTH) as u32;
                let y = (index / DISPLAY_WIDTH) as u32;
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                canvas.fill_rect(rect).unwrap();
            }

            canvas.present();
        }
    }
    Ok(())
}

fn breakpoint() {
    println!("Enter any text to continue...");
    let _: String = read!();
}
