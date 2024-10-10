//! The virtual CPU hardware.

mod register;

use register::Registers;

#[derive(Debug)]
pub struct Cpu {
    /// The CPU [Registers].
    pub regs: Registers,
}
