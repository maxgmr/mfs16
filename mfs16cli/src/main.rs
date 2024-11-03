use std::{fs::File, io::Read};

use camino::Utf8Path;
use clap::Parser;
use color_eyre::eyre;
use mfs16core::{Addr, Computer};

mod arg_parser;

use arg_parser::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();

    let mut computer = Computer::new(args.debug);

    let bytes: Vec<u8> = load_binary(&args.bin)?;
    computer.direct_write(Addr::new_default_range(0x00_0000), &bytes);

    loop {
        computer.cycle();
    }
}

fn load_binary(file_path: &Utf8Path) -> eyre::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
