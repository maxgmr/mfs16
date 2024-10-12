//! The virtual CPU hardware.
use std::{default::Default, fmt::Display};

mod flag;
mod instruction;
mod pc;
mod register;

// Re-exports
pub use flag::Flag;
pub use pc::Pc;
pub use register::{Reg16, Reg32, Reg8};

use crate::ram::Ram;
use flag::Flags;
use instruction::{step, Instruction};
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
    /// The byte last read by the CPU.
    last_byte: u8,
    /// The word last read by the CPU.
    last_word: u16,
    /// The second-last word read by the CPU.
    second_last_word: u16,
}
impl Cpu {
    /// Create a new [Cpu] with the given [Registers] and [Flags] values.
    pub fn new(regs: Registers, flags: Flags) -> Self {
        Self {
            regs,
            flags,
            ..Self::default()
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

    /// Wrapper function for self.regs.breg(Reg32). Fetch the value of the given 32-bit big
    /// register.
    pub fn breg(&self, breg: Reg32) -> u32 {
        self.regs.breg(breg)
    }

    /// Wrapper function for self.regs.set_breg(Reg32). Set the value of the given 32-bit big
    /// register.
    pub fn set_breg(&mut self, breg: Reg32, val: u32) {
        self.regs.set_breg(breg, val)
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

    /// Update the last word read by the CPU.
    pub fn update_last_word(&mut self, word: u16) {
        self.second_last_word = self.last_word;
        self.last_word = word;
    }

    /// Set the current instruction.
    fn read_opcode(&mut self, ram: &mut Ram) {
        self.read_next_word(ram);
        self.instr = Instruction::from_opcode(self.last_word);
    }

    /// Read a single byte from RAM at the program counter, advancing the program counter
    /// accordingly.
    fn read_next_byte(&mut self, ram: &Ram) {
        self.last_byte = ram.read_byte(self.pc.into());
        self.pc.wrapping_inc();
    }

    /// Read a single word from RAM at the program counter, advancing the program counter
    /// accordingly.
    fn read_next_word(&mut self, ram: &Ram) {
        self.update_last_word(ram.read_word(self.pc.into()));
        self.pc.wrapping_inc();
        self.pc.wrapping_inc();
    }

    /// Read a single word from RAM pointed to by the provided address.
    fn read_word_at_addr(&mut self, ram: &Ram, addr: u32) {
        self.update_last_word(ram.read_word(addr));
    }

    // /// Read a single byte from RAM before the program counter. Does not increment the program
    // /// counter.
    // fn read_prev_byte(&self, ram: &Ram) -> u8 {
    //     let mut address = self.pc;
    //     address.wrapping_dec();
    //     ram.read_byte(address.into())
    // }
    //
    // /// Read a single word from RAM before the program counter. Does not increment the program
    // /// counter.
    // fn read_prev_word(&self, ram: &Ram) -> u16 {
    //     let mut address = self.pc;
    //     address.wrapping_dec();
    //     address.wrapping_dec();
    //     ram.read_word(address.into())
    // }
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
            last_byte: 0x00,
            last_word: 0x0000,
            second_last_word: 0x0000,
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::RAM_SIZE;

    #[test]
    fn test_read() {
        let mut cpu = Cpu {
            pc: Pc::new((RAM_SIZE as u32) - 3),
            ..Cpu::default()
        };
        let mut ram = Ram::default();
        ram.write_word(0x00_0000, 0xABCD);
        ram.write_byte(0x00_0002, 0x01);
        ram.write_word((RAM_SIZE as u32) - 2, 0x2345);
        ram.write_byte((RAM_SIZE as u32) - 3, 0xFE);

        cpu.read_next_byte(&ram);
        assert_eq!(cpu.last_byte, 0xfe);
        assert_eq!(cpu.pc, Pc::new((RAM_SIZE as u32) - 2));

        cpu.read_next_word(&ram);
        assert_eq!(cpu.last_word, 0x2345);
        assert_eq!(cpu.pc, Pc::new(0x00_0000));

        cpu.read_next_word(&ram);
        assert_eq!(cpu.last_word, 0xABCD);
        assert_eq!(cpu.pc, Pc::new(0x00_0002));

        cpu.read_next_byte(&ram);
        assert_eq!(cpu.last_byte, 0x01);
        assert_eq!(cpu.pc, Pc::new(0x00_0003));
    }
}
