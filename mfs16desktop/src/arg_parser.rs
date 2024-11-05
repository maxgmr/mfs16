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

    /// The debug flag. Set this flag to print debug messages.
    #[clap(short = 'd', conflicts_with = "strong_debug")]
    pub debug: bool,

    /// The strong debug flag. Set this flag to print even more debug messages.
    #[clap(short = 'D', conflicts_with = "debug")]
    pub strong_debug: bool,
}
