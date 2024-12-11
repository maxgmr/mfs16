use std::default::Default;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::eyre;
use config::{self, Config};
use sdl2::keyboard::Scancode;
use serde::{Deserialize, Serialize};

use crate::{
    debug::{BreakCriteria, MemRange},
    palette::HexPalette,
    scancodes,
    utils::expand_path,
};

pub const DEFAULT_CONFIG_NAME: &str = "default";
pub const DEFAULT_CONFIG_EXT: &str = "toml";
pub const CONFIG_NAME: &str = "config";

/// The colour palette settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaletteSettings {
    pub preset_palette: String,
    pub custom_palette: CustomPalette,
}

/// The custom, user-defined colour palette.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomPalette {
    pub enabled: bool,
    pub palette: HexPalette,
}

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
    pub cycles_after_break: usize,
}

/// The user configuration settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserConfig {
    pub palette_settings: PaletteSettings,
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

    /// Get the currently-selected palette of the config.
    pub fn palette(&self) -> Option<HexPalette> {
        if self.palette_settings.custom_palette.enabled {
            // Use custom palette
            return Some(self.palette_settings.custom_palette.palette.clone());
        }

        // Use preset palette
        HexPalette::from_str(&self.palette_settings.preset_palette)
    }
}
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            palette_settings: PaletteSettings {
                preset_palette: String::from("default"),
                custom_palette: CustomPalette {
                    enabled: false,
                    palette: HexPalette::default(),
                },
            },
            path_settings: PathSettings { data_path: None },
            key_bindings: KeyBindings {
                exit: Scancode::Escape,
            },
            debugger_settings: DebuggerSettings {
                break_criteria: BreakCriteria {
                    pc_list: Vec::new(),
                    ei: false,
                    pc_upper_bound: None,
                    pc_lower_bound: None,
                    instr_list: Vec::new(),
                    reg_upper_bounds: Vec::new(),
                    reg_lower_bounds: Vec::new(),
                },
                mem_ranges: Vec::new(),
                history_size: 16,
                cycles_after_break: 16,
            },
        }
    }
}
