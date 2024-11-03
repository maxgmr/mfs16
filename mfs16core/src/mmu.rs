//! Memory management unit. Responsible for memory reads and writes across all components of the
//! system.
use std::default::Default;

use crate::{
    gpu::Gpu,
    helpers::{combine_u16_le, combine_u8_le},
    memory::Memory,
    RAM_OFFSET, RAM_SIZE, ROM_OFFSET, ROM_SIZE,
};

/// This byte is returned when the memory can't be read for any reason.
pub const NOT_READABLE_BYTE: u8 = 0xFF;

const ROM_END: usize = ROM_OFFSET + ROM_SIZE;
const RAM_END: usize = RAM_OFFSET + RAM_SIZE;

/// The memory management unit. Routes reads/writes and controls computer state.
#[derive(Debug, PartialEq, Clone)]
pub struct Mmu {
    /// The read-only memory of the computer.
    pub rom: Memory,
    /// The random-access memory of the computer.
    pub ram: Memory,
    /// The graphics processing unit of the computer.
    pub gpu: Gpu,
}
impl Mmu {
    /// Create a new memory management unit.
    pub fn new() -> Self {
        Self {
            rom: Memory::new_empty(ROM_SIZE, true, false),
            ram: Memory::new_empty(RAM_SIZE, true, true),
            ..Self::default()
        }
    }

    /// Read a byte from a given address.
    pub fn read_byte(&self, address: u32) -> u8 {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END => self.rom.read_byte(address - ROM_OFFSET as u32),
            RAM_OFFSET..RAM_END => self.ram.read_byte(address - RAM_OFFSET as u32),
            _ => NOT_READABLE_BYTE,
        }
    }

    /// Write a byte to a given address.
    pub fn write_byte(&mut self, address: u32, value: u8) {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END => self.rom.write_byte(address - ROM_OFFSET as u32, value),
            RAM_OFFSET..RAM_END => self.ram.write_byte(address - RAM_OFFSET as u32, value),
            _ => {}
        };
    }

    /// Read a word starting at a given address.
    pub fn read_word(&self, address: u32) -> u16 {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END => self.rom.read_word(address - ROM_OFFSET as u32),
            RAM_OFFSET..RAM_END => self.ram.read_word(address - RAM_OFFSET as u32),
            _ => combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
        }
    }

    /// Write a word to a given address.
    pub fn write_word(&mut self, address: u32, value: u16) {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END => self.rom.write_word(address - ROM_OFFSET as u32, value),
            RAM_OFFSET..RAM_END => self.ram.write_word(address - RAM_OFFSET as u32, value),
            _ => {}
        };
    }

    /// Read a double word starting at a given address.
    pub fn read_dword(&self, address: u32) -> u32 {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END => self.rom.read_dword(address - ROM_OFFSET as u32),
            RAM_OFFSET..RAM_END => self.ram.read_dword(address - RAM_OFFSET as u32),
            _ => combine_u16_le(
                combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
                combine_u8_le(self.read_byte(address + 2), self.read_byte(address + 3)),
            ),
        }
    }

    /// Write a double word to a given address.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END => self.rom.write_dword(address - ROM_OFFSET as u32, value),
            RAM_OFFSET..RAM_END => self.ram.write_dword(address - RAM_OFFSET as u32, value),
            _ => {}
        };
    }
}
impl Default for Mmu {
    fn default() -> Self {
        Self {
            rom: Memory::new_empty(ROM_SIZE, true, false),
            ram: Memory::new_empty(RAM_SIZE, true, true),
            gpu: Gpu::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_mmu() {
        let mut mmu = Mmu::default();

        // Test ROM and RAM
        for i in 0..(RAM_END as u32) {
            mmu.write_byte(i, i as u8);
        }

        for i in 0..(RAM_END as u32) {
            if ((i as usize) < ROM_END) && ((i as usize) >= ROM_OFFSET) {
                assert_eq!(mmu.read_byte(i), 0);
            } else if ((i as usize) < RAM_END) && ((i as usize) >= RAM_OFFSET) {
                assert_eq!(mmu.read_byte(i), i as u8);
            }
        }
    }
}
