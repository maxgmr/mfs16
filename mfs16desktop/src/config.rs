use std::default::Default;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre;
use config::{self, Config};
use sdl2::keyboard::Scancode;
use serde::{Deserialize, Serialize};

use crate::{
    debug::{BreakCriteria, MemRange},
    scancodes,
    utils::expand_path,
};

pub const DEFAULT_CONFIG_NAME: &str = "default";
pub const DEFAULT_CONFIG_EXT: &str = "toml";
pub const CONFIG_NAME: &str = "config";

/// The path settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PathSettings {
    pub data_path: Option<Utf8PathBuf>,
}

/// The key bindings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyBindings {
    #[serde(with = "scancodes")]
    pub exit: Scancode,
}

/// The debugger settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DebuggerSettings {
    pub break_criteria: BreakCriteria,
    pub mem_ranges: Vec<MemRange>,
    pub history_size: usize,
}

/// The user configuration settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserConfig {
    pub path_settings: PathSettings,
    pub key_bindings: KeyBindings,
    pub debugger_settings: DebuggerSettings,
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

        user_config
            .try_deserialize::<UserConfig>()?
            .expand_file_paths()
    }

    /// Create a default [UserConfig], serialized as a TOML string.
    pub fn serialized_default() -> eyre::Result<String> {
        Ok(toml::to_string(&Self::default())?)
    }

    /// Directly access the "Exit" scancode.
    pub fn exit_scancode(&self) -> &Scancode {
        &self.key_bindings.exit
    }

    /// Expand any files paths in the config.
    fn expand_file_paths(mut self) -> eyre::Result<Self> {
        if let Some(data_path) = &self.path_settings.data_path {
            self.path_settings.data_path = Some(expand_path(data_path)?);
        }
        Ok(self)
    }
}
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            path_settings: PathSettings { data_path: None },
            key_bindings: KeyBindings {
                exit: Scancode::Escape,
            },
            debugger_settings: DebuggerSettings {
                break_criteria: BreakCriteria {
                    pc_list: Vec::new(),
                    instr_list: Vec::new(),
                },
                mem_ranges: Vec::new(),
                history_size: 16,
            },
        }
    }
}
