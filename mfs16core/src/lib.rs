//! Core library for backend mfs16 functionality.
#![warn(missing_docs)]

mod computer;
mod cpu;
mod gpu;
pub mod helpers;
mod ram;

// Re-exports
pub use computer::{Computer, CLOCK_FREQ, DISPLAY_HEIGHT, DISPLAY_WIDTH, RAM_SIZE};
pub use cpu::{
    Addr, AsLargerType, Flag, Flags, HasMax, Instruction, Msb, NMinus1Mask, NumBits, Oneable, Reg,
    Reg16, Reg32, Reg8, WrappingAdd, WrappingSub, Zeroable,
};
pub use ram::{Ram, RamReadable, RamWritable};
