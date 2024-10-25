use std::default::Default;

use crate::{
    helpers::{combine_u16_le, combine_u8_le, split_dword, split_word},
    RAM_SIZE,
};

/// Random-access memory for direct interfacing with the CPU.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Ram {
    /// The memory contents of the RAM.
    pub memory: Vec<u8>,
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

    /// Read a double word from memory starting at the given address.
    pub fn read_dword(&self, address: u32) -> u32 {
        combine_u16_le(
            combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
            combine_u8_le(self.read_byte(address + 2), self.read_byte(address + 3)),
        )
    }

    /// Write a double word from memory starting at the given address.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        let (high_word, low_word) = split_dword(value);
        self.write_word(address, low_word);
        self.write_word(address + 2, high_word);
    }
}
impl Default for Ram {
    /// Default: All bytes in memory initialised to 0x00.
    fn default() -> Self {
        Self::new(vec![0x00; RAM_SIZE])
    }
}

/// Implementors of this trait can be read from RAM.
pub trait RamReadable {
    /// Read this value from RAM starting at the given address.
    fn ram_read(ram: &Ram, address: u32) -> Self;
}
/// Implementors of this trait can be written to RAM.
pub trait RamWritable {
    /// Write this value to RAM starting at the given address, returning the address after the
    /// written value.
    fn ram_write(&self, ram: &mut Ram, address: u32) -> u32;
}
macro_rules! impl_ram {
    ($(($t:ty, $r_fn:ident, $w_fn:ident, $num_bytes:literal)),+) => {
       $(
           impl RamReadable for $t {
               fn ram_read(ram: &Ram, address: u32) -> Self {
                   ram.$r_fn(address)
               }
           }
           impl RamWritable for $t {
               fn ram_write(&self, ram: &mut Ram, address: u32) -> u32 {
                   ram.$w_fn(address, *self);
                   address + $num_bytes
               }
           }
       )*
    };
}
impl_ram!(
    (u8, read_byte, write_byte, 1),
    (u16, read_word, write_word, 2),
    (u32, read_dword, write_dword, 4)
);

/// Generate a [Ram] from a list of values implementing [RamWritable].
#[macro_export]
macro_rules! gen_ram {
    [$($val:expr),*] => {
        {
            #[allow(unused_mut)]
            let mut ram = Ram::default();
            let mut _addr: u32 = 0x00_0000;
            $(
                _addr = $val.ram_write(&mut ram, _addr);
            )*
            ram
        }
    };
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
    fn test_ram_macro() {
        let ram = gen_ram![];
        assert_eq!(ram.read_byte(0x00_0000), 0x00);

        let ram = gen_ram![0x78_u8, 0x56_u8, 0x34_u8, 0x12_u8];
        assert_eq!(ram.read_byte(0x00_0000), 0x78);
        assert_eq!(ram.read_byte(0x00_0001), 0x56);
        assert_eq!(ram.read_byte(0x00_0002), 0x34);
        assert_eq!(ram.read_byte(0x00_0003), 0x12);
        let ram = gen_ram![0x5678_u16, 0x1234_u16, 0xFEDC_BA98_u32];
        assert_eq!(ram.read_dword(0x00_0000), 0x1234_5678);
        assert_eq!(ram.read_dword(0x00_0004), 0xFEDC_BA98);
    }

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
    fn test_ram_dword() {
        let mut ram = Ram::default();
        ram.write_dword(0x00_0000, 0x1234_5678);
        assert_eq!(ram.memory[0x00_0000], 0x78);
        assert_eq!(ram.memory[0x00_0001], 0x56);
        assert_eq!(ram.memory[0x00_0002], 0x34);
        assert_eq!(ram.memory[0x00_0003], 0x12);
        assert_eq!(ram.read_word(0x00_0000), 0x5678);
        assert_eq!(ram.read_dword(0x00_0000), 0x1234_5678);
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
