use std::default::Default;

use crate::RAM_SIZE;

/// Random-access memory for direct interfacing with the CPU.
pub struct Ram {
    /// The memory contents of the RAM.
    memory: [u8; RAM_SIZE],
}
impl Ram {
    /// Create new [Ram] of the set size. All bytes are initialised to the given array.
    pub fn new(memory: [u8; RAM_SIZE]) -> Self {
        Self { memory }
    }
}
impl Default for Ram {
    /// Default: All bytes in memory initialised to 0x00.
    fn default() -> Self {
        Self::new([0x00; RAM_SIZE])
    }
}
