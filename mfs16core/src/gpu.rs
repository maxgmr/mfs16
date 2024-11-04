//! The virtual GPU hardware.
use std::default::Default;

use crate::{
    helpers::{combine_u16_le, combine_u8_le, split_dword, split_word},
    VRAM_SIZE,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Gpu {
    /// Video RAM responsible for storing the pixel data of the computer. Each pixel takes up one
    /// nibble of space.
    pub vram: [u8; Self::VRAM_SIZE],
    /// Frame interrupt flag. Is collected by the MMU during the next cycle.
    pub frame_interrupt: bool,
}
impl Gpu {
    /// This GPU's VRAM size.
    pub const VRAM_SIZE: usize = VRAM_SIZE;

    /// Consume the frame interrupt flag.
    pub fn consume_frame_interrupt(&mut self) -> bool {
        let flag = self.frame_interrupt;
        self.frame_interrupt = false;
        flag
    }

    /// Write a double word from VRAM starting at the given address.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        let (high_word, low_word) = split_dword(value);
        self.write_word(address, low_word);
        self.write_word(address + 2, high_word);
    }

    /// Read a double word from VRAM starting at the given address.
    pub fn read_dword(&self, address: u32) -> u32 {
        combine_u16_le(
            combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
            combine_u8_le(self.read_byte(address + 2), self.read_byte(address + 3)),
        )
    }

    /// Write a word to VRAM starting at the given address.
    pub fn write_word(&mut self, address: u32, word: u16) {
        let (high_byte, low_byte) = split_word(word);
        self.write_byte(address, low_byte);
        self.write_byte(address + 1, high_byte);
    }

    /// Read a word from VRAM starting at the given address.
    pub fn read_word(&self, address: u32) -> u16 {
        combine_u8_le(self.read_byte(address), self.read_byte(address + 1))
    }

    /// Write a byte to VRAM at the given address.
    pub fn write_byte(&mut self, address: u32, byte: u8) {
        self.check_address_ok(address);
        self.vram[address as usize] = byte;
    }

    /// Read a byte from VRAM at the given address.
    pub fn read_byte(&self, address: u32) -> u8 {
        self.check_address_ok(address);
        self.vram[address as usize]
    }

    fn check_address_ok(&self, address: u32) {
        if (address as usize) >= Self::VRAM_SIZE {
            panic!("Illegal VRAM read at address {:#010X}.", address);
        }
    }
}
impl Default for Gpu {
    /// Default = black screen.
    fn default() -> Self {
        Self {
            vram: [0x00; Self::VRAM_SIZE],
            frame_interrupt: false,
        }
    }
}