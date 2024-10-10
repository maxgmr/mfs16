//! All functionality related to the CPU flags.
use std::{default::Default, fmt::Display};

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
    /// TODO better docs
    carry: bool,
}
impl Flags {
    /// Create new [Flags] set to the given values.
    pub fn new(carry: bool) -> Self {
        Self { carry }
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
