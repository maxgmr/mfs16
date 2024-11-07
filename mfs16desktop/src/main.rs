use std::{fs::File, io::Read};

use camino::Utf8Path;
use clap::Parser;
use color_eyre::eyre;
use mfs16core::{Addr, Computer};

mod arg_parser;
mod config;
mod debug;
mod emulator;
mod palette;
mod scancodes;
mod utils;

use arg_parser::Cli;
use config::UserConfig;
use emulator::run_emulator;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // Parse CLI args
    let mut args = Cli::parse();
    if args.strong_debug {
        args.debug = true;
    }

    // Set up directories
    let config_dir = utils::config_dir_setup()?;

    // Load config
    let config = UserConfig::new(&config_dir)?;

    // Create a new computer
    let mut computer = Computer::new(args.strong_debug);
    // Load the binary into RAM
    let bytes: Vec<u8> = load_binary(&args.bin)?;
    computer.direct_write(Addr::new_default_range(0x00_0000), &bytes);

    if args.debug {
        // Enable memory debug
        computer.mmu.debug = true;
        computer.mmu.ram.debug = true;
        computer.mmu.rom.debug = true;
    }

    // Run the emulator
    run_emulator(computer, &args, &config)?;

    Ok(())
}

fn load_binary(file_path: &Utf8Path) -> eyre::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
