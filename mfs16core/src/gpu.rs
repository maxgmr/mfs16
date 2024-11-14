//! The virtual GPU hardware.
use std::default::Default;

use crate::VRAM_SIZE;

#[derive(Debug, Clone, PartialEq)]
pub struct Gpu {
    /// Video RAM responsible for storing the pixel data of the computer. Each pixel takes up one
    /// nibble of space.
    pub vram: [u8; Self::VRAM_SIZE],
}
impl Gpu {
    /// This GPU's VRAM size.
    pub const VRAM_SIZE: usize = VRAM_SIZE;

    /// Write a double word from VRAM starting at the given address.
    pub fn write_dword(&mut self, address: u32, dword: u32) {
        let end = address + 3;
        self.vram[(address as usize)..=(end as usize)].copy_from_slice(&dword.to_le_bytes());
    }

    /// Read a double word from VRAM starting at the given address.
    pub fn read_dword(&self, address: u32) -> u32 {
        let end = address + 3;
        <u32>::from_le_bytes(
            self.vram[(address as usize)..=(end as usize)]
                .try_into()
                .expect("Failed to read word: slice with incorrect length"),
        )
    }

    /// Write a word to VRAM starting at the given address.
    pub fn write_word(&mut self, address: u32, word: u16) {
        let end = address + 1;
        self.vram[(address as usize)..=(end as usize)].copy_from_slice(&word.to_le_bytes());
    }

    /// Read a word from VRAM starting at the given address.
    pub fn read_word(&self, address: u32) -> u16 {
        let end = address + 1;
        <u16>::from_le_bytes(
            self.vram[(address as usize)..=(end as usize)]
                .try_into()
                .expect("Failed to read word: slice with incorrect length"),
        )
    }

    /// Write a byte to VRAM at the given address.
    pub fn write_byte(&mut self, address: u32, byte: u8) {
        self.vram[address as usize] = byte;
    }

    /// Read a byte from VRAM at the given address.
    pub fn read_byte(&self, address: u32) -> u8 {
        self.vram[address as usize]
    }
}
impl Default for Gpu {
    /// Default = black screen.
    fn default() -> Self {
        Self {
            vram: [0x00; Self::VRAM_SIZE],
        }
    }
}
