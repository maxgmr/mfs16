//! The virtual CPU hardware.
use std::{default::Default, fmt::Display};

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

const DEBUG: bool = true;

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
        if self.step_num >= self.instr.num_steps() {
            // Current instruction is done; move on to the next one.
            if DEBUG {
                println!("{}", self);
            }
            self.step_num = 0;
            self.read_opcode(ram);
        } else {
            // Current instruction is in progress; perform the appropriate instruction step.
            step(self, ram);
        }
        self.step_num += 1;
    }

    /// Wrapper function for self.regs.reg(Reg16). Fetch the value of the given CPU register.
    pub fn reg(&self, reg: Reg16) -> u16 {
        self.regs.reg(reg)
    }

    /// Wrapper function for self.regs.set_reg(Reg16). Set the value of the given CPU register.
    pub fn set_reg(&mut self, reg: Reg16, val: u16) {
        self.regs.set_reg(reg, val)
    }

    /// Wrapper function for self.regs.vreg(Reg8). Fetch the value of the given 8-bit virtual CPU
    /// register.
    pub fn vreg(&self, vreg: Reg8) -> u8 {
        self.regs.vreg(vreg)
    }

    /// Wrapper function for self.regs.set_vreg(Reg8). Set the value of the given 8-bit virtual CPU
    /// register.
    pub fn set_vreg(&mut self, vreg: Reg8, val: u8) {
        self.regs.set_vreg(vreg, val)
    }

    /// Wrapper function for self.flags.flag. Fetch the value of the given flag.
    pub fn flag(&self, flag: Flag) -> bool {
        self.flags.flag(flag)
    }

    /// Wrapper function for self.flags.set_flag. Set the value of the given flag.
    pub fn set_flag(&mut self, flag: Flag) {
        self.flags.set_flag(flag)
    }

    /// Wrapper function for self.flags.reset_flag. Reset the value of the given flag.
    pub fn reset_flag(&mut self, flag: Flag) {
        self.flags.reset_flag(flag)
    }

    /// Wrapper function for self.flags.change_flag. Change the flag to the given value.
    pub fn change_flag(&mut self, flag: Flag, val: bool) {
        self.flags.change_flag(flag, val)
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
impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<10}|PC:{} SP:{:#010X}|{}|{}",
            self.instr, self.pc, self.sp, self.regs, self.flags
        )
    }
}
