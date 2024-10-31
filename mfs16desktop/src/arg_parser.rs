use camino::Utf8PathBuf;
use clap::Parser;

use crate::utils;

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "mfs16")]
#[command(author)]
#[command(version = utils::info())]
#[command(about = "Desktop GUI for the MFS-16 virtual computer.")]
pub struct Cli {
    /// The path to the MFS-16 binary to load.
    pub bin: Utf8PathBuf,

    /// The debug flag. Set to print debug messages.
    #[clap(short, long)]
    pub debug: bool,
}
