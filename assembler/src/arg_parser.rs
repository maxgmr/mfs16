//! Parse command-line arguments for the assembler.
use camino::Utf8PathBuf;
use clap::{ArgGroup, Parser};

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "mfsa")]
#[command(author)]
#[command(about = "Assembler for MFS-16 assembly.")]
pub struct Cli {
    /// The list of files to assemble.
    pub files: Vec<Utf8PathBuf>,
}
