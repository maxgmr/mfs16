use camino::Utf8PathBuf;
use clap::Parser;

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "mfs16")]
#[command(author)]
#[command(about = "CLI for the MFS-16 virtual computer.")]
pub struct Cli {
    /// The path to the MFS-16 binary to load.
    pub bin: Utf8PathBuf,

    /// The debug flag. Set to print debug messages.
    #[clap(short, long)]
    pub debug: bool,
}
