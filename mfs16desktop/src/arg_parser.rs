use camino::Utf8PathBuf;
use clap::Parser;

use crate::utils;

const DEFAULT_FPS: f32 = 30.0;

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
    #[clap(short = 'd', long = "debug", group = "dbg")]
    pub debug: bool,

    /// The strong debug flag. Set this flag to print even more debug messages.
    #[clap(short = 'D', long = "strongdebug", group = "dbg")]
    pub strong_debug: bool,

    /// The CPU-only debug flag. Set this to ensure the debugger only records CPU states.
    #[clap(short = 'c', long = "cpudebug", group = "dbg")]
    pub cpu_debug: bool,

    /// The keyboard debug flag. Set this to print pressed and released keys to the console.
    #[clap(short = 'k', long = "keyboard")]
    pub kb_debug: bool,

    /// Set the desired FPS.
    #[clap(default_value_t = DEFAULT_FPS, short = 'f', long = "fps")]
    pub fps: f32,
}
