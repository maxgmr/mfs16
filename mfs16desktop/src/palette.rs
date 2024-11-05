use std::default::Default;

use hex_color::HexColor;
use serde::{Deserialize, Serialize};

/// A 16-colour palette of RGB24 colours.
#[derive(Debug, Clone)]
pub struct Rgb24Palette {
    black: u32,
    red: u32,
    green: u32,
    yellow: u32,
    blue: u32,
    magenta: u32,
    cyan: u32,
    white: u32,
    bright_black: u32,
    bright_red: u32,
    bright_green: u32,
    bright_yellow: u32,
    bright_blue: u32,
    bright_magenta: u32,
    bright_cyan: u32,
    bright_white: u32,
}
impl Rgb24Palette {
    /// Convert a [HexPalette] into a new [Rgb24Palette].
    pub fn from_hex_palette(palette: HexPalette) -> Self {
        Self {
            black: palette.hex_color_from_nib(0x00, true).to_u24(),
            red: palette.hex_color_from_nib(0x11, true).to_u24(),
            green: palette.hex_color_from_nib(0x22, true).to_u24(),
            yellow: palette.hex_color_from_nib(0x33, true).to_u24(),
            blue: palette.hex_color_from_nib(0x44, true).to_u24(),
            magenta: palette.hex_color_from_nib(0x55, true).to_u24(),
            cyan: palette.hex_color_from_nib(0x66, true).to_u24(),
            white: palette.hex_color_from_nib(0x77, true).to_u24(),
            bright_black: palette.hex_color_from_nib(0x88, true).to_u24(),
            bright_red: palette.hex_color_from_nib(0x99, true).to_u24(),
            bright_green: palette.hex_color_from_nib(0xAA, true).to_u24(),
            bright_yellow: palette.hex_color_from_nib(0xBB, true).to_u24(),
            bright_blue: palette.hex_color_from_nib(0xCC, true).to_u24(),
            bright_magenta: palette.hex_color_from_nib(0xDD, true).to_u24(),
            bright_cyan: palette.hex_color_from_nib(0xEE, true).to_u24(),
            bright_white: palette.hex_color_from_nib(0xFF, true).to_u24(),
        }
    }

    /// Get the RGB24 value corresponding to the given nibble.
    pub fn rgb24_from_nib(&self, byte: u8, is_high_nibble: bool) -> u32 {
        match if is_high_nibble {
            byte >> 4
        } else {
            byte & 0x0F
        } {
            0 => self.black,
            1 => self.red,
            2 => self.green,
            3 => self.yellow,
            4 => self.blue,
            5 => self.magenta,
            6 => self.cyan,
            7 => self.white,
            8 => self.bright_black,
            9 => self.bright_red,
            10 => self.bright_green,
            11 => self.bright_yellow,
            12 => self.bright_blue,
            13 => self.bright_magenta,
            14 => self.bright_cyan,
            15 => self.bright_white,
            _ => unreachable!("Value is too big for a nibble!"),
        }
    }

    /// Get the R channel of the color corresponding to the given nibble.
    pub fn r(&self, byte: u8, is_high_nibble: bool) -> u8 {
        self.channel_helper(byte, is_high_nibble, 16)
    }

    /// Get the G channel of the color corresponding to the given nibble.
    pub fn g(&self, byte: u8, is_high_nibble: bool) -> u8 {
        self.channel_helper(byte, is_high_nibble, 8)
    }

    /// Get the B channel of the color corresponding to the given nibble.
    pub fn b(&self, byte: u8, is_high_nibble: bool) -> u8 {
        self.channel_helper(byte, is_high_nibble, 0)
    }

    fn channel_helper(&self, byte: u8, is_high_nibble: bool, shift_count: u8) -> u8 {
        ((self.rgb24_from_nib(byte, is_high_nibble) >> shift_count) & 0xFF) as u8
    }
}
impl Default for Rgb24Palette {
    fn default() -> Self {
        Self::from_hex_palette(HexPalette::default())
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
        Self {
            black: HexColor::rgb(0x00, 0x00, 0x00),
            red: HexColor::rgb(0x80, 0x00, 0x00),
            green: HexColor::rgb(0x00, 0x80, 0x00),
            yellow: HexColor::rgb(0x80, 0x80, 0x00),
            blue: HexColor::rgb(0x00, 0x00, 0x80),
            magenta: HexColor::rgb(0x80, 0x00, 0x80),
            cyan: HexColor::rgb(0x00, 0x80, 0x80),
            white: HexColor::rgb(0xC0, 0xC0, 0xC0),
            bright_black: HexColor::rgb(0x80, 0x80, 0x80),
            bright_red: HexColor::rgb(0xFF, 0x00, 0x00),
            bright_green: HexColor::rgb(0x00, 0xFF, 0x00),
            bright_yellow: HexColor::rgb(0xFF, 0xFF, 0x00),
            bright_blue: HexColor::rgb(0x00, 0x00, 0xFF),
            bright_magenta: HexColor::rgb(0xFF, 0x00, 0xFF),
            bright_cyan: HexColor::rgb(0x00, 0xFF, 0xFF),
            bright_white: HexColor::rgb(0xFF, 0xFF, 0xFF),
        }
    }
}
