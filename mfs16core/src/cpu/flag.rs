//! All functionality related to the CPU flags.
use std::{default::Default, fmt::Display};

use Flag::*;

/// Enum to access the individual CPU flags.
#[derive(Debug, Copy, Clone)]
pub enum Flag {
    /// The Carry flag.
    /// TODO better docs
    Carry,
}

/// The CPU flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flags {
    /// The Carry flag.
    /// This flag is set when an arithmetic carry or borrow occurred during the arithmetic
    /// operation.
    carry: bool,
}
impl Flags {
    /// Create new [Flags] set to the given values.
    pub fn new(carry: bool) -> Self {
        Self { carry }
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
            Carry => self.carry = val,
        }
    }
}
impl Default for Flags {
    /// Default: All flags initialised to false.
    fn default() -> Self {
        Self::new(false)
    }
}
impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C: {}", self.carry)
    }
}
