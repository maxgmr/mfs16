//! All functionality related to the CPU flags.
use std::{default::Default, fmt::Display};

use Flag::*;

/// Enum to access the individual CPU flags.
#[derive(Debug, Copy, Clone)]
pub enum Flag {
    /// The Zero flag.
    /// This flag is set iff the operation result is zero.
    Zero,
    /// The Carry flag.
    /// This flag is set iff an arithmetic carry occurred during the operation.
    Carry,
    /// The Overflow flag.
    /// This flag is set iff an arithmetic overflow/underflow occurred during the operation.
    Overflow,
    /// The Parity flag.
    /// This flag is set iff the number of set bits in the operation result is even.
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

    /// Set the given [Flag].
    pub fn set(&mut self, flag: Flag) {
        self.change(flag, true);
    }

    /// Reset the given [Flag].
    pub fn reset(&mut self, flag: Flag) {
        self.change(flag, false);
    }

    /// Change the given [Flag] to the given boolean value.
    pub fn change(&mut self, flag: Flag, val: bool) {
        match flag {
            Zero => self.zero = val,
            Carry => self.carry = val,
            Overflow => self.overflow = val,
            Parity => self.parity = val,
            Negative => self.negative = val,
        }
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
