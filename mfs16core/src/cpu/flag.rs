//! All functionality related to the CPU flags.
use std::{default::Default, fmt::Display, ops::BitAnd};

use Flag::*;

/// Enum to access the individual CPU flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flag {
    /// The Zero flag.
    /// This flag is set iff the operation result is zero.
    Zero,
    /// The Carry flag.
    /// This flag is set iff an arithmetic carry/borrow occurred during the operation.
    Carry,
    /// The Overflow flag.
    /// This flag is set iff an arithmetic overflow occurred during the operation.
    Overflow,
    /// The Parity flag.
    /// This flag is set iff the operation result is even.
    Parity,
    /// The Negative flag.
    /// This flag is set iff the operation result is negative.
    Negative,
}
impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The CPU flags.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Flags {
    zero: bool,
    carry: bool,
    overflow: bool,
    parity: bool,
    negative: bool,
}
impl Flags {
    /// Create new [Flags] set to the given values.
    pub fn new(zero: bool, carry: bool, overflow: bool, parity: bool, negative: bool) -> Self {
        Self {
            zero,
            carry,
            overflow,
            parity,
            negative,
        }
    }

    /// Create new [Flags] based on a string.
    /// Make the flag's letter a capital to set the flag. Make the flag's letter lowercase, or
    /// absent entirely, to reset the flag.
    pub fn from_string(s: &str) -> Self {
        let mut result = Self::default();
        if s.find('Z').is_some() {
            result.zero = true;
        }
        if s.find('C').is_some() {
            result.carry = true;
        }
        if s.find('O').is_some() {
            result.overflow = true;
        }
        if s.find('P').is_some() {
            result.parity = true;
        }
        if s.find('N').is_some() {
            result.negative = true;
        }
        result
    }

    /// Reset all flags.
    pub fn reset_all(&mut self) {
        *self = Self::from_string("");
    }

    /// Get the value of the given [Flag].
    pub fn flag(&self, flag: Flag) -> bool {
        match flag {
            Zero => self.zero,
            Carry => self.carry,
            Overflow => self.overflow,
            Parity => self.parity,
            Negative => self.negative,
        }
    }

    /// Set the given [Flag].
    pub fn set_flag(&mut self, flag: Flag) {
        self.change_flag(flag, true);
    }

    /// Reset the given [Flag].
    pub fn reset_flag(&mut self, flag: Flag) {
        self.change_flag(flag, false);
    }

    /// Change the given [Flag] to the given boolean value.
    pub fn change_flag(&mut self, flag: Flag, val: bool) {
        match flag {
            Zero => self.zero = val,
            Carry => self.carry = val,
            Overflow => self.overflow = val,
            Parity => self.parity = val,
            Negative => self.negative = val,
        }
    }

    /// Change the [Zero] flag according to the given value.
    /// Return `true` iff the value == 0.
    pub fn change_zero<T>(&mut self, val: T)
    where
        T: Zeroable + PartialEq + Eq,
    {
        self.change_flag(Zero, val == T::zero())
    }

    /// Change the [Parity] flag according to the given value.
    /// [Parity] is set iff the value is even.
    pub fn change_parity<T>(&mut self, val: T)
    where
        T: BitAnd<Output = T> + Oneable + Zeroable + PartialEq,
    {
        self.change_flag(Parity, (val & T::one()) == T::zero())
    }

    /// Change the [Negative] flag according to the given value.
    /// [Negative] is set iff the value is negative when read as signed.
    pub fn change_negative<T>(&mut self, val: T)
    where
        T: Msb,
    {
        self.change_flag(Negative, val.msb())
    }
}
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            pf('z', self.zero),
            pf('c', self.carry),
            pf('o', self.overflow),
            pf('p', self.parity),
            pf('n', self.negative)
        )
    }
}

fn pf(c: char, val: bool) -> String {
    if val {
        c.to_uppercase().to_string()
    } else {
        c.to_lowercase().to_string()
    }
}

/// Trait to get the most significant bit of a data type.
pub trait Msb {
    /// Get the most significant bit of this value.
    fn msb(&self) -> bool;
}
macro_rules! impl_msb {
    ($($t:ty),+) => {
        $(impl Msb for $t {
            fn msb(&self) -> bool {
                (self >> (<$t>::BITS - 1)) == 1
            }
        })*
    }
}
impl_msb!(u8, u16, u32, u64, u128);

/// Trait to get 0 in the given data type.
pub trait Zeroable {
    /// Get the value 0 in this data type.
    fn zero() -> Self;
}
macro_rules! impl_zero {
    ($($t:ty),+) => {
        $(impl Zeroable for $t {
            fn zero() -> Self {
                0
            }
        })*
    }
}
impl_zero!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

/// Trait to get 1 in the given data type.
pub trait Oneable {
    /// Get the value 1 in this data type.
    fn one() -> Self;
}
macro_rules! impl_one {
    ($($t:ty),+) => {
        $(impl Oneable for $t {
            fn one() -> Self {
                1
            }
        })*
    }
}
impl_one!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msb() {
        let u8_1: u8 = 0b1010_0010;
        let u8_0: u8 = 0b0101_1111;
        let u16_1: u16 = 0b1011_1010_0101_1010;
        let u16_0: u16 = 0b0010_1010_0101_0000;

        assert!(u8_1.msb());
        assert!(!u8_0.msb());
        assert!(u16_1.msb());
        assert!(!u16_0.msb());
    }

    #[test]
    fn test_change_zero() {
        let mut flags = Flags::default();
        assert!(!flags.flag(Zero));

        flags.change_zero(0);
        assert!(flags.flag(Zero));

        flags.change_zero(1);
        assert!(!flags.flag(Zero));

        flags.change_zero(-1);
        assert!(!flags.flag(Zero));
    }

    #[test]
    fn test_change_parity() {
        let mut flags = Flags::default();
        assert!(!flags.flag(Parity));

        flags.change_parity(0);
        assert!(flags.flag(Parity));

        flags.change_parity(1);
        assert!(!flags.flag(Parity));

        flags.change_parity(0b1111_1010_u8);
        assert!(flags.flag(Parity));

        flags.change_parity(0b1111_1010_u8 as i8);
        assert!(flags.flag(Parity));

        flags.change_parity(-1);
        assert!(!flags.flag(Parity));

        flags.change_parity(-2);
        assert!(flags.flag(Parity));
    }

    #[test]
    fn test_change_negative() {
        let mut flags = Flags::default();
        assert!(!flags.flag(Negative));

        flags.change_negative(0b1111_1111_u8);
        assert!(flags.flag(Negative));

        flags.change_negative(1_u8);
        assert!(!flags.flag(Negative));

        flags.change_negative(0_u8);
        assert!(!flags.flag(Negative));
    }

    #[test]
    fn test_from_string() {
        let fs = Flags::from_string("zCoPn");
        assert_eq!(
            (fs.zero, fs.carry, fs.overflow, fs.parity, fs.negative),
            (false, true, false, true, false)
        );

        let fs = Flags::from_string("ZcOpN");
        assert_eq!(
            (fs.zero, fs.carry, fs.overflow, fs.parity, fs.negative),
            (true, false, true, false, true)
        );

        let fs = Flags::from_string("c");
        assert_eq!(
            (fs.zero, fs.carry, fs.overflow, fs.parity, fs.negative),
            (false, false, false, false, false)
        );

        let fs = Flags::from_string("mhm");
        assert_eq!(
            (fs.zero, fs.carry, fs.overflow, fs.parity, fs.negative),
            (false, false, false, false, false)
        );
    }
}
