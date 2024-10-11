use std::default::Default;

use crate::{
    helpers::{combine_u8_le, split_word},
    RAM_SIZE,
};

/// Random-access memory for direct interfacing with the CPU.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ram {
    /// The memory contents of the RAM.
    memory: Vec<u8>,
}
impl Ram {
    /// Create new [Ram] of the set size. All bytes are initialised to the given array.
    pub fn new(memory: Vec<u8>) -> Self {
        if memory.len() != RAM_SIZE {
            panic!(
                "Illegal RAM size. Given value is {} bytes, expected {} bytes.",
                memory.len(),
                RAM_SIZE,
            );
        }
        Self { memory }
    }

    /// Read a byte from memory at the given address.
    pub fn read_byte(&self, address: u32) -> u8 {
        check_addr(address, "read");
        self.memory[address as usize]
    }

    /// Write a byte to memory at the given address.
    pub fn write_byte(&mut self, address: u32, value: u8) {
        check_addr(address, "write");
        self.memory[address as usize] = value;
    }

    /// Read a word from memory starting at the given address.
    pub fn read_word(&self, address: u32) -> u16 {
        combine_u8_le(self.read_byte(address), self.read_byte(address + 1))
    }

    /// Write a word to memory starting at the given address.
    pub fn write_word(&mut self, address: u32, value: u16) {
        let (high_byte, low_byte) = split_word(value);
        self.write_byte(address, low_byte);
        self.write_byte(address + 1, high_byte);
    }
}
impl Default for Ram {
    /// Default: All bytes in memory initialised to 0x00.
    fn default() -> Self {
        Self::new(vec![0x00; RAM_SIZE])
    }
}

fn check_addr(address: u32, verb: &'static str) {
    if (address as usize) >= RAM_SIZE {
        panic!("Illegal memory {verb} at address {:#X}.", address);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::helpers::combine_u8_be;

    use super::*;

    #[test]
    #[should_panic(expected = "Illegal memory read at address 0x1000000.")]
    fn test_oob_read_byte() {
        let ram = Ram::default();
        ram.read_byte(0x100_0000);
    }

    #[test]
    #[should_panic(expected = "Illegal memory read at address 0x1000000.")]
    fn test_oob_read_word() {
        let ram = Ram::default();
        ram.read_word(0xFF_FFFF);
    }

    #[test]
    #[should_panic(expected = "Illegal memory write at address 0x1000000.")]
    fn test_oob_write_byte() {
        let mut ram = Ram::default();
        ram.write_byte(0x100_0000, 0xAB);
    }

    #[test]
    #[should_panic(expected = "Illegal memory write at address 0x1000000.")]
    fn test_oob_write_word() {
        let mut ram = Ram::default();
        ram.write_word(0xFF_FFFF, 0xAB);
    }

    #[test]
    #[should_panic(
        expected = "Illegal RAM size. Given value is 1234 bytes, expected 16777216 bytes."
    )]
    fn test_bad_ram_size() {
        let _ = Ram::new(vec![0x00; 1234]);
    }

    #[test]
    fn test_ram_construct_read_write() {
        let val_start = 0xFE;
        let addr_start = 0x00_0000;
        let val_word_msb = 0x12;
        let val_word_lsb = 0x34;
        let val_word = combine_u8_be(val_word_msb, val_word_lsb);
        let addr_word_start = 0x12_3456;
        let val_end = 0xAB;
        let addr_end = 0xFF_FFFF;
        let mut ram_contents: Vec<u8> = vec![0x00; RAM_SIZE];
        ram_contents[addr_start] = val_start;
        ram_contents[addr_word_start] = val_word_lsb;
        ram_contents[addr_word_start + 1] = val_word_msb;
        ram_contents[addr_end] = val_end;

        let mut ram = Ram::new(ram_contents);

        assert_eq!(ram.read_byte(addr_start as u32), val_start);
        assert_eq!(ram.read_byte(addr_word_start as u32), val_word_lsb);
        assert_eq!(ram.read_byte((addr_word_start as u32) + 1), val_word_msb);
        assert_eq!(ram.read_byte(addr_end as u32), val_end);

        assert_eq!(
            ram.read_word(addr_start as u32),
            combine_u8_le(val_start, 0x00)
        );
        assert_eq!(
            ram.read_word((addr_end as u32) - 1),
            combine_u8_le(0x00, val_end)
        );
        assert_eq!(ram.read_word(addr_word_start as u32), val_word);

        let write_byte = 0xCD;
        let write_word_msb = 0x56;
        let write_word_lsb = 0x78;
        let write_word = combine_u8_be(0x56, 0x78);

        ram.write_byte(addr_start as u32, write_byte);
        ram.write_word(addr_word_start as u32, write_word);

        assert_eq!(ram.memory[addr_start], write_byte);
        assert_eq!(ram.memory[addr_word_start], write_word_lsb);
        assert_eq!(ram.memory[addr_word_start + 1], write_word_msb);

        assert_eq!(ram.read_byte(addr_start as u32), write_byte);
        assert_eq!(ram.read_word(addr_word_start as u32), write_word);
    }
}
