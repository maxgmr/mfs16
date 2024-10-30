//! Assembler for mfs16asm.
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};

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

    let mut machine_code: Vec<u8> = Vec::new();

    for path in args.files {
        let file_contents = read_file(&path)?;
        let tokens = lex(&file_contents, &path)?;
        machine_code.extend(parse(
            tokens,
            &path,
            &file_contents,
            machine_code.len(),
            args.debug,
        )?);
    }

    if let Some(output_path) = &args.output {
        file_output(output_path, machine_code)?;
    } else {
        stdout_output(machine_code)?;
    }

    Ok(())
}

fn file_output(path: &Utf8Path, machine_code: Vec<u8>) -> eyre::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(&machine_code)?;
    println!(
        "Compiled to `{}` successfully! ({} bytes)",
        path,
        machine_code.len()
    );
    Ok(())
}

fn stdout_output(machine_code: Vec<u8>) -> eyre::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(&machine_code)?;
    println!();
    Ok(())
}

fn read_file(file_path: &Utf8Path) -> eyre::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}
