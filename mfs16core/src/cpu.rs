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

use crate::{
    computer::INTERRUPT_HANDLERS_OFFSET,
    mmu::{Mmu, IE_REGISTER_ADDR, INTERRUPT_REGISTER_ADDR},
    Interrupt, RAM_OFFSET, RAM_SIZE, ROM_OFFSET,
};
use register::Registers;

const BYTES_IN_DWORD: usize = 4;

/// The virtual CPU of the MFS-16 computer.
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
    /// If true, the CPU is stopped and will not do anything anymore.
    pub is_stopped: bool,
    /// If true, then maskable interrupts are enabled.
    pub interrupts_enabled: bool,
    /// If true, print debug messages to stdout.
    pub debug: bool,
    /// The total number of CPU cycles performed.
    pub total_cycles: u128,
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
    pub fn cycle(&mut self, mmu: &mut Mmu) {
        if self.is_stopped {
            return;
        }

        if self.handle_interrupts(mmu) {
            return;
        }

        if self.is_halted {
            return;
        }

        if self.step_num >= self.instr.num_steps() {
            // Current instruction is done; move on to the next one.
            self.step_num = 0;
            self.read_opcode(mmu);
            if self.debug {
                println!("{}", self);
            }
        } else {
            // Current instruction is in progress; perform the appropriate instruction step.
            step(self, mmu);
        }
        self.total_cycles += 1;
        self.step_num += 1;
    }

    /// Check whether the current instruction is done or not.
    pub fn instr_is_done(&self) -> bool {
        self.step_num >= self.instr.num_steps()
    }

    /// Handle interrupts, returning `true` iff any interrupts were handled.
    fn handle_interrupts(&mut self, mmu: &mut Mmu) -> bool {
        if !self.interrupts_enabled && !self.is_halted {
            return false;
        }

        let ie_register_val = mmu.read_byte(IE_REGISTER_ADDR as u32);
        let interrupt_register_val = mmu.read_byte(INTERRUPT_REGISTER_ADDR as u32);
        let activated_interrupts = ie_register_val & interrupt_register_val;
        if activated_interrupts == 0 {
            return false;
        }

        self.is_halted = false;

        if !self.interrupts_enabled {
            return false;
        }

        // Prioritise lowest activated interrupt
        let offset = activated_interrupts.trailing_zeros();
        if self.debug {
            println!("INTERRUPT: {}", Interrupt::from_byte(offset as u8));
        }
        mmu.write_byte(
            INTERRUPT_REGISTER_ADDR as u32,
            interrupt_register_val & !(1 << offset),
        );
        self.push_stack(mmu, self.pc.address());
        self.pc = Addr::new_default_range(
            ((ROM_OFFSET + INTERRUPT_HANDLERS_OFFSET) | ((offset as usize) << 3)) as u32,
        );
        true
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
    fn read_opcode(&mut self, mmu: &mut Mmu) {
        self.read_next_word(mmu);
        self.instr = Instruction::from_opcode(self.last_word);
    }

    /// Read a single byte from MMU at the program counter, advancing the program counter
    /// accordingly.
    fn read_next_byte(&mut self, mmu: &Mmu) {
        self.last_byte = mmu.read_byte(self.pc.address());
        self.pc.wrapping_inc();
    }

    /// Read a single word from MMU at the program counter, advancing the program counter
    /// accordingly.
    fn read_next_word(&mut self, mmu: &Mmu) {
        self.update_last_word(mmu.read_word(self.pc.address()));
        self.pc.wrapping_inc();
        self.pc.wrapping_inc();
    }

    /// Read a single word from MMU pointed to by the provided address.
    fn read_word_at_addr(&mut self, mmu: &Mmu, addr: u32) {
        self.update_last_word(mmu.read_word(addr));
    }

    /// Push a value to the stack.
    fn push_stack(&mut self, mmu: &mut Mmu, value: u32) {
        self.sp.wrapping_sub(BYTES_IN_DWORD as u32);
        mmu.write_dword(self.sp.address(), value);
    }

    /// Pop a value from the stack.
    fn pop_stack(&mut self, mmu: &mut Mmu) -> u32 {
        let popped_val = mmu.read_dword(self.sp.address());
        self.sp.wrapping_add(BYTES_IN_DWORD as u32);
        popped_val
    }

    /// Jump the program counter to the given address.
    fn jump(&mut self, address: u32) {
        self.pc = Addr::new_wrapped(0, <u32>::MAX as usize, address)
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
            sp: Addr::new(RAM_OFFSET, RAM_SIZE - 1, RAM_OFFSET as u32),
            instr: Instruction::default(),
            step_num: Instruction::default().num_steps(),
            is_halted: false,
            is_stopped: false,
            interrupts_enabled: false,
            debug: true,
            total_cycles: 0,
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

    #[test]
    fn test_stack() {
        let mut cpu = Cpu::default();
        let mut mmu = Mmu::default();

        cpu.push_stack(&mut mmu, 0x1234_5678);
        assert_eq!(cpu.sp.address(), ((RAM_OFFSET + RAM_SIZE) as u32) - 4);
        assert_eq!(mmu.read_byte(((RAM_OFFSET + RAM_SIZE) as u32) - 1), 0x12);
        assert_eq!(mmu.read_byte(((RAM_OFFSET + RAM_SIZE) as u32) - 2), 0x34);
        assert_eq!(mmu.read_byte(((RAM_OFFSET + RAM_SIZE) as u32) - 3), 0x56);
        assert_eq!(mmu.read_byte(((RAM_OFFSET + RAM_SIZE) as u32) - 4), 0x78);

        assert_eq!(cpu.pop_stack(&mut mmu), 0x1234_5678);
        assert_eq!(cpu.sp.address(), RAM_OFFSET as u32);
        assert_eq!(cpu.sp.relative_address(), 0);
    }
}
