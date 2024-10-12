//! General helper functions used by a wide variety of code.

/// Split a 32-bit double word into two words.
pub fn split_dword(val: u32) -> (u16, u16) {
    (((val >> 16) as u16), ((val & 0x0000_FFFF) as u16))
}

/// Split a 16-bit word into two bytes.
pub fn split_word(val: u16) -> (u8, u8) {
    (((val >> 8) as u8), ((val & 0x00FF) as u8))
}

/// Combine two words into a single, big-endian 32-bit value.
pub fn combine_u16_be(h: u16, l: u16) -> u32 {
    combine_u16_helper(h, l)
}

/// Combine two words into a single, little-endian 32-bit value.
pub fn combine_u16_le(h: u16, l: u16) -> u32 {
    let (byte_3, byte_2) = split_word(h);
    let (byte_1, byte_0) = split_word(l);
    let word_1 = combine_u8_le(byte_3, byte_2);
    let word_0 = combine_u8_le(byte_1, byte_0);
    combine_u16_helper(word_0, word_1)
}

/// Combine two bytes into a single, big-endian 16-bit value.
pub fn combine_u8_be(h: u8, l: u8) -> u16 {
    combine_u8_helper(h, l)
}

/// Combine two bytes into a single, little-endian 16-bit value.
pub fn combine_u8_le(h: u8, l: u8) -> u16 {
    combine_u8_helper(l, h)
}
// ------- HELPER FUNCTIONS -------
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
        let high_word = 0x1234;
        let low_word = 0x5678;
        let combined_u32_be = combine_u16_be(high_word, low_word);
        let combined_u32_le = combine_u16_le(high_word, low_word);
        assert_eq!(combined_u32_be, 0x1234_5678);
        assert_eq!(combined_u32_le, 0x7856_3412);
    }
}
