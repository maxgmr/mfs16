//! The CPU instruction set.
//! rn = [Reg16] n
//! vrn = 8-bit virtual [Reg8] n
//! brn = 32-bit big [Reg32] n
//! imm{n} = n-bit immediate value
//! cn = [ConditionKind] n
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[cfg(test)]
use strum_macros::EnumIter;

mod alu;
mod fpu;
mod helpers;
mod instruction_helpers;
mod instruction_impl;

use super::Cpu;
use crate::{mmu::Mmu, Reg16, Reg32, Reg8};
use alu::{AluDblOp::*, AluOp::*, *};
use helpers::*;
use instruction_helpers::*;
use Instruction::*;

// Re-exports
pub use alu::{AsLargerType, HasMax, NMinus1Mask, NumBits, WrappingAdd, WrappingSub};
pub use instruction_helpers::step;

// The last nibble of some instructions reserves numbers 0-6 for the 16-bit registers, with
// codes for the 32-bit big registers starting at 7.
const NUM_REGS: u8 = 7;
// The last nibble of some instructions reserves numbers 0-2 for the 32-bit registers, with
// codes for other instructions starting at 3.
const NUM_BREGS: u8 = 3;

// ------- ADDING NEW INSTRUCTION CHECKLIST -------
// - [instruction.rs]           Add Instruction enum entry
// - [instruction_helpers.rs]   Add match to step fn
// - [instruction_helpers.rs]   Add step helper fn
// - [instruction_impl.rs]      Impl from_opcode
// - [instruction_impl.rs]      Impl into_opcode
// - [instruction_impl.rs]      Impl num_steps
// - [instruction_impl.rs]      Impl Display
// - [tests]                    Add tests
// - [instruction_parser.rs]    Add Operation if new operation name
// - [instruction_parser.rs]    Add to instr_to_bytes
// - [parser_tests.rs]          Add a to_bytes_test
// - [parser_tests.rs]          (OPTIONAL) Add a parser_test

