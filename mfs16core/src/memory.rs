use std::default::Default;

use crate::{
    mmu::{print_warning_message, NOT_READABLE_BYTE},
    Instruction,
};

/// Memory used for direct interfacing with the CPU.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Memory {
    /// The memory contents.
    pub contents: Vec<u8>,
    /// Determines whether the memory is readable or not.
    readable: bool,
    /// Determines whether the memory is writable or not.
    writable: bool,
    /// If true, print debug messages to stderr.
    pub debug: bool,
}
impl Memory {
    /// Create new [Memory]. All bytes are initialised to the given array.
    pub fn new(contents: Vec<u8>, is_readable: bool, is_writable: bool, debug: bool) -> Self {
        Self {
            contents,
            readable: is_readable,
            writable: is_writable,
            debug,
        }
    }

    /// Create new empty [Memory] of the given size, with the given permissions.
    pub fn new_empty(size: usize, is_readable: bool, is_writable: bool) -> Self {
        Self::new(vec![0x00; size], is_readable, is_writable, false)
    }

    /// Check if this [Memory] is readable.
    pub fn is_readable(&self) -> bool {
        self.readable
    }

    /// Check if this [Memory] is writable.
    pub fn is_writable(&self) -> bool {
        self.writable
    }

    /// Change memory read permissions.
    pub fn set_readable(&mut self, value: bool) {
        self.readable = value
    }

    /// Change memory write permissions.
    pub fn set_writable(&mut self, value: bool) {
        self.writable = value
    }

    /// Read a byte from memory at the given address. If not readable, return default value.
    pub fn read_byte(&self, address: u32) -> u8 {
        if !self.readable {
            print_warning_message("read from unreadable memory", address, self.debug);
            return NOT_READABLE_BYTE;
        }

        self.check_addr(address, address, "read");

        self.contents[address as usize]
    }

    /// Write a byte to memory at the given address. Do nothing if not writable.
    pub fn write_byte(&mut self, address: u32, value: u8) {
        self.write_helper(address, address, &value.to_le_bytes());
    }

    /// Read a word from memory starting at the given address. If not readable, return default
    /// value.
    pub fn read_word(&self, address: u32) -> u16 {
        if !self.readable {
            print_warning_message("read from unreadable memory", address, self.debug);
            return ((NOT_READABLE_BYTE as u16) << 8) | (NOT_READABLE_BYTE as u16);
        }

        let end = address + 1;
        self.check_addr(address, end, "read");

        <u16>::from_le_bytes(
            self.contents[(address as usize)..=(end as usize)]
                .try_into()
                .expect("Failed to read word: slice with incorrect length."),
        )
    }

    /// Write a word to memory starting at the given address. Do nothing if not writable.
    pub fn write_word(&mut self, address: u32, value: u16) {
        self.write_helper(address, address + 1, &value.to_le_bytes());
    }

    /// Read a double word from memory starting at the given address. If not readable, return
    /// default value.
    pub fn read_dword(&self, address: u32) -> u32 {
        if !self.readable {
            print_warning_message("read from unreadable memory", address, self.debug);
            return ((NOT_READABLE_BYTE as u32) << 24)
                | ((NOT_READABLE_BYTE as u32) << 12)
                | ((NOT_READABLE_BYTE as u32) << 8)
                | (NOT_READABLE_BYTE as u32);
        }

        let end = address + 3;
        self.check_addr(address, end, "read");

        <u32>::from_le_bytes(
            self.contents[(address as usize)..=(end as usize)]
                .try_into()
                .expect("Failed to read dword: slice with incorrect length."),
        )
    }

