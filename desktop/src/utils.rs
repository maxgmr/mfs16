use std::{env, fs};

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre::{self, eyre};
use directories::ProjectDirs;

/// String displaying the package version, build date, & system OS version.
const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")\r\n",
    env!("VERGEN_SYSINFO_OS_VERSION"),
);

/// String displaying the total memory used on the system to run the build.
const TOTAL_MEMORY: &str = env!("VERGEN_SYSINFO_TOTAL_MEMORY");

/// Get the version, author info, and directories of the package.
pub fn info() -> String {
    let authors = clap::crate_authors!();
    format!(
        "{VERSION_MESSAGE}
Authors:\t\t\t{authors}
Configuration Directory:\t{}
Total Memory:\t\t\t{}",
        config_dir().unwrap(),
        TOTAL_MEMORY,
    )
}

/// Ensure the config directory is properly set up, returning the path to the directory.
pub fn config_dir_setup() -> eyre::Result<Utf8PathBuf> {
    // Create the directory where configuration data is stored if it doesn't already exist.
    let dir = config_dir()?;

    if fs::metadata(&dir).is_err() {
        fs::create_dir_all(&dir)?;
    }

    // Create a default config file if it doesn't already exist.
    // TODO

    Ok(dir)
}

fn config_dir() -> eyre::Result<Utf8PathBuf> {
    if let Some(path) = get_env_var_path("CONFIG") {
        // Prioritise environment variables.
        Ok(path)
    } else if let Some(proj_dirs) = project_dir() {
        // Second priority: XDG-standardised local directory.
        match Utf8PathBuf::from_path_buf(proj_dirs.config_local_dir().to_path_buf()) {
            Ok(utf8_path_buf) => Ok(utf8_path_buf),
            Err(_) => Err(eyre!(
                "Path to config directory is not a valid UTF-8 sequence."
            )),
        }
    } else {
        Err(eyre!("No config file found."))
    }
}

fn project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from("ca", "maxgmr", env!("CARGO_PKG_NAME"))
}

fn get_env_var_path(suffix: &str) -> Option<Utf8PathBuf> {
    env::var(format!("{}_{}", pkg_name_constant_case(), suffix))
        .ok()
        .map(Utf8PathBuf::from)
}

fn pkg_name_constant_case() -> String {
    env!("CARGO_PKG_NAME").to_uppercase().to_string()
}

/// Expand the given file path.
pub fn expand_path<P: AsRef<Utf8Path>>(path: P) -> eyre::Result<Utf8PathBuf> {
    Ok(Utf8PathBuf::from(&shellexpand::full(&path.as_ref())?))
}
