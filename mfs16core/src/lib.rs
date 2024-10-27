//! Core library for backend mfs16 functionality.
#![warn(missing_docs)]

mod computer;
mod cpu;
mod helpers;
mod ram;

// Re-exports
pub use computer::{Computer, CLOCK_FREQ, RAM_SIZE};
pub use cpu::{
    AsLargerType, Flag, Flags, HasMax, Instruction, Msb, NMinus1Mask, NumBits, Oneable, Pc, Reg,
    Reg16, Reg32, Reg8, WrappingAdd, WrappingSub, Zeroable,
};
pub use helpers::*;
pub use ram::{Ram, RamReadable, RamWritable};
