use std::default::Default;

use hex_color::HexColor;
use serde::{Deserialize, Serialize};

mod preset_palettes;

/// A 16-colour palette of RGB24 colours.
#[derive(Debug, Clone)]
pub struct Rgb24Palette {
    black: U24Colour,
    red: U24Colour,
    green: U24Colour,
    yellow: U24Colour,
    blue: U24Colour,
    magenta: U24Colour,
    cyan: U24Colour,
    white: U24Colour,
    bright_black: U24Colour,
    bright_red: U24Colour,
    bright_green: U24Colour,
    bright_yellow: U24Colour,
    bright_blue: U24Colour,
    bright_magenta: U24Colour,
    bright_cyan: U24Colour,
    bright_white: U24Colour,
}
impl Rgb24Palette {
    /// Convert a [HexPalette] into a new [Rgb24Palette].
    pub fn from_hex_palette(palette: HexPalette) -> Self {
        Self {
            black: <U24Colour>::from(palette.hex_color_from_nib(0x00, true).to_u24()),
            red: <U24Colour>::from(palette.hex_color_from_nib(0x11, true).to_u24()),
            green: <U24Colour>::from(palette.hex_color_from_nib(0x22, true).to_u24()),
            yellow: <U24Colour>::from(palette.hex_color_from_nib(0x33, true).to_u24()),
            blue: <U24Colour>::from(palette.hex_color_from_nib(0x44, true).to_u24()),
            magenta: <U24Colour>::from(palette.hex_color_from_nib(0x55, true).to_u24()),
            cyan: <U24Colour>::from(palette.hex_color_from_nib(0x66, true).to_u24()),
            white: <U24Colour>::from(palette.hex_color_from_nib(0x77, true).to_u24()),
            bright_black: <U24Colour>::from(palette.hex_color_from_nib(0x88, true).to_u24()),
            bright_red: <U24Colour>::from(palette.hex_color_from_nib(0x99, true).to_u24()),
            bright_green: <U24Colour>::from(palette.hex_color_from_nib(0xAA, true).to_u24()),
            bright_yellow: <U24Colour>::from(palette.hex_color_from_nib(0xBB, true).to_u24()),
            bright_blue: <U24Colour>::from(palette.hex_color_from_nib(0xCC, true).to_u24()),
            bright_magenta: <U24Colour>::from(palette.hex_color_from_nib(0xDD, true).to_u24()),
            bright_cyan: <U24Colour>::from(palette.hex_color_from_nib(0xEE, true).to_u24()),
            bright_white: <U24Colour>::from(palette.hex_color_from_nib(0xFF, true).to_u24()),
        }
    }

    /// Get the RGB24 value corresponding to the given nibble.
    pub fn rgb24_from_nib(&self, byte: u8, is_high_nibble: bool) -> &U24Colour {
        match if is_high_nibble {
            byte >> 4
        } else {
            byte & 0x0F
        } {
            0 => &self.black,
            1 => &self.red,
            2 => &self.green,
            3 => &self.yellow,
            4 => &self.blue,
            5 => &self.magenta,
            6 => &self.cyan,
            7 => &self.white,
            8 => &self.bright_black,
            9 => &self.bright_red,
            10 => &self.bright_green,
            11 => &self.bright_yellow,
            12 => &self.bright_blue,
            13 => &self.bright_magenta,
            14 => &self.bright_cyan,
            15 => &self.bright_white,
            _ => unreachable!("Value is too big for a nibble!"),
        }
    }
}
impl Default for Rgb24Palette {
    fn default() -> Self {
        Self::from_hex_palette(HexPalette::default())
    }
}

/// A U24 colour stored as its RGB channels.
#[derive(Debug, Clone)]
pub struct U24Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl From<u32> for U24Colour {
    fn from(value: u32) -> Self {
        Self {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
        }
    }
}

/// A 16-colour palette of defined RGB hex colours.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HexPalette {
    black: HexColor,
    red: HexColor,
    green: HexColor,
    yellow: HexColor,
    blue: HexColor,
    magenta: HexColor,
    cyan: HexColor,
    white: HexColor,
    bright_black: HexColor,
    bright_red: HexColor,
    bright_green: HexColor,
    bright_yellow: HexColor,
    bright_blue: HexColor,
    bright_magenta: HexColor,
    bright_cyan: HexColor,
    bright_white: HexColor,
}
impl HexPalette {
    /// Get the preset [HexPalette] corresponding to the given string, returning [None] if the
    /// string does not match any preset palette.
    ///
    /// String matching is case-insensitive.
    pub fn from_str<S: AsRef<str>>(str: S) -> Option<Self> {
        match str.as_ref().to_lowercase().as_str() {
            "gruvbox" => Some(preset_palettes::GRUVBOX),
            "default" => Some(preset_palettes::DEFAULT),
            _ => None,
        }
    }

    /// Get the [HexColor] corresponding to the given nibble.
    pub fn hex_color_from_nib(&self, byte: u8, is_high_nibble: bool) -> &HexColor {
        match if is_high_nibble {
            byte >> 4
        } else {
            byte & 0x0F
        } {
            0 => &self.black,
            1 => &self.red,
            2 => &self.green,
            3 => &self.yellow,
            4 => &self.blue,
            5 => &self.magenta,
            6 => &self.cyan,
            7 => &self.white,
            8 => &self.bright_black,
            9 => &self.bright_red,
            10 => &self.bright_green,
            11 => &self.bright_yellow,
            12 => &self.bright_blue,
            13 => &self.bright_magenta,
            14 => &self.bright_cyan,
            15 => &self.bright_white,
            _ => unreachable!("Value is too big for a nibble!"),
        }
    }
}
impl Default for HexPalette {
    fn default() -> Self {
        preset_palettes::DEFAULT
    }
}
