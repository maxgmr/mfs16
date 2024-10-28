//! General helper functions used by a wide variety of code.
use std::ops::{BitAnd, Not, Shl, Shr, Sub};

use crate::{NumBits, Oneable, Zeroable};

/// Get the n least significant bits of a given value.
pub fn n_least_significant<T>(val: T, n: T) -> T
where
    T: NumBits
        + PartialOrd
        + PartialEq
        + Zeroable
        + Oneable
        + Copy
        + BitAnd<Output = T>
        + Shl<Output = T>
        + Sub<Output = T>,
{
    let clamped_n: T = clamp_significant_n(n);

    if clamped_n == <T>::zero() {
        return <T>::zero();
    }
    if clamped_n == <T>::num_bits() {
        return val;
    }

    let bit_mask: T = (<T>::one() << clamped_n) - <T>::one();
    val & bit_mask
}

/// Get the n most significant bits of a given value.
pub fn n_most_significant<T>(val: T, n: T) -> T
where
    T: NumBits
        + PartialOrd
        + PartialEq
        + Zeroable
        + Oneable
        + Copy
        + Not<Output = T>
        + BitAnd<Output = T>
        + Shl<Output = T>
        + Shr<Output = T>
        + Sub<Output = T>,
{
    let clamped_n: T = clamp_significant_n(n);

    if clamped_n == <T>::zero() {
        return <T>::zero();
    }
    if clamped_n == <T>::num_bits() {
        return val;
    }

    let bit_mask: T = !((<T>::one() << (<T>::num_bits() - clamped_n)) - <T>::one());
    (val & bit_mask) >> (<T>::num_bits() - clamped_n)
}

fn clamp_significant_n<T>(n: T) -> T
where
    T: NumBits + PartialOrd + PartialEq + Zeroable,
{
    if n == <T>::zero() {
        return <T>::zero();
    }

    if n > <T>::num_bits() {
        return <T>::num_bits();
    }

    n
}

/// Get the value of a given bit index.
pub fn get_bit<T>(val: T, index: T) -> T
where
    T: Oneable + Zeroable + Shl<Output = T> + NumBits + PartialEq + PartialOrd + BitAnd<Output = T>,
{
    if test_bit(val, index) {
        <T>::one()
    } else {
        <T>::zero()
    }
}

/// Return the truthy/falsy value of a given bit index.
pub fn test_bit<T>(val: T, index: T) -> bool
where
    T: Oneable + Zeroable + Shl<Output = T> + NumBits + PartialEq + PartialOrd + BitAnd<Output = T>,
{
    if index >= <T>::num_bits() {
        return false;
    }

    (val & (<T>::one() << index)) != <T>::zero()
}

/// Split a 64-bit quad word into two double words.
pub fn split_qword(val: u64) -> (u32, u32) {
    (((val >> 32) as u32), ((val & 0x0000_0000_FFFF_FFFF) as u32))
}

/// Split a 32-bit double word into two words.
pub fn split_dword(val: u32) -> (u16, u16) {
    (((val >> 16) as u16), ((val & 0x0000_FFFF) as u16))
}

/// Split a 16-bit word into two bytes.
pub fn split_word(val: u16) -> (u8, u8) {
    (((val >> 8) as u8), ((val & 0x00FF) as u8))
}

/// Combine two big-endian words into a single, big-endian 32-bit value.
pub fn combine_u16_be(h: u16, l: u16) -> u32 {
    combine_u16_helper(h, l)
}

/// Combine two little-endian words into a single, little-endian 32-bit value.
pub fn combine_u16_le(h: u16, l: u16) -> u32 {
    combine_u16_helper(l, h)
}

/// Combine two bytes into a single, big-endian 16-bit value.
pub fn combine_u8_be(h: u8, l: u8) -> u16 {
    combine_u8_helper(h, l)
}

/// Combine two bytes into a single, little-endian 16-bit value.
pub fn combine_u8_le(h: u8, l: u8) -> u16 {
    combine_u8_helper(l, h)
}

fn combine_u8_helper(left_byte: u8, right_byte: u8) -> u16 {
    ((left_byte as u16) << 8) | (right_byte as u16)
}

fn combine_u16_helper(left_word: u16, right_word: u16) -> u32 {
    ((left_word as u32) << 16) | (right_word as u32)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_split_word() {
        let original_val = 0x1234;
        let (hb, lb) = split_word(original_val);
        assert_eq!(hb, 0x12);
        assert_eq!(lb, 0x34);
    }

    #[test]
    fn test_split_dword() {
        let original_val = 0x1234_5678;
        let (hw, lw) = split_dword(original_val);
        assert_eq!(hw, 0x1234);
        assert_eq!(lw, 0x5678);
    }

    #[test]
    fn test_combine_u8() {
        let high_byte = 0x12;
        let low_byte = 0x34;
        let combined_u16_be = combine_u8_be(high_byte, low_byte);
        let combined_u16_le = combine_u8_le(high_byte, low_byte);
        assert_eq!(combined_u16_be, 0x1234);
        assert_eq!(combined_u16_le, 0x3412);
    }

    #[test]
    fn test_combine_u16() {
        let lsb = 0x78;
        let slsb = 0x56;
        let smsb = 0x34;
        let msb = 0x12;

        let combined_u16_msw_le = combine_u8_le(msb, smsb);
        let combined_u16_lsw_le = combine_u8_le(slsb, lsb);
        let combined_u32_le = combine_u16_le(combined_u16_msw_le, combined_u16_lsw_le);
        assert_eq!(combined_u32_le, 0x7856_3412);

        let combined_u16_msw_be = combine_u8_be(msb, smsb);
        let combined_u16_lsw_be = combine_u8_be(slsb, lsb);
        let combined_u32_be = combine_u16_be(combined_u16_msw_be, combined_u16_lsw_be);
        assert_eq!(combined_u32_be, 0x1234_5678);
    }

    #[test]
    fn test_test_bit() {
        assert!(test_bit(0b1010_1010_u8, 1));
        assert!(!test_bit(0b1010_1010_u8, 8));
        assert!(test_bit(0b1010_1010_u8, 7));
        assert!(test_bit(0b1010_1010_1010_1010_u16, 15));
        assert!(!test_bit(0b1010_1010_1010_1010_u16, 0));
    }

    #[test]
    fn test_n_significant() {
        assert_eq!(n_most_significant(0b1010_1010_u8, 255_u8), 0b1010_1010);
        assert_eq!(n_most_significant(0b1010_1010_u8, 0_u8), 0);
        assert_eq!(n_most_significant(0b1010_1010_u8, 1_u8), 1);
        assert_eq!(n_most_significant(0b1010_1010_u8, 2_u8), 0b10);

        assert_eq!(n_least_significant(0b1010_1010_u8, 255_u8), 0b1010_1010);
        assert_eq!(n_least_significant(0b1010_1010_u8, 0_u8), 0);
        assert_eq!(n_least_significant(0b1010_1010_u8, 1_u8), 0);
        assert_eq!(n_least_significant(0b1010_1010_u8, 2_u8), 0b10);
    }
}
