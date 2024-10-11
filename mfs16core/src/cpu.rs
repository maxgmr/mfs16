//! The virtual CPU hardware.
use std::default::Default;

mod flag;
mod instruction;
mod register;

use flag::Flags;
use register::Registers;

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
}
impl Cpu {
    /// Create a new [Cpu] with the given [Registers] and [Flags] values.
    pub fn new(regs: Registers, flags: Flags) -> Self {
        Self {
            regs,
            flags,
            pc: 0x0000_0000,
            sp: 0xFFFF_FFFF,
        }
    }
}
impl Default for Cpu {
    /// Default: Stack pointer at top of stack. Everything else initialised to 0/false.
    fn default() -> Self {
        Self {
            regs: Registers::default(),
            flags: Flags::default(),
            pc: 0x0000_0000,
            sp: 0xFFFF_FFFF,
        }
    }
}
