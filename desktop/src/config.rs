use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre;
use config::{self, Config};
use sdl2::keyboard::Scancode;
use serde::Deserialize;

use crate::scancodes;

const DEFAULT_CONFIG_NAME: &str = "default";
const CONFIG_NAME: &str = "config";

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct KeyBindings {
    #[serde(with = "scancodes")]
    pub exit: Scancode,
}

/// The user configuration settings.
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct UserConfig {
    pub key_bindings: KeyBindings,
}
impl UserConfig {
    /// Load a new [UserConfig], overwriting default values with any custom-set values.
    pub fn new(config_dir: &Utf8Path) -> eyre::Result<Self> {
        let user_config = Config::builder()
            // Get default values
            .add_source(
                config::File::with_name(
                    [&config_dir, &DEFAULT_CONFIG_NAME.into()]
                        .iter()
                        .collect::<Utf8PathBuf>()
                        .as_str(),
                )
                .required(true),
            )
            // Overwrite with any user-set values
            .add_source(
                config::File::with_name(
                    [&config_dir, &CONFIG_NAME.into()]
                        .iter()
                        .collect::<Utf8PathBuf>()
                        .as_str(),
                )
                .required(false),
            )
            .build()?;

        Ok(user_config.try_deserialize::<UserConfig>()?)
    }

    /// Directly access the "Exit" scancode.
    pub fn exit_scancode(&self) -> &Scancode {
        &self.key_bindings.exit
    }
}
