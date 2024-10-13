//! The CPU instruction set.
//! rn = register n
//! vrn = 8-bit virtual half-register n
//! brn = 32-bit big register n
//! m = memory address, imm{n} = n-bit immediate value
use std::fmt::Display;

mod helpers;

use super::{Cpu, Ram};
use crate::{Reg16, Reg32, Reg8};
use helpers::*;
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
    /// 0x01A0 - LD SP,imm32
    /// Load 32-bit immediate value into stack pointer.
    /// SP = imm32
    LdSpImm32,
    /// 0x01A1 - LD [imm32],SP
    /// Load stack pointer into the two words (little-endian) starting at address imm32.
    /// [imm32] = SP
    LdImm32Sp,
    /// 0x01Ba - LD SP,bra
    /// Load register bra into stack pointer.
    /// SP = bra
    LdSpBra(Reg32),
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
    /// 0x033a - LD [bra],imm16
    /// Load 16-bit immediate value in the byte pointed to by 32-bit big register bra.
    LdBraImm16(Reg32),
    /// 0x04ab - LD [bra],rb
    /// Load rb in the word starting at the address pointed to by bra.
    /// [bra] = rb
    LdBraRb(Reg32, Reg16),
    /// 0x05ab - LD ra,[brb]
    /// Load the word pointed to by brb in register ra.
    /// ra = [brb]
    LdRaBrb(Reg16, Reg32),
    /// 0x06ab - LDI [bra],rb
    /// Load rb into [bra], then increment bra by two.
    /// [bra] = rb; bra += 2
    LdiBraRb(Reg32, Reg16),
    /// 0x07ab - LDD [bra],rb
    /// Load rb in [bra], then decrement bra by two.
    /// [bra] = rb; bra -= 2
    LddBraRb(Reg32, Reg16),
    /// 0x08ab - LDI ra,[brb]
    /// Load [brb] in ra, then increment brb by two.
    /// ra = [brb]; brb += 2
    LdiRaBrb(Reg16, Reg32),
    /// 0x09ab - LDD ra,[brb]
    /// Load [brb] in ra, then decrement brb by two.
    /// ra = [brb]; brb -= 2
    LddRaBrb(Reg16, Reg32),
    /// 0x10ab - ADD ra,rb
    /// Add rb to ra.
    /// ra += rb
    /// FLAGS: Z,C,O,P,N
    AddRaRb(Reg16, Reg16),
    /// 0x10(a+7)(b+7) - ADD bra,brb
    /// Add brb to bra. 32-bit addition.
    /// bra += brb
    /// FLAGS: Z,C,O,P,N
    AddBraBrb(Reg32, Reg32),
    /// 0x11ab - ADD vra,vrb
    /// Add vrb to vra. 8-bit addition.
    /// vra += vrb
    /// FLAGS: Z,C,O,P,N
    AddVraVrb(Reg8, Reg8),
}
impl Instruction {
    /// Get the [Instruction] from the given opcode.
    pub fn from_opcode(opcode: u16) -> Self {
        // The last nibble of some instructions reserve numbers 0-6 for the 16-bit registers, with
        // codes for the 32-bit big registers starting at 7.
        let reg_nib_offset = 7;

        let nib_1 = (opcode >> 12) as u8;
        let nib_2 = ((opcode & 0x0F00) >> 8) as u8;
        let nib_3 = ((opcode & 0x00F0) >> 4) as u8;
        let nib_4 = (opcode & 0x000F) as u8;

        match (nib_1, nib_2, nib_3, nib_4) {
            (0x0, 0x0, _, _) => Nop,
            (0x0, 0x1, 0xA, 0x0) => LdSpImm32,
            (0x0, 0x1, 0xA, 0x1) => LdImm32Sp,
            (0x0, 0x1, 0xB, bra) => LdSpBra(Reg32::from_nib(bra)),
            (0x0, 0x1, ra, rb) if ra < reg_nib_offset && rb < reg_nib_offset => {
                LdRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x0, 0x1, bra, brb) => LdBraBrb(
                Reg32::from_nib(bra - reg_nib_offset),
                Reg32::from_nib(brb - reg_nib_offset),
            ),
            (0x0, 0x2, vra, vrb) => LdVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x0, 0x3, 0x0, ra) => LdRaImm16(Reg16::from_nib(ra)),
            (0x0, 0x3, 0x1, bra) => LdBraImm32(Reg32::from_nib(bra)),
            (0x0, 0x3, 0x2, vra) => LdVraImm8(Reg8::from_nib(vra)),
            (0x0, 0x3, 0x3, bra) => LdBraImm16(Reg32::from_nib(bra)),
            (0x0, 0x4, bra, rb) => LdBraRb(Reg32::from_nib(bra), Reg16::from_nib(rb)),
            (0x0, 0x5, ra, brb) => LdRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x0, 0x6, bra, rb) => LdiBraRb(Reg32::from_nib(bra), Reg16::from_nib(rb)),
            (0x0, 0x7, bra, rb) => LddBraRb(Reg32::from_nib(bra), Reg16::from_nib(rb)),
            (0x0, 0x8, ra, brb) => LdiRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x0, 0x9, ra, brb) => LddRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0x0, ra, rb) if ra < reg_nib_offset && rb < reg_nib_offset => {
                AddRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x0, bra, brb) => AddBraBrb(
                Reg32::from_nib(bra - reg_nib_offset),
                Reg32::from_nib(brb - reg_nib_offset),
            ),
            (0x1, 0x1, vra, vrb) => AddVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            _ => panic!("Opcode {:#04X} has no corresponding instruction.", opcode),
        }
    }
    /// Return the number of CPU steps this instruction takes to execute.
    pub fn num_steps(&self) -> u32 {
        match self {
            Nop => 2,
            LdRaRb(..) => 2,
            LdBraBrb(..) => 2,
            LdSpImm32 => 4,
            LdImm32Sp => 4,
            LdSpBra(..) => 2,
            LdVraVrb(..) => 2,
            LdRaImm16(..) => 3,
            LdBraImm32(..) => 4,
            LdVraImm8(..) => 3,
            LdBraImm16(..) => 3,
            LdBraRb(..) => 3,
            LdRaBrb(..) => 3,
            LdiBraRb(..) => 3,
            LddBraRb(..) => 3,
            LdiRaBrb(..) => 3,
            LddRaBrb(..) => 3,
            AddRaRb(..) => 2,
            AddVraVrb(..) => 2,
            AddBraBrb(..) => 2,
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
                LdSpImm32 => String::from("LD SP,imm32"),
                LdImm32Sp => String::from("LD [imm32],SP"),
                LdSpBra(bra) => format!("LD SP,{bra}"),
                LdVraVrb(vra, vrb) => format!("LD {vra},{vrb}"),
                LdRaImm16(ra) => format!("LD {ra},imm16"),
                LdBraImm32(bra) => format!("LD {bra},imm32"),
                LdVraImm8(vra) => format!("LD {vra},imm8"),
                LdBraImm16(bra) => format!("LD [{bra}],imm16"),
                LdBraRb(bra, rb) => format!("LD [{bra}],{rb}"),
                LdRaBrb(ra, brb) => format!("LD {ra},[{brb}]"),
                LdiBraRb(bra, rb) => format!("LDI [{bra}],{rb}"),
                LddBraRb(bra, rb) => format!("LDD [{bra}],{rb}"),
                LdiRaBrb(ra, brb) => format!("LDI {ra},[{brb}]"),
                LddRaBrb(ra, brb) => format!("LDD {ra},[{brb}]"),
                AddRaRb(ra, rb) => format!("ADD {ra},{rb}"),
                AddBraBrb(bra, brb) => format!("ADD {bra},{brb}"),
                AddVraVrb(vra, vrb) => format!("ADD {vra},{vrb}"),
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
        LdSpImm32 => ld_sp_imm32(cpu, ram),
        LdImm32Sp => ld_imm32_sp(cpu, ram),
        LdSpBra(bra) => ld_sp_bra(cpu, bra),
        LdVraVrb(vra, vrb) => ld_vra_vrb(cpu, vra, vrb),
        LdRaImm16(ra) => ld_ra_imm16(cpu, ram, ra),
        LdBraImm32(bra) => ld_bra_imm32(cpu, ram, bra),
        LdVraImm8(vra) => ld_vra_imm8(cpu, ram, vra),
        LdBraImm16(bra) => ld_bra_imm16(cpu, ram, bra),
        LdBraRb(bra, rb) => ld_bra_rb(cpu, ram, bra, rb),
        LdRaBrb(ra, brb) => ld_ra_brb(cpu, ram, ra, brb),
        LdiBraRb(bra, rb) => ldi_bra_rb(cpu, ram, bra, rb),
        LddBraRb(bra, rb) => ldd_bra_rb(cpu, ram, bra, rb),
        LdiRaBrb(ra, brb) => ldi_ra_brb(cpu, ram, ra, brb),
        LddRaBrb(ra, brb) => ldd_ra_brb(cpu, ram, ra, brb),
        AddRaRb(ra, rb) => add_ra_rb(cpu, ra, rb),
        AddBraBrb(bra, brb) => add_bra_brb(cpu, bra, brb),
        AddVraVrb(vra, vrb) => add_vra_vrb(cpu, vra, vrb),
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
        1 => cpu.set_reg(ra, cpu.reg(rb)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_brb(cpu: &mut Cpu, bra: Reg32, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.set_breg(bra, cpu.breg(brb)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_sp_imm32(cpu: &mut Cpu, ram: &mut Ram) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => cpu.read_next_word(ram),
        3 => cpu.sp = get_dword_from_last(cpu),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_imm32_sp(cpu: &mut Cpu, ram: &mut Ram) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => cpu.read_next_word(ram),
        3 => write_dword_to_last(cpu, ram, cpu.sp),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_sp_bra(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.sp = cpu.breg(bra),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_vra_vrb(cpu: &mut Cpu, vra: Reg8, vrb: Reg8) {
    match cpu.step_num {
        1 => cpu.set_vreg(vra, cpu.vreg(vrb)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_ra_imm16(cpu: &mut Cpu, ram: &Ram, ra: Reg16) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => cpu.set_reg(ra, cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_imm16(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => ram.write_word(cpu.breg(bra), cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_imm32(cpu: &mut Cpu, ram: &Ram, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => cpu.read_next_word(ram),
        3 => cpu.set_breg(bra, get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_vra_imm8(cpu: &mut Cpu, ram: &Ram, vra: Reg8) {
    match cpu.step_num {
        1 => cpu.read_next_byte(ram),
        2 => cpu.set_vreg(vra, cpu.last_byte),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_rb(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => ram.write_word(cpu.breg(bra), cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_ra_brb(cpu: &mut Cpu, ram: &mut Ram, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.update_last_word(ram.read_word(cpu.breg(brb))),
        2 => cpu.set_reg(ra, cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldi_bra_rb(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => {
            ram.write_word(cpu.breg(bra), cpu.last_word);
            dbl_inc_br(cpu, bra);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldd_bra_rb(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => {
            ram.write_word(cpu.breg(bra), cpu.last_word);
            dbl_dec_br(cpu, bra);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldi_ra_brb(cpu: &mut Cpu, ram: &mut Ram, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.update_last_word(ram.read_word(cpu.breg(brb))),
        2 => {
            cpu.set_reg(ra, cpu.last_word);
            dbl_inc_br(cpu, brb);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldd_ra_brb(cpu: &mut Cpu, ram: &mut Ram, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.update_last_word(ram.read_word(cpu.breg(brb))),
        2 => {
            cpu.set_reg(ra, cpu.last_word);
            dbl_dec_br(cpu, brb);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn add_ra_rb(cpu: &mut Cpu, ra: Reg16, rb: Reg16) {
    match cpu.step_num {
        1 => {
            let result = alu_add16(cpu, ra, rb, false);
            cpu.set_reg(ra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn add_bra_brb(cpu: &mut Cpu, bra: Reg32, brb: Reg32) {
    match cpu.step_num {
        1 => {
            let result = alu_add32(cpu, bra, brb, false);
            cpu.set_breg(bra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn add_vra_vrb(cpu: &mut Cpu, vra: Reg8, vrb: Reg8) {
    match cpu.step_num {
        1 => {
            let result = alu_add8(cpu, vra, vrb, false);
            cpu.set_vreg(vra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}
