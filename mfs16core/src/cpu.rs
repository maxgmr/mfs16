//! The virtual CPU hardware.
use std::{default::Default, fmt::Display};

mod addr;
mod flag;
mod instruction;
mod register;

// Re-exports
pub use addr::Addr;
pub use flag::{Flag, Flags, Msb, Oneable, Zeroable};
pub use instruction::{
    step, AsLargerType, HasMax, Instruction, NMinus1Mask, NumBits, WrappingAdd, WrappingSub,
};
pub use register::{Reg, Reg16, Reg32, Reg8};

use crate::{gpu::Gpu, ram::Ram};
use register::Registers;

const BYTES_IN_DWORD: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cpu {
    /// The CPU [Registers].
    pub regs: Registers,
    /// The CPU [Flags].
    pub flags: Flags,
    /// The program counter.
    pub pc: Addr,
    /// The stack pointer.
    pub sp: Addr,
    /// The current instruction.
    pub instr: Instruction,
    /// Step number within the current instruction.
    pub step_num: u32,
    /// If true, the CPU is halted and will not do anything until an interrupt.
    pub is_halted: bool,
    /// If true, print debug messages to stdout.
    pub debug: bool,
    /// The byte last read by the CPU.
    last_byte: u8,
    /// The word last read by the CPU.
    last_word: u16,
    /// The second-last word read by the CPU.
    second_last_word: u16,
    /// Whether the most recently-checked conditional was satisfied or not.
    last_conditional_satisfied: bool,
}
impl Cpu {
    /// Create a new [Cpu] with the given [Registers] and [Flags] values.
    pub fn new(regs: Registers, flags: Flags, debug: bool) -> Self {
        Self {
            regs,
            flags,
            debug,
            ..Self::default()
        }
    }

    /// Perform one clock cycle.
    pub fn cycle(&mut self, gpu: &mut Gpu, ram: &mut Ram) {
        // TODO check for interrupts

        if self.is_halted {
            return;
        }

        if self.step_num >= self.instr.num_steps() {
            // Current instruction is done; move on to the next one.
            self.step_num = 0;
            self.read_opcode(ram);
            if self.debug {
                println!("{}", self);
            }
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

    /// Push a value to the stack.
    fn push_stack(&mut self, ram: &mut Ram, value: u32) {
        self.sp.wrapping_sub(BYTES_IN_DWORD as u32);
        ram.write_dword(self.sp.into(), value);
    }

    /// Pop a value from the stack.
    fn pop_stack(&mut self, ram: &mut Ram) -> u32 {
        let popped_val = ram.read_dword(self.sp.into());
        self.sp.wrapping_add(BYTES_IN_DWORD as u32);
        popped_val
    }

    /// Jump the program counter to the given address.
    fn jump(&mut self, address: u32) {
        self.pc = Addr::new_wrapped(address)
    }

    /// Relative jump the program counter based on the given offset, interpreted as a signed value.
    fn relative_jump(&mut self, offset: u32) {
        self.pc.wrapping_add(offset);
    }

    /// Check the conditional against one of the [Flags].
    fn check_conditional(&mut self, flag: Flag, expected: bool) {
        self.last_conditional_satisfied = self.flag(flag) == expected;
    }
}
impl Default for Cpu {
    /// Default: Stack pointer at top of stack. Everything else initialised to 0/false.
    fn default() -> Self {
        Self {
            regs: Registers::default(),
            flags: Flags::default(),
            pc: Addr::default(),
            sp: Addr::default(),
            instr: Instruction::default(),
            step_num: Instruction::default().num_steps(),
            is_halted: false,
            debug: true,
            last_byte: 0x00,
            last_word: 0x0000,
            second_last_word: 0x0000,
            last_conditional_satisfied: false,
        }
    }
}
impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<10}|PC:{} SP:{}|{}|{}",
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
            pc: Addr::new((RAM_SIZE as u32) - 3),
            ..Cpu::default()
        };
        let mut ram = Ram::default();
        ram.write_word(0x00_0000, 0xABCD);
        ram.write_byte(0x00_0002, 0x01);
        ram.write_word((RAM_SIZE as u32) - 2, 0x2345);
        ram.write_byte((RAM_SIZE as u32) - 3, 0xFE);

        cpu.read_next_byte(&ram);
        assert_eq!(cpu.last_byte, 0xfe);
        assert_eq!(cpu.pc, Addr::new((RAM_SIZE as u32) - 2));

        cpu.read_next_word(&ram);
        assert_eq!(cpu.last_word, 0x2345);
        assert_eq!(cpu.pc, Addr::new(0x00_0000));

        cpu.read_next_word(&ram);
        assert_eq!(cpu.last_word, 0xABCD);
        assert_eq!(cpu.pc, Addr::new(0x00_0002));

        cpu.read_next_byte(&ram);
        assert_eq!(cpu.last_byte, 0x01);
        assert_eq!(cpu.pc, Addr::new(0x00_0003));
    }

    #[test]
    fn test_stack() {
        let mut cpu = Cpu::default();
        let mut ram = Ram::default();
        cpu.sp = Addr::new(0x00_0000);

        cpu.push_stack(&mut ram, 0x1234_5678);
        assert_eq!(cpu.sp, Addr::new(0xFF_FFFC));
        assert_eq!(ram.read_byte(0xFF_FFFF), 0x12);
        assert_eq!(ram.read_byte(0xFF_FFFE), 0x34);
        assert_eq!(ram.read_byte(0xFF_FFFD), 0x56);
        assert_eq!(ram.read_byte(0xFF_FFFC), 0x78);

        assert_eq!(cpu.pop_stack(&mut ram), 0x1234_5678);
        assert_eq!(cpu.sp, Addr::new(0x00_0000));
    }
}
