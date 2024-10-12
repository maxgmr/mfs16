//! The CPU instruction set.
//! rn = register n, 8rn = 8-bit half-register n, m = memory address, imm{n} = n-bit immediate value
use std::fmt::Display;

use super::{flag::Flags, register::Registers, Cpu, Ram};
use crate::{Flag, Reg16, Reg8};
use Instruction::*;

/// Enum for accessing the different CPU instructions.
#[derive(Debug, Clone, Copy, Default)]
pub enum Instruction {
    #[default]
    /// 0x0000 - NOP
    /// Do nothing for 4 cycles.
    Nop,
    /// 0x01ab - LD ra, rb
    /// ra = rb
    LdRaRb(Reg16, Reg16),
}
impl Instruction {
    /// Get the [Instruction] from the given opcode.
    pub fn from_opcode(opcode: u16) -> Self {
        let opcode_main = (opcode >> 8) as u8;
        let arg_1 = ((opcode & 0x00F0) >> 4) as u8;
        let arg_2 = (opcode & 0x000F) as u8;
        match (opcode_main, arg_1, arg_2) {
            (0x00, _, _) => Nop,
            (0x01, ra, rb) => LdRaRb(Reg16::from_nibble(ra), Reg16::from_nibble(rb)),
            _ => panic!("Opcode {:#04X} has no corresponding instruction.", opcode),
        }
    }
    /// Return the number of CPU steps this instruction takes to execute.
    pub fn num_steps(&self) -> u32 {
        match self {
            Nop => 2,
            LdRaRb(..) => 2,
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<10}",
            match self {
                Nop => String::from("NOP"),
                LdRaRb(ra, rb) => format!("LD {}, {}", ra, rb),
            }
        )
    }
}

/// Perform the current step of the current CPU instruction.
pub fn step(cpu: &mut Cpu, ram: &mut Ram) {
    match cpu.instr {
        Nop => {}
        LdRaRb(ra, rb) => ld_ra_rb(cpu, ra, rb),
        _ => unimplemented!("Instruction {} is unimplemented.", cpu.instr),
    }
}

fn invalid_step_panic(instr: Instruction, step_num: u32) {
    panic!(
        "Invalid step number {} for instruction {} ({} steps)",
        step_num,
        instr,
        instr.num_steps()
    );
}

// ------- CPU INSTRUCTION FUNCTIONS -------
fn ld_ra_rb(cpu: &mut Cpu, ra: Reg16, rb: Reg16) {
    match cpu.step_num {
        1 => {
            let val = cpu.regs.reg(rb);
            cpu.regs.set_reg(ra, val);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

#[cfg(test)]
mod instruction_tests;
