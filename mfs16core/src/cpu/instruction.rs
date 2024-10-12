//! The CPU instruction set.
//! rn = register n
//! vrn = 8-bit virtual half-register n
//! brn = 32-bit big register n
//! m = memory address, imm{n} = n-bit immediate value
use std::fmt::Display;

use super::{Cpu, Ram};
use crate::{helpers::combine_u16_be, Reg16, Reg32, Reg8};
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
    /// 0x01(a+7)(b+7) - LD bra,brb
    /// 32-bit register-register load.
    /// bra = brb
    LdBraBrb(Reg32, Reg32),
    /// 0x02ab - LD vra,vrb
    /// 8-bit register-register load.
    /// vra = vrb
    LdVraVrb(Reg8, Reg8),
    /// 0x030a - LD ra,imm16
    /// Load 16-bit immediate value into register ra.
    /// ra = imm16
    LdRaImm16(Reg16),
    /// 0x031a - LD bra,imm32
    /// Load 32-bit immediate value into 32-bit big register bra.
    /// bra = imm32
    LdBraImm32(Reg32),
    /// 0x032a - LD vra,imm8
    /// Load 8-bit immediate value in 8-bit virtual register vra.
    /// vra = imm8
    LdVraImm8(Reg8),
    /// 0x04ab - LD [bra],rb
    /// Load rb in the word starting at the address pointed to by bra.
    /// [bra] = rb
    LdBraRb(Reg32, Reg16),
}
impl Instruction {
    /// Get the [Instruction] from the given opcode.
    pub fn from_opcode(opcode: u16) -> Self {
        let nib_1 = (opcode >> 12) as u8;
        let nib_2 = ((opcode & 0x0F00) >> 8) as u8;
        let nib_3 = ((opcode & 0x00F0) >> 4) as u8;
        let nib_4 = (opcode & 0x000F) as u8;
        match (nib_1, nib_2, nib_3, nib_4) {
            (0x0, 0x0, _, _) => Nop,
            (0x0, 0x1, ra, rb) if ra < 7 && rb < 7 => {
                LdRaRb(Reg16::from_nibble(ra), Reg16::from_nibble(rb))
            }
            (0x0, 0x1, bra, brb) => {
                LdBraBrb(Reg32::from_nibble(bra - 7), Reg32::from_nibble(brb - 7))
            }
            (0x0, 0x2, vra, vrb) => LdVraVrb(Reg8::from_nibble(vra), Reg8::from_nibble(vrb)),
            (0x0, 0x3, 0x0, ra) => LdRaImm16(Reg16::from_nibble(ra)),
            (0x0, 0x3, 0x1, bra) => LdBraImm32(Reg32::from_nibble(bra)),
            (0x0, 0x3, 0x2, vra) => LdVraImm8(Reg8::from_nibble(vra)),
            (0x0, 0x4, bra, rb) => LdBraRb(Reg32::from_nibble(bra), Reg16::from_nibble(rb)),
            _ => panic!("Opcode {:#04X} has no corresponding instruction.", opcode),
        }
    }
    /// Return the number of CPU steps this instruction takes to execute.
    pub fn num_steps(&self) -> u32 {
        match self {
            Nop => 2,
            LdRaRb(..) => 2,
            LdBraBrb(..) => 2,
            LdVraVrb(..) => 2,
            LdRaImm16(..) => 3,
            LdBraImm32(..) => 4,
            LdVraImm8(..) => 3,
            LdBraRb(..) => 3,
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
                LdBraBrb(bra, brb) => format!("LD {bra},{brb}"),
                LdVraVrb(vra, vrb) => format!("LD {vra},{vrb}"),
                LdRaImm16(ra) => format!("LD {ra},imm16"),
                LdBraImm32(bra) => format!("LD {bra},imm32"),
                LdVraImm8(vra) => format!("LD {vra},imm8"),
                LdBraRb(bra, rb) => format!("LD [{bra}],{rb}"),
            }
        )
    }
}

/// Perform the current step of the current CPU instruction.
pub fn step(cpu: &mut Cpu, ram: &mut Ram) {
    match cpu.instr {
        Nop => {}
        LdRaRb(ra, rb) => ld_ra_rb(cpu, ra, rb),
        LdBraBrb(bra, brb) => ld_bra_brb(cpu, bra, brb),
        LdVraVrb(vra, vrb) => ld_vra_vrb(cpu, vra, vrb),
        LdRaImm16(ra) => ld_ra_imm16(cpu, ram, ra),
        LdBraImm32(bra) => ld_bra_imm32(cpu, ram, bra),
        LdVraImm8(vra) => ld_vra_imm8(cpu, ram, vra),
        LdBraRb(bra, rb) => ld_bra_rb(cpu, ram, bra, rb),
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

fn ld_bra_brb(cpu: &mut Cpu, bra: Reg32, brb: Reg32) {
    match cpu.step_num {
        1 => {
            let val = cpu.breg(brb);
            cpu.set_breg(bra, val);
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
        1 => {
            cpu.read_next_word(ram);
        }
        2 => {
            cpu.set_reg(ra, cpu.last_word);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_imm32(cpu: &mut Cpu, ram: &Ram, bra: Reg32) {
    match cpu.step_num {
        1 => {
            cpu.read_next_word(ram);
        }
        2 => {
            cpu.read_next_word(ram);
        }
        3 => {
            let val = combine_u16_be(cpu.last_word, cpu.second_last_word);
            println!("{:#06X}", cpu.last_word);
            println!("{:#06X}", cpu.second_last_word);
            cpu.set_breg(bra, val);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_vra_imm8(cpu: &mut Cpu, ram: &Ram, vra: Reg8) {
    match cpu.step_num {
        1 => {
            cpu.read_next_byte(ram);
        }
        2 => {
            cpu.set_vreg(vra, cpu.last_byte);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_rb(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => {
            cpu.update_last_word(cpu.reg(rb));
            dbg!(&cpu.pc);
        }
        2 => {
            dbg!(&cpu.pc);
            let addr = cpu.breg(bra);
            dbg!(&cpu.pc);
            ram.write_word(addr, cpu.last_word);
            dbg!(&cpu.pc);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}
