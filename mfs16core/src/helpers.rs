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
