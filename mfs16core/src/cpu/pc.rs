use std::{convert::Into, fmt::Display};

use crate::RAM_SIZE;

/// Program counter. Restricted to [RAM_SIZE].
#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub struct Pc(u32);
impl Pc {
    /// Create a new [Pc], panicking if out of bounds.
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

    /// Increment this [Pc], wrapping on overflow.
    pub fn wrapping_inc(&mut self) {
        self.0 = (self.0 + 1) % (RAM_SIZE as u32);
    }

    /// Decrement this [Pc], wrapping on underflow.
    pub fn wrapping_dec(&mut self) {
        self.0 = (self.0.wrapping_sub(1)) % (RAM_SIZE as u32);
    }
}
#[allow(clippy::from_over_into)]
impl Into<u32> for Pc {
    fn into(self) -> u32 {
        self.0
    }
}
#[allow(clippy::from_over_into)]
impl Into<u32> for &Pc {
    fn into(self) -> u32 {
        self.0
    }
}
impl Display for Pc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010X}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    #[should_panic]
    fn test_pc_too_big() {
        let _ = Pc::new(RAM_SIZE as u32);
    }

    #[test]
    fn test_pc_wrapping_inc() {
        let mut pc = Pc::default();
        assert_eq!(pc.0, 0);
        pc.wrapping_inc();
        assert_eq!(pc.0, 1);
        let mut pc = Pc::new((RAM_SIZE as u32) - 2);
        assert_eq!(pc.0, (RAM_SIZE as u32) - 2);
        pc.wrapping_inc();
        assert_eq!(pc.0, (RAM_SIZE as u32) - 1);
        pc.wrapping_inc();
        assert_eq!(pc.0, 0);
    }

    #[test]
    fn test_pc_wrapping_dec() {
        let mut pc = Pc::default();
        assert_eq!(pc.0, 0);
        pc.wrapping_dec();
        assert_eq!(pc.0, (RAM_SIZE as u32) - 1);
        pc.wrapping_dec();
        assert_eq!(pc.0, (RAM_SIZE as u32) - 2);
    }
}