    /// Write a double word from memory starting at the given address. Do nothing if not writable.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        self.write_helper(address, address + 3, &value.to_le_bytes());
    }

    fn write_helper(&mut self, address: u32, end: u32, le_bytes: &[u8]) {
        if !self.writable {
            print_warning_message("write to unwritable memory", address, self.debug);
            return;
        }

        self.check_addr(address, end, "write");
        self.contents[(address as usize)..=(end as usize)].copy_from_slice(le_bytes);
    }

    /// Load a slice of bytes directly into memory starting at the given address, overwriting any
    /// existing data in that range. Overriddes `self.writable`.
    pub fn direct_write(&mut self, start: u32, bytes: &[u8]) {
        self.contents[(start as usize)..((start as usize) + bytes.len())].copy_from_slice(bytes);
    }

    fn check_addr(&self, first_address: u32, last_address: u32, verb: &'static str) {
        for address in first_address..=last_address {
            if (address) as usize >= self.contents.len() {
                panic!("Illegal memory {verb} at address {:#X}.", address);
            }
        }
    }
}
impl Default for Memory {
    /// Default: All bytes in memory initialised to 0x00. 0x100_0000 bytes size. Can read and
    /// write. Debug messages off.
    fn default() -> Self {
        Self::new(vec![0x00; 0x100_0000], true, true, false)
    }
}

/// Implementors of this trait can be read from memory.
pub trait MemReadable {
    /// Read this value from memory starting at the given address.
    fn mem_read(mem: &Memory, address: u32) -> Self;
}
/// Implementors of this trait can be written to memory.
pub trait MemWritable {
    /// Write this value to memory starting at the given address, returning the address after the
    /// written value.
    fn mem_write(&self, mem: &mut Memory, address: u32) -> u32;
}
macro_rules! impl_mem {
    ($(($t:ty, $r_fn:ident, $w_fn:ident, $num_bytes:literal)),+) => {
       $(
           impl MemReadable for $t {
               fn mem_read(mem: &Memory, address: u32) -> Self {
                   mem.$r_fn(address)
               }
           }
           impl MemWritable for $t {
               fn mem_write(&self, mem: &mut Memory, address: u32) -> u32 {
                   mem.$w_fn(address, *self);
                   address + $num_bytes
               }
           }
       )*
    };
}
impl_mem!(
    (u8, read_byte, write_byte, 1),
    (u16, read_word, write_word, 2),
    (u32, read_dword, write_dword, 4)
);
impl MemReadable for Instruction {
    fn mem_read(mem: &Memory, address: u32) -> Self {
        Instruction::from_opcode(mem.read_word(address))
    }
}
impl MemWritable for Instruction {
    fn mem_write(&self, mem: &mut Memory, address: u32) -> u32 {
        mem.write_word(address, self.into_opcode());
        address + 2
    }
}

