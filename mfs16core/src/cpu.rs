//! The virtual CPU hardware.
use std::default::Default;

mod flag;
mod instruction;
mod pc;
mod register;

// Re-exports
pub use flag::Flag;
pub use register::{Reg16, Reg8};

use crate::ram::Ram;
use flag::Flags;
use instruction::{
    step,
    Instruction::{self, *},
};
use pc::Pc;
use register::Registers;

#[derive(Debug)]
pub struct Cpu {
    /// The CPU [Registers].
    pub regs: Registers,
    /// The CPU [Flags].
    pub flags: Flags,
    /// The program counter.
    pub pc: Pc,
    /// The stack pointer.
    pub sp: u32,
    /// The current instruction.
    pub instr: Instruction,
    /// Step number within the current instruction.
    pub step_num: u32,
}
impl Cpu {
    /// Create a new [Cpu] with the given [Registers] and [Flags] values.
    pub fn new(regs: Registers, flags: Flags) -> Self {
        Self {
            regs,
            flags,
            pc: Pc::new(0x0000_0000),
            sp: 0xFFFF_FFFF,
            instr: Nop,
            step_num: Nop.num_steps(),
        }
    }

    /// Perform one clock cycle.
    pub fn cycle(&mut self, ram: &mut Ram) {
        dbg!(&self.step_num, &self.instr.num_steps());
        if self.step_num >= self.instr.num_steps() {
            // Current instruction is done; move on to the next one.
            self.step_num = 0;
            self.read_opcode(ram);
        } else {
            // Current instruction is in progress; perform the appropriate instruction step.
            step(self, ram);
        }
        self.step_num += 1;
    }

    /// Set the current instruction.
    fn read_opcode(&mut self, ram: &mut Ram) {
        self.instr = Instruction::from_opcode(self.read_next_word(ram));
    }

    /// Read a single word from RAM at the program counter, advancing the program counter.
    fn read_next_word(&mut self, ram: &Ram) -> u16 {
        let val = ram.read_word(self.pc.into());
        self.pc.wrapping_inc();
        self.pc.wrapping_inc();
        val
    }
}
impl Default for Cpu {
    /// Default: Stack pointer at top of stack. Everything else initialised to 0/false.
    fn default() -> Self {
        Self {
            regs: Registers::default(),
            flags: Flags::default(),
            pc: Pc::default(),
            sp: 0xFFFF_FFFF,
            instr: Instruction::default(),
            step_num: Instruction::default().num_steps(),
        }
    }
}
