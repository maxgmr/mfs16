//! Parse command-line arguments for the assembler.
use camino::Utf8PathBuf;
use clap::Parser;

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "mfsa")]
#[command(author)]
#[command(about = "Assembler for MFS-16 assembly.")]
pub struct Cli {
    /// The list of files to assemble.
    pub files: Vec<Utf8PathBuf>,

    /// The debug flag. Set to print debug messages.
    #[clap(short, long)]
    pub debug: bool,

    /// The replace flag. Set to overwrite any existing files with the output.
    #[clap(short, long)]
    pub replace: bool,

    /// Output machine code to this file path. Leave blank to output to stdout.
    #[clap(short, long)]
    pub output: Option<Utf8PathBuf>,
}
