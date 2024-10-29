//! The CPU instruction set.
//! rn = register n
//! vrn = 8-bit virtual half-register n
//! brn = 32-bit big register n
//! imm{n} = n-bit immediate value
use std::fmt::Display;

#[cfg(test)]
use strum_macros::EnumIter;

mod alu;
mod helpers;
mod instruction_helpers;
mod instruction_impl;

use super::{Cpu, Ram};
use crate::{Reg16, Reg32, Reg8};
use alu::{AluOp::*, *};
use helpers::*;
use instruction_helpers::*;
use Instruction::*;

// Re-exports
pub use alu::{AsLargerType, HasMax, NMinus1Mask, NumBits, WrappingAdd, WrappingSub};
pub use instruction_helpers::step;

// The last nibble of some instructions reserve numbers 0-6 for the 16-bit registers, with
// codes for the 32-bit big registers starting at 7.
const NUM_REGS: u8 = 7;

/// Enum for accessing the different CPU instructions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
    // /// 0x40ab - CMP ra,rb
    // /// Subtract ra from rb, setting flags accordingly. Discard the result.
    // /// ra - rb
    // CmpRaRb(Reg16, Reg16),
    // /// 0x40(a+7)(b+7) - CMP bra,brb
    // /// Subtract bra from brb, setting flags accordingly. Discard the result.
    // /// bra - brb
    // CmpBraBrb(Reg32, Reg32),
    // /// 0x41ab - CMP vra,vrb
    // /// Subtract vra from vrb, setting flags accordingly. Discard the result.
    // /// vra - vrb
    // CmpVraVrb(Reg8, Reg8),
    // TODO
    // Read/write the program counter from/to a register.
    // Read/write the state of a flag from/to a register.
}

#[cfg(test)]
mod instruction_tests;
