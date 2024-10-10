//! General helper functions used by a wide variety of code.

/// Split a 16-bit word into two bytes.
pub fn split_word(val: u16) -> (u8, u8) {
    (((val >> 8) as u8), ((val & 0x00FF) as u8))
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
    fn test_combine_u8() {
        let high_byte = 0x12;
        let low_byte = 0x34;
        let combined_u16_be = combine_u8_be(high_byte, low_byte);
        let combined_u16_le = combine_u8_le(high_byte, low_byte);
        assert_eq!(combined_u16_be, 0x1234);
        assert_eq!(combined_u16_le, 0x3412);
    }
}
