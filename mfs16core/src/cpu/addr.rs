use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    convert::Into,
    default::Default,
    fmt::Display,
};

const MAX_TOO_BIG_MSG: &str =
    "The given range exceeds the maximum bounds of a 32-bit unsigned integer.";

/// Address in memory. Restricted to a given size and offset.
#[derive(Debug, Clone, Eq)]
pub struct Addr {
    address: u32,
    start: u32,
    end: u32,
}
impl Addr {
    /// Create a new [Addr], panicking if out of bounds.
    pub fn new(offset: usize, size: usize, value: u32) -> Self {
        Self::check_max(offset, size);

        if value > ((size + offset) as u32) {
            panic!(
                "Illegal address value. Given value `{}` is greater than maximum value `{}`.",
                value,
                (size + offset)
            )
        }

        if value < (offset as u32) {
            panic!(
                "Illegal address value. Given value `{}` is smaller than minimum value `{}`.",
                value, offset
            )
        }

        Self {
            address: value,
            start: offset.try_into().unwrap(),
            end: (offset + size).try_into().unwrap(),
        }
    }

    /// Create a new [Addr] with the default range and offset.
    pub fn new_default_range(value: u32) -> Self {
        Self {
            address: value,
            ..Self::default()
        }
    }

    /// Create a new [Addr], wrapping if out of bounds.
    pub fn new_wrapped(offset: usize, size: usize, value: u32) -> Self {
        Self::check_max(offset, size);
        let wrapped_value = value % ((offset + size) as u32);

        Self {
            address: wrapped_value,
            start: offset.try_into().unwrap(),
            end: (offset + size).try_into().unwrap(),
        }
    }

    fn check_max(offset: usize, size: usize) {
        if (size + offset) > u32::MAX.try_into().unwrap() {
            panic!("{}", MAX_TOO_BIG_MSG);
        }
    }

    /// Get the address of this [Addr].
    pub fn address(&self) -> u32 {
        self.address
    }

    /// Get the start value of this [Addr]'s range.
    pub fn range_start(&self) -> u32 {
        self.start
    }

    /// Get the end value of this [Addr]'s range.
    pub fn range_end(&self) -> u32 {
        self.end
    }

    fn modulo_arg(&self) -> u32 {
        (self.end - self.start).wrapping_add(1)
    }

    /// Get the address of this [Addr] relative to the offset of its range.
    pub fn relative_address(&self) -> u32 {
        self.address - self.start
    }

    /// Add value to this [Addr], wrapping if out of bounds.
    pub fn wrapping_add(&mut self, value: u32) {
        if self.modulo_arg() == 0 {
            self.address = self.relative_address().wrapping_add(value) + self.start;
        } else {
            self.address =
                (self.relative_address().wrapping_add(value) % self.modulo_arg()) + self.start;
        }
    }