/// Generate [Memory] from a list of values implementing [MemWritable].
#[macro_export]
macro_rules! gen_mem {
    [$($val:expr),*] => {
        {
            #[allow(unused_mut)]
            let mut mem = Memory::new_empty(0x100_0000, true, true);
            let mut _addr: u32 = 0x00_0000;
            $(
                _addr = $val.mem_write(&mut mem, _addr);
            )*
            mem
        }
    };
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::helpers::combine_u8_be;

    use super::*;

    #[test]
    fn test_mem_macro() {
        let mem = gen_mem![];
        assert_eq!(mem.read_byte(0x00_0000), 0x00);

        let mem = gen_mem![0x78_u8, 0x56_u8, 0x34_u8, 0x12_u8];
        assert_eq!(mem.read_byte(0x00_0000), 0x78);
        assert_eq!(mem.read_byte(0x00_0001), 0x56);
        assert_eq!(mem.read_byte(0x00_0002), 0x34);
        assert_eq!(mem.read_byte(0x00_0003), 0x12);
        let mem = gen_mem![0x5678_u16, 0x1234_u16, 0xFEDC_BA98_u32];
        assert_eq!(mem.read_dword(0x00_0000), 0x1234_5678);
        assert_eq!(mem.read_dword(0x00_0004), 0xFEDC_BA98);
    }

    #[test]
    #[should_panic(expected = "Illegal memory read at address 0x1000000.")]
    fn test_oob_read_byte() {
        let mem = Memory::default();
        mem.read_byte(0x100_0000);
    }

    #[test]
    #[should_panic(expected = "Illegal memory read at address 0x1000000.")]
    fn test_oob_read_word() {
        let mem = Memory::default();
        mem.read_word(0xFF_FFFF);
    }

    #[test]
    #[should_panic(expected = "Illegal memory write at address 0x1000000.")]
    fn test_oob_write_byte() {
        let mut mem = Memory::default();
        mem.write_byte(0x100_0000, 0xAB);
    }

    #[test]
    #[should_panic(expected = "Illegal memory write at address 0x1000000.")]
    fn test_oob_write_word() {
        let mut mem = Memory::default();
        mem.write_word(0xFF_FFFF, 0xAB);
    }

    #[test]
    fn test_mem_dword() {
        let mut mem = Memory::default();
        mem.write_dword(0x00_0000, 0x1234_5678);
        assert_eq!(mem.contents[0x00_0000], 0x78);
        assert_eq!(mem.contents[0x00_0001], 0x56);
        assert_eq!(mem.contents[0x00_0002], 0x34);
        assert_eq!(mem.contents[0x00_0003], 0x12);
        assert_eq!(mem.read_word(0x00_0000), 0x5678);
        assert_eq!(mem.read_dword(0x00_0000), 0x1234_5678);
    }

    #[test]
    fn test_mem_construct_read_write() {
        let val_start = 0xFE;
        let addr_start = 0x00_0000;
        let val_word_msb = 0x12;
        let val_word_lsb = 0x34;
        let val_word = combine_u8_be(val_word_msb, val_word_lsb);
        let addr_word_start = 0x12_3456;
        let val_end = 0xAB;
        let addr_end = 0xFF_FFFF;
        let mut mem_contents: Vec<u8> = vec![0x00; 0x100_0000];
        mem_contents[addr_start] = val_start;
        mem_contents[addr_word_start] = val_word_lsb;
        mem_contents[addr_word_start + 1] = val_word_msb;
        mem_contents[addr_end] = val_end;

        let mut mem = Memory::new(mem_contents, true, true, true);

        assert_eq!(mem.read_byte(addr_start as u32), val_start);
        assert_eq!(mem.read_byte(addr_word_start as u32), val_word_lsb);
        assert_eq!(mem.read_byte((addr_word_start as u32) + 1), val_word_msb);
        assert_eq!(mem.read_byte(addr_end as u32), val_end);

        assert_eq!(
            mem.read_word(addr_start as u32),
            <u16>::from_le_bytes([val_start, 0x00])
        );
        assert_eq!(
            mem.read_word((addr_end as u32) - 1),
            <u16>::from_le_bytes([0x00, val_end])
        );
        assert_eq!(mem.read_word(addr_word_start as u32), val_word);

        let write_byte = 0xCD;
        let write_word_msb = 0x56;
        let write_word_lsb = 0x78;
        let write_word = combine_u8_be(0x56, 0x78);

        mem.write_byte(addr_start as u32, write_byte);
        mem.write_word(addr_word_start as u32, write_word);

        assert_eq!(mem.contents[addr_start], write_byte);
        assert_eq!(mem.contents[addr_word_start], write_word_lsb);
        assert_eq!(mem.contents[addr_word_start + 1], write_word_msb);

        assert_eq!(mem.read_byte(addr_start as u32), write_byte);
        assert_eq!(mem.read_word(addr_word_start as u32), write_word);
    }

    #[test]
    fn test_mem_permissions() {
        let mut mem = Memory::new_empty(0x100_0000, false, false);

        assert_eq!(mem.read_byte(0), NOT_READABLE_BYTE);
        mem.set_readable(true);
        assert_eq!(mem.read_byte(0), 0x00);

        mem.write_byte(0, 0xAB);
        assert_eq!(mem.read_byte(0), 0x00);
        mem.set_writable(true);
        mem.write_byte(0, 0xAB);
        assert_eq!(mem.read_byte(0), 0xAB);
    }
}
