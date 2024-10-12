//! The CPU instruction set.
//! rn = register n, vrn = 8-bit virtual half-register n, m = memory address, imm{n} = n-bit immediate value
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
    /// 0x01ab - LD ra,rb
    /// 16-bit register-register load.
    /// ra = rb
    LdRaRb(Reg16, Reg16),
    /// 0x02ab - LD vra,vrb
    /// 8-bit register-register load.
    /// vra = vrb
    LdVraVrb(Reg8, Reg8),
    /// 0x03a_ - LD ra,imm16
    /// Load 16-bit immediate value into register ra.
    LdRaImm16(Reg16),
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
            (0x02, vra, vrb) => LdVraVrb(Reg8::from_nibble(vra), Reg8::from_nibble(vrb)),
            (0x03, ra, _) => LdRaImm16(Reg16::from_nibble(ra)),
            _ => panic!("Opcode {:#04X} has no corresponding instruction.", opcode),
        }
    }
    /// Return the number of CPU steps this instruction takes to execute.
    pub fn num_steps(&self) -> u32 {
        match self {
            Nop => 2,
            LdRaRb(..) => 2,
            LdVraVrb(..) => 2,
            LdRaImm16(..) => 3,
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
                LdRaRb(ra, rb) => format!("LD {ra},{rb}"),
                LdVraVrb(vra, vrb) => format!("LD {vra},{vrb}"),
                LdRaImm16(ra) => format!("LD {ra},IMM16"),
            }
        )
    }
}

/// Perform the current step of the current CPU instruction.
pub fn step(cpu: &mut Cpu, ram: &mut Ram) {
    match cpu.instr {
        Nop => {}
        LdRaRb(ra, rb) => ld_ra_rb(cpu, ra, rb),
        LdVraVrb(vra, vrb) => ld_vra_vrb(cpu, vra, vrb),
        LdRaImm16(ra) => ld_ra_imm16(cpu, ram, ra),
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
            let val = cpu.reg(rb);
            cpu.set_reg(ra, val);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_vra_vrb(cpu: &mut Cpu, vra: Reg8, vrb: Reg8) {
    match cpu.step_num {
        1 => {
            let val = cpu.vreg(vrb);
            cpu.set_vreg(vra, val);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_ra_imm16(cpu: &mut Cpu, ram: &Ram, ra: Reg16) {
    match cpu.step_num {
        1 => {}
        2 => {
            let val = cpu.read_next_word(ram);
            cpu.set_reg(ra, val);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

#[cfg(test)]
mod instruction_tests;
