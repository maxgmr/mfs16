//! Assembler for mfs16asm.
use clap::Parser;
use color_eyre::eyre::{self, eyre};

mod arg_parser;
mod asm_lexer;
mod asm_parser;
mod codemap;

use crate::arg_parser::Cli;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    if args.files.is_empty() {
        return Err(eyre!("No input files given."));
    }

    Ok(())
}