    /// Subtract value from this [Addr], wrapping if out of bounds.
    pub fn wrapping_sub(&mut self, value: u32) {
        if self.modulo_arg() == 0 {
            self.address = self.relative_address().wrapping_sub(value) + self.start;
        } else {
            self.address =
                (self.relative_address().wrapping_sub(value) % self.modulo_arg()) + self.start;
        }
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
impl PartialOrd for Addr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Addr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
    }
}
impl PartialEq for Addr {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
impl Default for Addr {
    fn default() -> Self {
        Self::new(0, <u32>::MAX.try_into().unwrap(), 0)
    }
}
#[allow(clippy::from_over_into)]
impl Into<u32> for Addr {
    fn into(self) -> u32 {
        self.address
    }
}
#[allow(clippy::from_over_into)]
impl Into<u32> for &Addr {
    fn into(self) -> u32 {
        self.address
    }
}
impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010X}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{RAM_OFFSET, RAM_SIZE, ROM_OFFSET, ROM_SIZE};

    #[test]
    #[should_panic]
    fn test_addr_too_big() {
        let _ = Addr::new(RAM_OFFSET, RAM_SIZE - 1, (RAM_SIZE + RAM_OFFSET) as u32);
    }

    #[test]
    fn test_addr_wrapping_add() {
        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32 + 0x12_3456);
        // add 1
        addr.wrapping_add(1);
        assert_eq!(addr.address(), RAM_OFFSET as u32 + 0x12_3457);

        let mut addr = Addr::new(
            RAM_OFFSET,
            RAM_SIZE - 1,
            ((RAM_OFFSET + RAM_SIZE) - 1) as u32,
        );
        // add 2 (wrap)
        addr.wrapping_add(2);
        assert_eq!(addr.address(), RAM_OFFSET as u32 + 1);

        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32 + 0x12_3456);
        // add -1
        addr.wrapping_add(0xFFFF_FFFF);
        assert_eq!(addr.address(), RAM_OFFSET as u32 + 0x12_3455);

        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32 + 0x12_3456);
        // add -0x0012_3456
        addr.wrapping_add(0xFFED_CBAA);
        assert_eq!(addr.address(), RAM_OFFSET as u32);

        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32 + 0x12_3456);
        // add -0x0012_3457
        addr.wrapping_add(0xFFED_CBA9);
        assert_eq!(addr.address(), (RAM_OFFSET + RAM_SIZE) as u32 - 1);
    }

    #[test]
    fn test_addr_wrapping_sub() {
        let mut addr = Addr::new(ROM_OFFSET, ROM_SIZE - 1, ROM_OFFSET as u32 + 0x0012_3456);
        // sub 1
        addr.wrapping_sub(1);
        assert_eq!(addr.address(), ROM_OFFSET as u32 + 0x12_3455);

        let mut addr = Addr::new(ROM_OFFSET, ROM_SIZE - 1, ROM_OFFSET as u32 + 0x00_0001);
        // sub 3
        addr.wrapping_sub(3);
        assert_eq!(addr.address(), (ROM_OFFSET + ROM_SIZE) as u32 - 2);

        let mut addr = Addr::new(ROM_OFFSET, ROM_SIZE - 1, ROM_OFFSET as u32 + 0x12_3456);
        // sub -1
        addr.wrapping_sub(0xFFFF_FFFF);
        assert_eq!(addr.address(), ROM_OFFSET as u32 + 0x12_3457);

        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32 + 0x12_3456);
        // sub 0x0012_3456
        addr.wrapping_sub(0x12_3456);
        assert_eq!(addr.address(), RAM_OFFSET as u32);

        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32 + 0x12_3456);
        // sub 0x0012_3457
        addr.wrapping_sub(0x12_3457);
        assert_eq!(addr.address(), (RAM_OFFSET as u32 + RAM_SIZE as u32) - 1);
    }

    #[test]
    fn test_addr_wrapping_inc() {
        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32);
        assert_eq!(addr.address(), RAM_OFFSET as u32);
        addr.wrapping_inc();
        assert_eq!(addr.address(), RAM_OFFSET as u32 + 1);
        let mut addr = Addr::new(
            RAM_OFFSET,
            RAM_SIZE - 1,
            ((RAM_SIZE + RAM_OFFSET) as u32) - 2,
        );
        assert_eq!(addr.address(), ((RAM_SIZE + RAM_OFFSET) as u32) - 2);
        addr.wrapping_inc();
        assert_eq!(addr.address(), ((RAM_SIZE + RAM_OFFSET) as u32) - 1);
        addr.wrapping_inc();
        assert_eq!(addr.address(), RAM_OFFSET as u32);
    }

    #[test]
    fn test_addr_wrapping_dec() {
        let mut addr = Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32);
        assert_eq!(addr.address(), RAM_OFFSET as u32);
        addr.wrapping_dec();
        assert_eq!(addr.address(), ((RAM_SIZE + RAM_OFFSET) as u32) - 1);
        addr.wrapping_dec();
        assert_eq!(addr.address(), ((RAM_SIZE + RAM_OFFSET) as u32) - 2);
    }
}
