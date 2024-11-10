use std::default::Default;

use crate::{
    helpers::{combine_u16_le, combine_u8_le, split_dword, split_word},
    mmu::NOT_READABLE_BYTE,
};

mod kb_code;

use super::mmu::print_warning_message;

/// Size of the keyboard register.
pub const KB_REG_SIZE: usize = 0x0000_0040;

/// The register storing the state of the keyboard keys. Each byte corresponds to a key.
/// 1 = pressed, 0 = not pressed.
#[derive(Debug, Clone, PartialEq)]
pub struct KbReg {
    /// The raw byte contents of the keyboard register.
    bytes: [u8; KB_REG_SIZE],
    /// If true, will print warning messages to stderr.
    pub debug: bool,
}
impl KbReg {
    /// Create a new [KbReg].
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            ..Default::default()
        }
    }

    /// Read a raw byte from the KbReg bytes.
    pub fn read_byte(&self, address: u32) -> u8 {
        match address {
            i if (i as usize) < KB_REG_SIZE => self.bytes[i as usize],
            _ => {
                print_warning_message("read from keyboard register", address, self.debug);
                NOT_READABLE_BYTE
            }
        }
    }

    /// Write a raw byte to the KbReg.
    pub fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            i if (i as usize) < KB_REG_SIZE => self.bytes[i as usize] = value,
            _ => print_warning_message("read from keyboard register", address, self.debug),
        }
    }

    /// Write a double word from the KbReg starting at the given address.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        let (high_word, low_word) = split_dword(value);
        self.write_word(address, low_word);
        self.write_word(address + 2, high_word);
    }

    /// Read a double word from the KbReg starting at the given address.
    pub fn read_dword(&self, address: u32) -> u32 {
        combine_u16_le(
            combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
            combine_u8_le(self.read_byte(address + 2), self.read_byte(address + 3)),
        )
    }

    /// Write a word to the KbReg starting at the given address.
    pub fn write_word(&mut self, address: u32, word: u16) {
        let (high_byte, low_byte) = split_word(word);
        self.write_byte(address, low_byte);
        self.write_byte(address + 1, high_byte);
    }

    /// Read a word from the KbReg starting at the given address.
    pub fn read_word(&self, address: u32) -> u16 {
        combine_u8_le(self.read_byte(address), self.read_byte(address + 1))
    }

    /// Get the status of the bit corresponding to the given [KbCode] or index.
    pub fn key<C: Into<u16> + Copy>(&self, code: C) -> bool {
        let (byte_index, bit_index) = Self::byte_and_bit_indicies(code);
        (self.bytes[byte_index as usize] & (1 << bit_index)) != 0
    }

    /// Set the register bit corresponding to the given [KbCode] or index.
    pub fn key_down<C: Into<u16> + Copy>(&mut self, code: C) {
        self.change_bit(code, true);
    }

    /// Reset the register bit corresponding to the given [KbCode] or index.
    pub fn key_up<C: Into<u16> + Copy>(&mut self, code: C) {
        self.change_bit(code, false);
    }

    fn change_bit<C: Into<u16> + Copy>(&mut self, code: C, value: bool) {
        let (byte_index, bit_index) = Self::byte_and_bit_indicies(code);

        if value {
            self.bytes[byte_index as usize] |= 1 << bit_index;
        } else {
            self.bytes[byte_index as usize] &= !(1 << bit_index);
        }
    }

    /// Get the indicies of the byte and bit corresponding to the given index.
    fn byte_and_bit_indicies<C: Into<u16> + Copy>(index: C) -> (u8, u8) {
        let quotient = index.into() / (<u8>::BITS as u16);
        let remainder = index.into() % (<u8>::BITS as u16);
        (quotient as u8, remainder as u8)
    }
}
impl Default for KbReg {
    fn default() -> Self {
        Self {
            bytes: [0; KB_REG_SIZE],
            debug: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    use super::kb_code::KbCode::*;

    #[test]
    fn test_kb_code() {
        assert_eq!(A as u16, 4);
        assert_eq!(Num as u16, 512);
    }

    #[test]
    fn test_get_set() {
        let mut kbr = KbReg::new(true);

        let mut i: u16 = 0;
        for byte in 0..KB_REG_SIZE {
            for bit in 0..<u8>::BITS {
                let cmp_val: u16 = (1 << (bit + 1)) - 1;

                assert_eq!(kbr.key(i), false);
                kbr.key_down(i);
                assert_eq!(kbr.key(i), true);
                kbr.key_up(i);
                assert_eq!(kbr.key(i), false);
                kbr.key_down(i);
                assert_eq!(cmp_val, kbr.bytes[byte] as u16);

                i += 1;
            }
        }
    }

    #[test]
    fn test_press_codes() {
        let mut kbr = KbReg::new(true);

        assert!(!kbr.key(A));

        kbr.key_down(A);
        assert!(kbr.key(A));

        kbr.key_down(A);
        assert!(kbr.key(A));

        kbr.key_down(Z);
        assert!(kbr.key(A));
        assert!(kbr.key(Z));

        kbr.key_up(A);
        assert!(!kbr.key(A));
    }
}
