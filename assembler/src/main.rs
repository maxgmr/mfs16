//! Assembler for mfs16asm.
use std::{fs::File, io::Read};

use camino::Utf8Path;
use clap::Parser;
use color_eyre::eyre::{self, eyre};

mod arg_parser;
mod asm_lexer;
mod asm_parser;

use arg_parser::Cli;
use asm_lexer::lex;
use asm_parser::parse;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    if args.files.is_empty() {
        return Err(eyre!("No input files given."));
    }

    for path in args.files {
        let file_contents = read_file(&path)?;
        let tokens = lex(&file_contents, &path)?;
        let machine_code = parse(tokens, &path, &file_contents, args.debug)?;
        println!("{machine_code:?}");
    }

    Ok(())
}

fn read_file(file_path: &Utf8Path) -> eyre::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}
