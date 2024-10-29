use std::{convert::Into, fmt::Display};

use crate::RAM_SIZE;

/// Address on RAM. Restricted to [RAM_SIZE].
#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub struct Addr(u32);
impl Addr {
    /// Create a new [Addr], panicking if out of bounds.
    pub fn new(value: u32) -> Self {
        if (value as usize) >= RAM_SIZE {
            panic!(
                "Illegal program counter value. Given value {} is greater than maximum value {}.",
                value,
                RAM_SIZE - 1
            );
        }

        Self(value)
    }

    /// Create a new [Addr], wrapping if out of bounds.
    pub fn new_wrapped(value: u32) -> Self {
        let wrapped_value = value % (RAM_SIZE as u32);
        Self(wrapped_value)
    }

    /// Add value to this [Addr], wrapping if out of bounds.
    pub fn wrapping_add(&mut self, value: u32) {
        self.0 = (self.0.wrapping_add(value)) % (RAM_SIZE as u32);
    }

    /// Subtract value from this [Addr], wrapping if out of bounds.
    pub fn wrapping_sub(&mut self, value: u32) {
        self.0 = (self.0.wrapping_sub(value)) % (RAM_SIZE as u32);
    }

    /// Increment this [Addr], wrapping on overflow.
    pub fn wrapping_inc(&mut self) {
        self.wrapping_add(1);
    }

    /// Decrement this [Addr], wrapping on underflow.
    pub fn wrapping_dec(&mut self) {
        self.wrapping_sub(1);
    }
}
#[allow(clippy::from_over_into)]
impl Into<u32> for Addr {
    fn into(self) -> u32 {
        self.0
    }
}
#[allow(clippy::from_over_into)]
impl Into<u32> for &Addr {
    fn into(self) -> u32 {
        self.0
    }
}
impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#08X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    #[should_panic]
    fn test_addr_too_big() {
        let _ = Addr::new(RAM_SIZE as u32);
    }

    #[test]
    fn test_addr_wrapping_add() {
        let mut addr = Addr::new(0x0012_3456);
        // add 1
        addr.wrapping_add(1);
        assert_eq!(addr, Addr::new(0x12_3457));

        let mut addr = Addr::new(0x00FF_FFFF);
        // add 2 (wrap)
        addr.wrapping_add(2);
        assert_eq!(addr, Addr::new(0x00_0001));

        let mut addr = Addr::new(0x12_3456);
        // add -1
        addr.wrapping_add(0xFFFF_FFFF);
        assert_eq!(addr, Addr::new(0x12_3455));

        let mut addr = Addr::new(0x12_3456);
        // add -0x0012_3456
        addr.wrapping_add(0xFFED_CBAA);
        assert_eq!(addr, Addr::new(0x0));

        let mut addr = Addr::new(0x12_3456);
        // add -0x0012_3457
        addr.wrapping_add(0xFFED_CBA9);
        assert_eq!(addr, Addr::new(0xFF_FFFF));
    }

    #[test]
    fn test_addr_wrapping_sub() {
        let mut addr = Addr::new(0x0012_3456);
        // sub 1
        addr.wrapping_sub(1);
        assert_eq!(addr, Addr::new(0x12_3455));

        let mut addr = Addr::new(0x00_0001);
        // sub 3
        addr.wrapping_sub(3);
        assert_eq!(addr, Addr::new(0xFF_FFFE));

        let mut addr = Addr::new(0x12_3456);
        // sub -1
        addr.wrapping_sub(0xFFFF_FFFF);
        assert_eq!(addr, Addr::new(0x12_3457));

        let mut addr = Addr::new(0x12_3456);
        // sub 0x0012_3456
        addr.wrapping_sub(0x12_3456);
        assert_eq!(addr, Addr::new(0x0));

        let mut addr = Addr::new(0x12_3456);
        // sub 0x0012_3457
        addr.wrapping_sub(0x12_3457);
        assert_eq!(addr, Addr::new(0xFF_FFFF));
    }

    #[test]
    fn test_addr_wrapping_inc() {
        let mut addr = Addr::default();
        assert_eq!(addr.0, 0);
        addr.wrapping_inc();
        assert_eq!(addr.0, 1);
        let mut addr = Addr::new((RAM_SIZE as u32) - 2);
        assert_eq!(addr.0, (RAM_SIZE as u32) - 2);
        addr.wrapping_inc();
        assert_eq!(addr.0, (RAM_SIZE as u32) - 1);
        addr.wrapping_inc();
        assert_eq!(addr.0, 0);
    }

    #[test]
    fn test_addr_wrapping_dec() {
        let mut addr = Addr::default();
        assert_eq!(addr.0, 0);
        addr.wrapping_dec();
        assert_eq!(addr.0, (RAM_SIZE as u32) - 1);
        addr.wrapping_dec();
        assert_eq!(addr.0, (RAM_SIZE as u32) - 2);
    }
}