/// Enum for accessing the different CPU instructions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Instruction {
    /// 0x0000 - NOP
    /// Do nothing for 4 cycles.
    #[default]
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
    /// Load stack pointer into the dword (little-endian) starting at address imm32.
    /// [imm32] = SP
    LdImm32Sp,
    /// 0x01Ba - LD SP,bra
    /// Load register bra into stack pointer.
    /// SP = bra
    LdSpBra(Reg32),
    /// 0x01Ca - LD bra,SP
    /// Load stack pointer into bra.
    /// bra = SP
    LdBraSp(Reg32),
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
    /// 0x057a - LDR ra,imm32
    /// Relative load - load the word pointed to by HL + (imm32 interpreted as a signed integer)
    /// into ra.
    /// ra = [HL + imm32]
    LdrRaImm32(Reg16),
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
    /// 0x097a - LDI [bra],imm16
    /// Load imm16 into [bra], then increment bra by two.
    LdiBraImm16(Reg32),
    /// 0x098a - LDD [bra],imm16
    /// Load imm16 into [bra], then decrement bra by two.
    LddBraImm16(Reg32),
    /// 0x099a - LD [imm32],ra
    /// Load ra into the memory location denoted by the 32-bit little-endian immediate value.
    LdImm32Ra(Reg16),
    /// 0x09Aa - LD ra,[imm32]
    /// Load the value pointed to by the little-endian 32-bit immediate value into ra.
    LdRaImm32(Reg16),
    /// 0x0Aab - VLD [bra],brb
    /// VRAM load. Special, faster 32-bit version of LD [bra],rb for VRAM only.
    VldBraBrb(Reg32, Reg32),
    /// 0x0Bab - VLDI [bra],brb
    /// VRAM load. Special, faster 32-bit version of LDI [bra],rb for VRAM only.
    VldiBraBrb(Reg32, Reg32),
    /// 0x0Cab - VLDD [bra],brb
    /// VRAM load. Special, faster 32-bit version of LDD [bra],rb for VRAM only.
    VlddBraBrb(Reg32, Reg32),
    /// 0x0C3a - VLD [bra],imm32
    /// VRAM load. Special, faster 32-bit version of LD [bra],imm16 for VRAM only.
    VldBraImm32(Reg32),
    /// 0x0C4a - VLDI [bra],imm32
    /// VRAM load. Special, faster 32-bit version of LDI [bra],imm16 for VRAM only.
    VldiBraImm32(Reg32),
    /// 0x0C5a - VLDD [bra],imm32
    /// VRAM load. Special, faster 32-bit version of LDD [bra],imm16 for VRAM only.
    VlddBraImm32(Reg32),
    /// 0x10ab - ADD ra,rb
    /// Add rb to ra.
    /// ra += rb
    AddRaRb(Reg16, Reg16),
    /// 0x10(a+7)(b+7) - ADD bra,brb
    /// Add brb to bra. 32-bit addition.
    /// bra += brb
    AddBraBrb(Reg32, Reg32),
    /// 0x11ab - ADD vra,vrb
    /// Add vrb to vra. 8-bit addition.
    /// vra += vrb
    AddVraVrb(Reg8, Reg8),
    /// 0x12ab - ADC ra,rb
    /// Add rb and the carry flag to ra.
    /// ra += rb + C
    AdcRaRb(Reg16, Reg16),
    /// 0x12(a+7)(b+7) - ADC bra,brb
    /// Add brb and the carry flag to bra. 32-bit addition.
    /// bra += brb + C
    AdcBraBrb(Reg32, Reg32),
    /// 0x13ab - ADC vra,vrb
    /// Add vrb and the carry flag to vra. 8-bit addition.
    /// vra += vrb + C
    AdcVraVrb(Reg8, Reg8),
    /// 0x14ab - SUB ra,rb
    /// Subtract rb from ra.
    /// ra -= rb
    SubRaRb(Reg16, Reg16),
    /// 0x14(a+7)(b+7) - SUB bra,brb
    /// Subtract brb from bra. 32-bit subtraction.
    /// bra -= brb
    SubBraBrb(Reg32, Reg32),
    /// 0x15ab - SUB vra,vrb
    /// Subtract vrb from vra. 8-bit subtraction.
    /// vra -= vrb
    SubVraVrb(Reg8, Reg8),
    /// 0x16ab - SBB ra,rb
    /// Subtract rb and the carry flag from ra.
    /// ra -= rb + C
    SbbRaRb(Reg16, Reg16),
    /// 0x16(a+7)(b+7) - SBB bra,brb
    /// Subtract brb and the carry flag from bra. 32-bit subtraction.
    /// bra -= brb + C
    SbbBraBrb(Reg32, Reg32),
    /// 0x17ab - SBB vra,vrb
    /// Subtract vrb and the carry flag from vra. 8-bit subtraction.
    /// vra -= vrb + C
    SbbVraVrb(Reg8, Reg8),
    /// 0x180a - ADD ra,imm16
    /// Add the 16-bit immediate value to ra.
    /// ra += imm16
    AddRaImm16(Reg16),
    /// 0x181a - ADC ra,imm16
    /// Add the 16-bit immediate value + the carry flag to ra.
    /// ra += imm16 + C
    AdcRaImm16(Reg16),
    /// 0x182a - ADD bra,imm32
    /// Add the 32-bit immediate value to bra.
    /// bra += imm32
    AddBraImm32(Reg32),
    /// 0x183a - ADC bra,imm32
    /// Add the 32-bit immediate value + the carry flag to bra.
    /// bra += imm32 + C
    AdcBraImm32(Reg32),
    /// 0x184a - ADD vra,imm8
    /// Add the 8-bit immediate value to vra.
    /// vra += imm8
    AddVraImm8(Reg8),
    /// 0x185a - ADC vra,imm8
    /// Add the 8-bit immediate value + the carry flag to vra.
    /// vra += imm8 + C
    AdcVraImm8(Reg8),
    /// 0x186a - SUB ra,imm16
    /// Subtract the 16-bit immediate value from ra.
    /// ra -= imm16
    SubRaImm16(Reg16),
    /// 0x187a - SBB ra,imm16
    /// Subtract the 16-bit immediate value + the carry flag from ra.
    /// ra -= imm16 + C
    SbbRaImm16(Reg16),
    /// 0x188a - SUB bra,imm32
    /// Subtract the 32-bit immediate value from bra.
    /// bra -= imm32
    SubBraImm32(Reg32),
    /// 0x189a - SBB bra, imm32
    /// Subtract the 32-bit immediate value + the carry flag from bra.
    /// bra -= imm32 + C
    SbbBraImm32(Reg32),
    /// 0x18Aa - SUB vra,imm8
    /// Subtract the 8-bit immediate value from vra.
    /// vra -= imm8
    SubVraImm8(Reg8),
    /// 0x18Ba - SBB vra,imm8
    /// Subtract the 8-bit immediate value + the carry flag from vra.
    /// vra -= imm8 + C
    SbbVraImm8(Reg8),
    /// 0x19ab - ADD ra,[brb]
    /// Add [brb] to ra.
    /// ra += [brb]
    AddRaBrb(Reg16, Reg32),
    /// 0x1Aab - ADC ra,[brb]
    /// Add [brb] + the carry flag to ra.
    /// ra += [brb] + C
    AdcRaBrb(Reg16, Reg32),
    /// 0x1Bab - SUB ra,[brb]
    /// Subtract [brb] from ra.
    /// ra -= brb
    SubRaBrb(Reg16, Reg32),
    /// 0x1Cab - SBB ra,[brb]
    /// Subtract [brb] + the carry flag from ra.
    /// ra -= [brb] + C
    SbbRaBrb(Reg16, Reg32),
    /// 0x1D0a - TCP ra
    /// Two's complement ra. Set Carry = 0 iff ra == 0.
    /// ra = -ra
    TcpRa(Reg16),
    /// 0x1D1a - TCP bra
    /// Two's complement bra. Set Carry = 0 iff bra == 0.
    /// bra = -bra
    TcpBra(Reg32),
    /// 0x1D2a - TCP vra
    /// Two's complement vra. Set Carry = 0 iff vra == 0.
    /// vra = -vra
    TcpVra(Reg8),
    /// 0x1D3a - INC ra
    /// Increment ra. Does not affect the Carry, Overflow, and Negative flags.
    /// ra += 1
    IncRa(Reg16),
    /// 0x1D4a - INC bra
    /// Increment bra. Does not affect the Carry, Overflow, and Negative flags.
    /// bra += 1
    IncBra(Reg32),
    /// 0x1D5a - INC vra
    /// Increment vra. Does not affect the Carry, Overflow, and Negative flags.
    /// vra += 1
    IncVra(Reg8),
    /// 0x1D6a - DEC ra
    /// Decrement ra. Does not affect the Carry, Overflow, and Negative flags.
    /// ra -= 1
    DecRa(Reg16),
    /// 0x1D7a - DEC bra
    /// Decrement bra. Does not affect the Carry, Overflow, and Negative flags.
    /// bra -= 1
    DecBra(Reg32),
    /// 0x1D8a - DEC vra
    /// Decrement vra. Does not affect the Carry, Overflow, and Negative flags.
    /// vra -= 1
    DecVra(Reg8),
    /// 0x1D9a - PSS ra
    /// Pass through ra. Sets the ALU flags based on ra.
    /// ra = ra
    PssRa(Reg16),
    /// 0x1DAa - PSS bra
    /// Pass through bra. Sets the ALU flags based on bra.
    /// bra = bra
    PssBra(Reg32),
    /// 0x1DBa - PSS vra
    /// Pass through vra. Sets the ALU flags based on vra.
    /// vra = vra
    PssVra(Reg8),
    /// 0x1DC0 - PSS imm16
    /// Pass through the immediate 16-bit value. Sets the ALU flags accordingly.
    PssImm16,
    /// 0x1DC1 - PSS imm32
    /// Pass through the immediate 32-bit value. Sets the ALU flags accordingly.
    PssImm32,
    /// 0x1DC2 - PSS imm8
    /// Pass through the immediate 8-bit value. Sets the ALU flags accordingly.
    PssImm8,
    /// 0x1Eab - AND ra,rb
    /// Set ra to bitwise AND of ra and rb.
    /// ra &= rb
    AndRaRb(Reg16, Reg16),
    /// 0x1Fab - AND bra,brb
    /// Set bra to bitwise AND of bra and brb.
    /// bra &= brb
    AndBraBrb(Reg32, Reg32),
    /// 0x20ab - AND vra,vrb
    /// Set vra to bitwise AND of vra and vrb.
    /// vra &= vrb
    AndVraVrb(Reg8, Reg8),
    /// 0x21ab - AND ra,[brb]
    /// Set ra to bitwise AND of ra and [brb].
    /// ra &= [brb]
    AndRaBrb(Reg16, Reg32),
    /// 0x22ab - OR ra,rb
    /// Set ra to bitwise OR of ra and rb.
    /// ra |= rb
    OrRaRb(Reg16, Reg16),
    /// 0x23ab - OR bra,brb
    /// Set bra to bitwise OR of bra and brb.
    /// bra |= brb
    OrBraBrb(Reg32, Reg32),
    /// 0x24ab - OR vra,vrb
    /// Set vra to bitwise OR of vra and vrb.
    /// vra |= vrb
    OrVraVrb(Reg8, Reg8),
    /// 0x25ab - OR ra,[brb]
    /// Set ra to bitwise OR of ra and [brb].
    /// ra |= [brb]
    OrRaBrb(Reg16, Reg32),
    /// 0x26ab - XOR ra,rb
    /// Set ra to bitwise XOR of ra and rb.
    /// ra ^= rb
    XorRaRb(Reg16, Reg16),
    /// 0x27ab - XOR bra,brb
    /// Set bra to bitwise XOR of bra and brb.
    /// bra ^= brb
    XorBraBrb(Reg32, Reg32),
    /// 0x28ab - XOR vra,vrb
    /// Set vra to bitwise XOR of vra and vrb.
    /// vra ^= vrb
    XorVraVrb(Reg8, Reg8),
    /// 0x29ab - XOR ra,[brb]
    /// Set ra to bitwise XOR of ra and [brb].
    /// ra ^= [brb]
    XorRaBrb(Reg16, Reg32),
    /// 0x2A0a - AND ra,imm16
    /// Set ra to bitwise AND of ra and 16-bit immediate value.
    /// ra &= imm16
    AndRaImm16(Reg16),
    /// 0x2A1a - AND bra,imm32
    /// Set bra to bitwise AND of bra and 32-bit immediate value.
    /// bra &= imm32
    AndBraImm32(Reg32),
    /// 0x2A2a - AND vra,imm8
    /// Set vra to bitwise AND of vra and 8-bit immediate value.
    /// vra &= imm8
    AndVraImm8(Reg8),
    /// 0x2A3a - OR ra,imm16
    /// Set ra to bitwise OR of ra and 16-bit immediate value.
    /// ra |= imm16
    OrRaImm16(Reg16),
    /// 0x2A4a - OR bra,imm32
    /// Set bra to bitwise OR of bra and 32-bit immediate value.
    /// bra |= imm32
    OrBraImm32(Reg32),
    /// 0x2A5a - OR vra,imm8
    /// Set vra to bitwise OR of vra and 8-bit immediate value.
    /// vra |= imm8
    OrVraImm8(Reg8),
    /// 0x2A6a - XOR ra,imm16
    /// Set ra to bitwise XOR of ra and 16-bit immediate value.
    /// ra ^= imm16
    XorRaImm16(Reg16),
    /// 0x2A7a - XOR bra,imm32
    /// Set bra to bitwise XOR of bra and 32-bit immediate value.
    /// bra ^= imm32
    XorBraImm32(Reg32),
    /// 0x2A8a - XOR vra,imm8
    /// Set vra to bitwise XOR of vra and 8-bit immediate value.
    /// vra ^= imm8
    XorVraImm8(Reg8),
    /// 0x2A9a - NOT ra
    /// Flip all the bits of ra.
    /// ra = !ra
    NotRa(Reg16),
    /// 0x2AAa - NOT bra
    /// Flip all the bits of bra.
    /// bra = !bra
    NotBra(Reg32),
    /// 0x2ABa - NOT vra
    /// Flip all the bits of vra.
    /// vra = !vra
    NotVra(Reg8),
    /// 0x2Bab - ASR ra,b
    /// Arithmetic shift. Shift ra right b bits, preserving the most significant bit.
    /// ra >>= b
    AsrRaB(Reg16, u8),
    /// 0x2Cab - ASR bra,b
    /// Arithmetic shift. Shift bra right b bits, preserving the most significant bit.
    /// bra >>= b
    AsrBraB(Reg32, u8),
    /// 0x2Dab - ASR vra,b
    /// Arithmetic shift. Shift vra right b bits, preserving the most significant bit.
    /// vra >>= b
    AsrVraB(Reg8, u8),
    /// 0x2Eab - ASL ra,b
    /// Arithmetic shift. Shift ra left b bits, shifting on zeroes.
    /// ra <<= b
    AslRaB(Reg16, u8),
    /// 0x2Fab - ASL bra,b
    /// Arithmetic shift. Shift bra left b bits, shifting on zeroes.
    /// ra <<= b
    AslBraB(Reg32, u8),
    /// 0x30ab - ASL vra,b
    /// Arithmetic shift. Shift vra left b bits, shifting on zeroes.
    /// ra <<= b
    AslVraB(Reg8, u8),
    /// 0x31ab - LSR ra,b
    /// Logical shift. Shift ra right b bits, shifting on zeroes.
    /// ra >>= b
    LsrRaB(Reg16, u8),
    /// 0x32ab - LSR bra,b
    /// Logical shift. Shift bra right b bits, shifting on zeroes.
    /// bra >>= b
    LsrBraB(Reg32, u8),
    /// 0x33ab - LSR vra,b
    /// Logical shift. Shift vra right b bits, shifting on zeroes.
    /// vra >>= b
    LsrVraB(Reg8, u8),
    /// 0x34ab - RTR ra,b
    /// Rotate. Rotate ra right b bits.
    RtrRaB(Reg16, u8),
    /// 0x35ab - RTR bra,b
    /// Rotate. Rotate bra right b bits.
    RtrBraB(Reg32, u8),
    /// 0x36ab - RTR vra,b
    /// Rotate. Rotate vra right b bits.
    RtrVraB(Reg8, u8),
    /// 0x37ab - RTL ra,b
    /// Rotate. Rotate ra left b bits.
    RtlRaB(Reg16, u8),
    /// 0x38ab - RTL bra,b
    /// Rotate. Rotate bra left b bits.
    RtlBraB(Reg32, u8),
    /// 0x39ab - RTL vra,b
    /// Rotate. Rotate vra left b bits.
    RtlVraB(Reg8, u8),
    /// 0x3Aab - RCR ra,b
    /// Rotate carry. Rotate ra right b bits through the carry flag.
    RcrRaB(Reg16, u8),
    /// 0x3Bab - RCR bra,b
    /// Rotate carry. Rotate bra right b bits through the carry flag.
    RcrBraB(Reg32, u8),
    /// 0x3Cab - RCR vra,b
    /// Rotate carry. Rotate vra right b bits through the carry flag.
    RcrVraB(Reg8, u8),
    /// 0x3Dab - RCL ra,b
    /// Rotate carry. Rotate ra left b bits through the carry flag.
    RclRaB(Reg16, u8),
    /// 0x3Eab - RCL bra,b
    /// Rotate carry. Rotate bra left b bits through the carry flag.
    RclBraB(Reg32, u8),
    /// 0x3Fab - RCL vra,b
    /// Rotate carry. Rotate vra left b bits through the carry flag.
    RclVraB(Reg8, u8),
    /// 0x40ab - CMP ra,rb
    /// Subtract rb from ra, setting flags accordingly. Discard the result.
    /// ra - rb
    CmpRaRb(Reg16, Reg16),
    /// 0x40(a+7)(b+7) - CMP bra,brb
    /// Subtract brb from bra, setting flags accordingly. Discard the result.
    /// bra - brb
    CmpBraBrb(Reg32, Reg32),
    /// 0x41ab - CMP vra,vrb
    /// Subtract vrb from vra, setting flags accordingly. Discard the result.
    /// vra - vrb
    CmpVraVrb(Reg8, Reg8),
    /// 0x420a - CMP ra,imm16
    /// Subtract imm16 from ra, setting flags accordingly. Discard the result.
    /// ra - imm16
    CmpRaImm16(Reg16),
    /// 0x421a - CMP bra,imm32
    /// Subtract imm32 from bra, setting flags accordingly. Discard the result.
    /// bra - imm32
    CmpBraImm32(Reg32),
    /// 0x422a - CMP vra,imm8
    /// Subtract imm8 from vra, setting flags accordingly. Discard the result.
    CmpVraImm8(Reg8),
    /// 0x423a - CMP imm16,ra
    /// Subtract ra from imm16, setting flags accordingly. Discard the result.
    /// imm16 - ra
    CmpImm16Ra(Reg16),
    /// 0x424a - CMP imm32,bra
    /// Subtract bra from imm32, setting flags accordingly. Discard the result.
    CmpImm32Bra(Reg32),
    /// 0x425a - CMP imm8,vra
    /// Subtract vra from imm8, setting flags accordingly. Discard the result.
    CmpImm8Vra(Reg8),
    /// 0x43ab - CMP ra,[brb]
    /// Subtract the value at address brb from ra, setting flags accordingly. Discard the result.
    CmpRaBrb(Reg16, Reg32),
    /// 0x44ab - CMP [bra],rb
    /// Subtract rb from the value at address bra, setting flags accordingly. Discard the result.
    CmpBraRb(Reg32, Reg16),
    /// 0x45ab - BIT ra,b
    /// Set the Zero flag to bit b of ra.
    /// Z = ra[b]
    BitRaB(Reg16, u8),
    /// 0x46ab - BIT [bra],b
    /// Set the Zero flag to bit b of the value stored at address bra.
    /// Z = [bra][b]
    BitBraB(Reg32, u8),
    /// 0x47ab - STB ra,b
    /// Set bit b of ra.
    /// ra[b] = 1
    StbRaB(Reg16, u8),
    /// 0x48ab - STB [bra],b
    /// Set bit b of the value at address bra.
    /// [bra][b] = 1
    StbBraB(Reg32, u8),
    /// 0x49ab - RSB ra,b
    /// Reset bit b of ra.
    /// ra[b] = 0
    RsbRaB(Reg16, u8),
    /// 0x4Aab - RSB [bra],b
    /// Reset bit b of the value at address bra.
    /// [bra][b] = 0
    RsbBraB(Reg32, u8),
    /// 0x4Bab - TGB ra,b
    /// Toggle bit b of ra.
    /// ra[b] = !ra[b]
    TgbRaB(Reg16, u8),
    /// 0x4Cab - TGB [bra],b
    /// Toggle bit b of the value at address bra.
    /// [bra][b] = ![bra][b]
    TgbBraB(Reg32, u8),
    /// 0x4D0a - SWP ra
    /// Swap the high and low bytes of ra.
    SwpRa(Reg16),
    /// 0x4D1a - SWP [bra]
    /// Swap the high and low bytes of the value at address bra.
    SwpBra(Reg32),
    /// 0x4D20 - SZF
    /// Set the Zero flag.
    Szf,
    /// 0x4D21 - RZF
    /// Reset the Zero flag.
    Rzf,
    /// 0x4D22 - TZF
    /// Toggle the Zero flag.
    Tzf,
    /// 0x4D23 - SCF
    /// Set the Carry flag.
    Scf,
    /// 0x4D24 - RCF
    /// Reset the Carry flag.
    Rcf,
    /// 0x4D25 - TCF
    /// Toggle the Carry flag.
    Tcf,
    /// 0x4D26 - SOF
    /// Set the Overflow flag.
    Sof,
    /// 0x4D27 - ROF
    /// Reset the Overflow flag.
    Rof,
    /// 0x4D28 - TOF
    /// Toggle the Overflow flag.
    Tof,
    /// 0x4D29 - SPF
    /// Set the Parity flag.
    Spf,
    /// 0x4D2A - RPF
    /// Reset the Parity flag.
    Rpf,
    /// 0x4D2B - TPF
    /// Toggle the Parity flag.
    Tpf,
    /// 0x4D2C - SNF
    /// Set the Negative flag.
    Snf,
    /// 0x4D2D - RNF
    /// Reset the Negative flag.
    Rnf,
    /// 0x4D2E - TNF
    /// Toggle the Negative flag.
    Tnf,
    /// 0x4D2F - SAF
    /// Set all flags.
    Saf,
    /// 0x4D30 - RAF
    /// Reset all flags.
    Raf,
    /// 0x50ab - MULU ra,rb
    /// Unsigned multiply ra and rb.
    /// ra *= rb
    MuluRaRb(Reg16, Reg16),
    /// 0x51ab - MULI ra,rb
    /// Signed multiply ra and rb.
    /// ra *= rb
    MuliRaRb(Reg16, Reg16),
    /// 0x52ab - DIVU ra,rb
    /// Unsigned divide ra and rb, storing the remainder in rb.
    /// ra /= rb; rb = ra % rb
    DivuRaRb(Reg16, Reg16),
    /// 0x53ab - DIVI ra,rb
    /// Signed divide ra and rb, storing the remainder in rb.
    /// ra /= rb; rb = ra % rb
    DiviRaRb(Reg16, Reg16),
    /// 0x50(a+7)(b+7) - MULU bra,brb
    /// Unsigned multiply bra and brb.
    /// bra *= brb
    MuluBraBrb(Reg32, Reg32),
    /// 0x51(a+7)(b+7) - MULI bra,brb
    /// Signed multiply bra and brb.
    /// bra *= brb
    MuliBraBrb(Reg32, Reg32),
    /// 0x52(a+7)(b+7) - DIVU bra,brb
    /// Unsigned divide bra and brb, storing the remainder in brb.
    /// bra /= brb; brb = bra % brb
    DivuBraBrb(Reg32, Reg32),
    /// 0x53(a+7)(b+7) - DIVI bra,brb
    /// Signed divide bra and brb, storing the remainder in brb.
    /// bra /= brb; brb = bra % brb
    DiviBraBrb(Reg32, Reg32),
    /// 0x54ab - MULU vra,vrb
    /// Unsigned multiply vra and vrb.
    /// vra *= vrb
    MuluVraVrb(Reg8, Reg8),
    /// 0x55ab - MULI vra,vrb
    /// Signed multiply vra and vrb.
    /// vra *= vrb
    MuliVraVrb(Reg8, Reg8),
    /// 0x56ab - DIVU vra,vrb
    /// Unsigned divide vra and vrb, storing the remainder in vrb.
    /// vra /= vrb; vrb = vra % vrb
    DivuVraVrb(Reg8, Reg8),
    /// 0x57ab - DIVI vra,vrb
    /// Signed divide vra and vrb, storing the remainder in vrb.
    /// vra /= vrb; vrb = vra % vrb
    DiviVraVrb(Reg8, Reg8),
    /// 0x58ab - MULU ra,[brb]
    /// Unsigned multiply ra and the value stored at brb.
    /// ra *= brb
    MuluRaBrb(Reg16, Reg32),
    /// 0x59ab - MULI ra,[brb]
    /// Signed multiply ra and the value stored at brb.
    /// ra *= brb
    MuliRaBrb(Reg16, Reg32),
    /// 0x5Aab - DIVU ra,[brb]
    /// Unsigned divide ra and the value stored at brb, storing the remainder in [brb].
    /// ra /= brb; brb = ra % brb
    DivuRaBrb(Reg16, Reg32),
    /// 0x5Bab - DIVI ra,[brb]
    /// Signed divide ra and the value stored at brb, storing the remainder in [brb].
    /// ra /= brb; brb = ra % brb
    DiviRaBrb(Reg16, Reg32),
    /// 0x5C0a - MULU ra,imm16
    /// Unsigned multiply ra and imm16.
    /// ra *= imm16
    MuluRaImm16(Reg16),
    /// 0x5C1a - MULI ra,imm16
    /// Signed multiply ra and imm16.
    /// ra *= imm16
    MuliRaImm16(Reg16),
    /// 0x5C2a - DIVU ra,imm16
    /// Unsigned divide ra and imm16, storing the remainder in A.
    /// ra /= imm16; A = ra % imm16
    DivuRaImm16(Reg16),
    /// 0x5C3a - DIVI ra,imm16
    /// Signed divide ra and imm16, storing the remainder in A.
    /// ra /= imm16; A = ra % imm16
    DiviRaImm16(Reg16),
    /// 0x5C4a - MULU bra,imm32
    /// Unsigned multiply bra and imm32.
    /// bra *= imm32
    MuluBraImm32(Reg32),
    /// 0x5C5a - MULI bra,imm32
    /// Signed multiply bra and imm32.
    /// bra *= imm32
    MuliBraImm32(Reg32),
    /// 0x5C6a - DIVU bra,imm32
    /// Unsigned divide bra and imm32, storing the remainder in BC.
    /// bra /= imm32; BC = bra % imm32
    DivuBraImm32(Reg32),
    /// 0x5C7a - DIVI bra,imm32
    /// Signed divide bra and imm32, storing the remainder in BC.
    /// bra /= imm32; BC = bra % imm32
    DiviBraImm32(Reg32),
    /// 0x5C8a - MULU vra,imm8
    /// Unsigned multiply vra and imm8.
    /// vra *= imm8
    MuluVraImm8(Reg8),
    /// 0x5C9a - MULI vra,imm8
    /// Signed multiply vra and imm8.
    /// vra *= imm8
    MuliVraImm8(Reg8),
    /// 0x5CAa - DIVU vra,imm8
    /// Unsigned divide vra and imm8, storing the remainder in A1.
    /// vra /= imm8; A1 = vra % imm8
    DivuVraImm8(Reg8),
    /// 0x5CBa - DIVI vra,imm8
    /// Signed divide vra and imm8, storing the remainder in A1.
    /// vra /= imm8; A1 = vra % imm8
    DiviVraImm8(Reg8),
    /// 0x600a - RAND ra
    /// Fill ra with a pseudorandom LFSR-based random number.
    RandRa(Reg16),
    /// 0x601a - RAND bra
    /// Fill bra with a pseudorandom LFSR-based random number.
    RandBra(Reg32),
    /// 0x602a - RAND vra
    /// Fill vra with a pseudorandom LFSR-based random number,
    RandVra(Reg8),
    /// 0x8000 - JP imm32
    /// Jump to the address stored in the immediate 32-bit value.
    JpImm32,
    /// 0x8001 - JR imm32
    /// Relative jump imm32 (interpreted as a signed integer) bytes forwards/backwards.
    JrImm32,
    /// 0x8002 - JPZ imm32
    /// Jump to the address stored in imm32 iff the Zero flag is set.
    JpzImm32,
    /// 0x8003 - JNZ imm32
    /// Jump to the address stored in imm32 iff the Zero flag is reset.
    JnzImm32,
    /// 0x8004 - JPC imm32
    /// Jump to the address stored in imm32 iff the Carry flag is set.
    JpcImm32,
    /// 0x8005 - JNC imm32
    /// Jump to the address stored in imm32 iff the Carry flag is reset.
    JncImm32,
    /// 0x8006 - JPO imm32
    /// Jump to the address stored in imm32 iff the Overflow flag is set.
    JpoImm32,
    /// 0x8007 - JNO imm32
    /// Jump to the address stored in imm32 iff the Overflow flag is reset.
    JnoImm32,
    /// 0x8008 - JPP imm32
    /// Jump to the address stored in imm32 iff the Parity flag is set.
    JppImm32,
    /// 0x8009 - JNP imm32
    /// Jump to the address stored in imm32 iff the Parity flag is reset.
    JnpImm32,
    /// 0x800A - JPN imm32
    /// Jump to the address stored in imm32 iff the Negative flag is set.
    JpnImm32,
    /// 0x800B - JNN imm32
    /// Jump to the address stored in imm32 iff the Negative flag is reset.
    JnnImm32,
    /// 0x801a - JP bra
    /// Jump to the address stored in bra.
    JpBra(Reg32),
    /// 0x802a - JR bra
    /// Relative jump bra (interpreted as a signed integer) bytes forwards/backwards.
    JrBra(Reg32),
    /// 0x803a - JPZ bra
    /// Jump to the address stored in bra iff the Zero flag is set.
    JpzBra(Reg32),
    /// 0x804a - JNZ bra
    /// Jump to the address stored in bra iff the Zero flag is reset.
    JnzBra(Reg32),
    /// 0x805a - JPC bra
    /// Jump to the address stored in bra iff the Carry flag is set.
    JpcBra(Reg32),
    /// 0x806a - JNC bra
    /// Jump to the address stored in bra iff the Carry flag is reset.
    JncBra(Reg32),
    /// 0x807a - JPO bra
    /// Jump to the address stored in bra iff the Overflow flag is set.
    JpoBra(Reg32),
    /// 0x808a - JNO bra
    /// Jump to the address stored in bra iff the Overflow flag is reset.
    JnoBra(Reg32),
    /// 0x809a - JPP bra
    /// Jump to the address stored in bra iff the Parity flag is set.
    JppBra(Reg32),
    /// 0x80Aa - JNP bra
    /// Jump to the address stored in bra iff the Parity flag is reset.
    JnpBra(Reg32),
    /// 0x80Ba - JPN bra
    /// Jump to the address stored in bra iff the Negative flag is set.
    JpnBra(Reg32),
    /// 0x80Ca - JNN bra
    /// Jump to the address stored in bra iff the Negative flag is reset.
    JnnBra(Reg32),
    /// 0x8100 - CALL imm32
    /// Push the address of the instruction after the CALL into the stack, then jump to imm32.
    CallImm32,
    /// 0x8101 - CLZ imm32
    /// Call imm32 iff the Zero flag is set.
    ClzImm32,
    /// 0x8102 - CNZ imm32
    /// Call imm32 iff the Zero flag is reset.
    CnzImm32,
    /// 0x8103 - CLC imm32
    /// Call imm32 iff the Carry flag is set.
    ClcImm32,
    /// 0x8104 - CNC imm32
    /// Call imm32 iff the Carry flag is reset.
    CncImm32,
    /// 0x8105 - CLO imm32
    /// Call imm32 iff the Overflow flag is set.
    CloImm32,
    /// 0x8106 - CNO imm32
    /// Call imm32 iff the Overflow flag is reset.
    CnoImm32,
    /// 0x8107 - CLP imm32
    /// Call imm32 iff the Parity flag is set.
    ClpImm32,
    /// 0x8108 - CNP imm32
    /// Call imm32 iff the Parity flag is reset.
    CnpImm32,
    /// 0x8109 - CLN imm32
    /// Call imm32 iff the Negative flag is set.
    ClnImm32,
    /// 0x810A - CNN imm32
    /// Call imm32 iff the Negative flag is reset.
    CnnImm32,
    /// 0x811a - CALL bra
    /// Push the address of the instruction after the CALL into the stack, then jump to bra.
    CallBra(Reg32),
    /// 0x8113 - RET
    /// Return from subroutine, setting the program counter to the value popped off the stack.
    Ret,
    /// 0x8114 - RTZ
    /// Return iff the Zero flag is set.
    Rtz,
    /// 0x8115 - RNZ
    /// Return iff the Zero flag is reset.
    Rnz,
    /// 0x8116 - RTC
    /// Return iff the Carry flag is set.
    Rtc,
    /// 0x8117 - RNC
    /// Return iff the Carry flag is reset.
    Rnc,
    /// 0x8118 - RTO
    /// Return iff the Overflow flag is set.
    Rto,
    /// 0x8119 - RNO
    /// Return iff the Overflow flag is reset.
    Rno,
    /// 0x811A - RTP
    /// Return iff the Parity flag is set.
    Rtp,
    /// 0x811B - RNP
    /// Return iff the Parity flag is reset.
    Rnp,
    /// 0x811C - RTN
    /// Return iff the Negative flag is set.
    Rtn,
    /// 0x811D - RNN
    /// Return iff the Negative flag is reset.
    Rnn,
    /// 0x811E - RETI
    /// Return from subroutine then enable interrupts.
    Reti,
    /// 0x812a - CLZ bra
    /// Call bra iff the Zero flag is set.
    ClzBra(Reg32),
    /// 0x813a - CNZ bra
    /// Call bra iff the Zero flag is reset.
    CnzBra(Reg32),
    /// 0x814a - CLC bra
    /// Call bra iff the Carry flag is set.
    ClcBra(Reg32),
    /// 0x815a - CNC bra
    /// Call bra iff the Carry flag is reset.
    CncBra(Reg32),
    /// 0x816a - CLO bra
    /// Call bra iff the Overflow flag is set.
    CloBra(Reg32),
    /// 0x817a - CNO bra
    /// Call bra iff the Overflow flag is reset.
    CnoBra(Reg32),
    /// 0x818a - CLP bra
    /// Call bra iff the Parity flag is set.
    ClpBra(Reg32),
    /// 0x819a - CNP bra
    /// Call bra iff the Parity flag is reset.
    CnpBra(Reg32),
    /// 0x81Aa - CLN bra
    /// Call bra iff the Negative flag is set.
    ClnBra(Reg32),
    /// 0x81Ba - CNN bra
    /// Call bra iff the Negative flag is reset.
    CnnBra(Reg32),
    /// 0x820a - PUSH bra
    /// Push bra to the stack.
    PushBra(Reg32),
    /// 0x820(a+3) - POP bra
    /// Pop the top of the stack into bra.
    PopBra(Reg32),
    /// 0x820(a+6) - PEEK bra
    /// Peek the top value of the stack into bra.
    PeekBra(Reg32),
    /// 0x8209 - PUSH imm32
    /// Push imm32 to the stack.
    PushImm32,
    /// 0xFFFB - CLV
    /// Clear VRAM. Resets all bits in VRAM to 0.
    Clv,
    /// 0xFFFC - STOP
    /// Irreversibly stop the CPU. Essentially a power-off message.
    Stop,
    /// 0xFFFD - EI
    /// Enable interrupts.
    Ei,
    /// 0xFFFE - DI
    /// Disable interrupts.
    Di,
    /// 0xFFFF - HALT
    /// Halt the CPU, stopping cycles until an external interrupt is received.
    Halt,
    // TODO
    // Push/pop ra,vra onto/from stack
    // Push/pop BC,DE,HL onto/from stack
    // Shift/test bit based on register
    // Absolute value
    // Min/Max
    // Get clock count
    // Set FPS
}

#[cfg(test)]
mod instruction_tests;
