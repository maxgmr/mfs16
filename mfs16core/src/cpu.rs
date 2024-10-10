//! The virtual CPU hardware.
mod flag;
mod register;

use flag::Flags;
use register::Registers;

/// CPU clock frequency: 33_554_432 Hz (33.55 MHz)
pub const CLOCK_FREQ: u32 = 2_u32.pow(25);

#[derive(Debug)]
pub struct Cpu {
    /// The CPU [Registers].
    pub regs: Registers,
    /// The CPU [Flags].
    pub flags: Flags,
    /// The program counter.
    pub pc: u32,
    /// The stack pointer.
    pub sp: u32,
    /// The cycle counter.
    pub cycles: u128,
}
