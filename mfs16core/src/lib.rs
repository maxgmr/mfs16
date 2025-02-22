//! Core library for backend mfs16 functionality.
#![warn(missing_docs)]

mod computer;
mod cpu;
mod drive;
mod gpu;
pub mod helpers;
mod keyboard;
mod memory;
mod mmu;

// Re-exports
pub use computer::{
    Computer, CLOCK_FREQ, DISPLAY_HEIGHT, DISPLAY_WIDTH, RAM_OFFSET, RAM_SIZE, ROM_OFFSET,
    ROM_SIZE, VRAM_OFFSET, VRAM_SIZE,
};
pub use cpu::{
    Addr, AsLargerType, Cpu, Flag, Flags, HasMax, Instruction, Msb, NMinus1Mask, NumBits, Oneable,
    Reg, Reg16, Reg32, Reg8, WrappingAdd, WrappingSub, Zeroable,
};
pub use drive::{DriveFlag, DRIVE_FLAGS_ADDR};
pub use memory::{MemReadable, MemWritable, Memory};
pub use mmu::{Interrupt, Mmu};
