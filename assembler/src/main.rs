//! Assembler for mfs16asm.
use clap::Parser;
use color_eyre::eyre::{self, eyre};

mod arg_parser;
mod asm_parser;

use crate::{arg_parser::Cli, asm_parser::AsmParser};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    if args.files.is_empty() {
        return Err(eyre!("No input files given."));
    }

    for path in args.files {
        let mut asm_parser = AsmParser::from_path(path, args.debug)?;
        asm_parser.parse_input()?;
    }

    Ok(())
}
