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

use super::{Cpu, Ram};
use crate::{Reg16, Reg32, Reg8};
use alu::{AluOp::*, *};
use helpers::*;
use Instruction::*;

// Re-exports
pub use alu::{AsLargerType, HasMax, NMinus1Mask, NumBits, WrappingAdd, WrappingSub};

// The last nibble of some instructions reserve numbers 0-6 for the 16-bit registers, with
// codes for the 32-bit big registers starting at 7.
const NUM_REGS: u8 = 7;

/// Enum for accessing the different CPU instructions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(test, derive(EnumIter))]
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
    /// Load stack pointer into the dword (little-endian) starting at address imm32.
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
    // TODO
    // Read/write the program counter from/to a register.
    // Read/write the state of a flag from/to a register.
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
            (0x0, 0x1, 0xA, 0x0) => LdSpImm32,
            (0x0, 0x1, 0xA, 0x1) => LdImm32Sp,
            (0x0, 0x1, 0xB, bra) => LdSpBra(Reg32::from_nib(bra)),
            (0x0, 0x1, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                LdRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x0, 0x1, bra, brb) => LdBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
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
            (0x1, 0x0, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                AddRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x0, bra, brb) => AddBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x1, vra, vrb) => AddVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x2, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                AdcRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x2, bra, brb) => AdcBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x3, vra, vrb) => AdcVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x4, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                SubRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x4, bra, brb) => SubBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x5, vra, vrb) => SubVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x6, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                SbbRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x6, bra, brb) => SbbBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x7, vra, vrb) => SbbVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x8, 0x0, ra) => AddRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x1, ra) => AdcRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x2, bra) => AddBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0x3, bra) => AdcBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0x4, vra) => AddVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x8, 0x5, vra) => AdcVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x8, 0x6, ra) => SubRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x7, ra) => SbbRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x8, bra) => SubBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0x9, bra) => SbbBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0xA, vra) => SubVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x8, 0xB, vra) => SbbVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x9, ra, brb) => AddRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xA, ra, brb) => AdcRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xB, ra, brb) => SubRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xC, ra, brb) => SbbRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xD, 0x0, ra) => TcpRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0x1, bra) => TcpBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0x2, vra) => TcpVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0x3, ra) => IncRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0x4, bra) => IncBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0x5, vra) => IncVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0x6, ra) => DecRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0x7, bra) => DecBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0x8, vra) => DecVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0x9, ra) => PssRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0xA, bra) => PssBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0xB, vra) => PssVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0xC, 0x0) => PssImm16,
            (0x1, 0xD, 0xC, 0x1) => PssImm32,
            (0x1, 0xD, 0xC, 0x2) => PssImm8,
            (0x1, 0xE, ra, rb) => AndRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb)),
            (0x1, 0xF, bra, brb) => AndBraBrb(Reg32::from_nib(bra), Reg32::from_nib(brb)),
            (0x2, 0x0, vra, vrb) => AndVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x2, 0x1, ra, brb) => AndRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x2, 0x2, ra, rb) => OrRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb)),
            (0x2, 0x3, bra, brb) => OrBraBrb(Reg32::from_nib(bra), Reg32::from_nib(brb)),
            (0x2, 0x4, vra, vrb) => OrVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x2, 0x5, ra, brb) => OrRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x2, 0x6, ra, rb) => XorRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb)),
            (0x2, 0x7, bra, brb) => XorBraBrb(Reg32::from_nib(bra), Reg32::from_nib(brb)),
            (0x2, 0x8, vra, vrb) => XorVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x2, 0x9, ra, brb) => XorRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x2, 0xA, 0x0, ra) => AndRaImm16(Reg16::from_nib(ra)),
            (0x2, 0xA, 0x1, bra) => AndBraImm32(Reg32::from_nib(bra)),
            (0x2, 0xA, 0x2, vra) => AndVraImm8(Reg8::from_nib(vra)),
            (0x2, 0xA, 0x3, ra) => OrRaImm16(Reg16::from_nib(ra)),
            (0x2, 0xA, 0x4, bra) => OrBraImm32(Reg32::from_nib(bra)),
            (0x2, 0xA, 0x5, vra) => OrVraImm8(Reg8::from_nib(vra)),
            (0x2, 0xA, 0x6, ra) => XorRaImm16(Reg16::from_nib(ra)),
            (0x2, 0xA, 0x7, bra) => XorBraImm32(Reg32::from_nib(bra)),
            (0x2, 0xA, 0x8, vra) => XorVraImm8(Reg8::from_nib(vra)),
            (0x2, 0xA, 0x9, ra) => NotRa(Reg16::from_nib(ra)),
            (0x2, 0xA, 0xA, bra) => NotBra(Reg32::from_nib(bra)),
            (0x2, 0xA, 0xB, vra) => NotVra(Reg8::from_nib(vra)),
            (0x2, 0xB, ra, b) => AsrRaB(Reg16::from_nib(ra), b),
            (0x2, 0xC, bra, b) => AsrBraB(Reg32::from_nib(bra), b),
            (0x2, 0xD, vra, b) => AsrVraB(Reg8::from_nib(vra), b),
            (0x2, 0xE, ra, b) => AslRaB(Reg16::from_nib(ra), b),
            (0x2, 0xF, bra, b) => AslBraB(Reg32::from_nib(bra), b),
            (0x3, 0x0, vra, b) => AslVraB(Reg8::from_nib(vra), b),
            (0x3, 0x1, ra, b) => LsrRaB(Reg16::from_nib(ra), b),
            (0x3, 0x2, bra, b) => LsrBraB(Reg32::from_nib(bra), b),
            (0x3, 0x3, vra, b) => LsrVraB(Reg8::from_nib(vra), b),
            (0x3, 0x4, ra, b) => RtrRaB(Reg16::from_nib(ra), b),
            (0x3, 0x5, bra, b) => RtrBraB(Reg32::from_nib(bra), b),
            (0x3, 0x6, vra, b) => RtrVraB(Reg8::from_nib(vra), b),
            (0x3, 0x7, ra, b) => RtlRaB(Reg16::from_nib(ra), b),
            (0x3, 0x8, bra, b) => RtlBraB(Reg32::from_nib(bra), b),
            (0x3, 0x9, vra, b) => RtlVraB(Reg8::from_nib(vra), b),
            (0x3, 0xA, ra, b) => RcrRaB(Reg16::from_nib(ra), b),
            (0x3, 0xB, bra, b) => RcrBraB(Reg32::from_nib(bra), b),
            (0x3, 0xC, vra, b) => RcrVraB(Reg8::from_nib(vra), b),
            (0x3, 0xD, ra, b) => RclRaB(Reg16::from_nib(ra), b),
            (0x3, 0xE, bra, b) => RclBraB(Reg32::from_nib(bra), b),
            (0x3, 0xF, vra, b) => RclVraB(Reg8::from_nib(vra), b),
            _ => panic!("Opcode {:#04X} has no corresponding instruction.", opcode),
        }
    }

    /// Convert the [Instruction] into its opcode.
    pub fn into_opcode(&self) -> u16 {
        match self {
            Nop => 0x0000,
            LdRaRb(ra, rb) => opc_2arg(0x01_u16, *ra, *rb),
            LdBraBrb(bra, brb) => opc_2arg_off(0x01_u16, *bra, *brb, NUM_REGS as u16),
            LdSpImm32 => 0x01A0,
            LdImm32Sp => 0x01A1,
            LdSpBra(bra) => opc_1arg(0x01B_u16, *bra),
            LdVraVrb(vra, vrb) => opc_2arg(0x02_u16, *vra, *vrb),
            LdRaImm16(ra) => opc_1arg(0x030_u16, *ra),
            LdBraImm32(bra) => opc_1arg(0x031_u16, *bra),
            LdVraImm8(vra) => opc_1arg(0x032_u16, *vra),
            LdBraImm16(bra) => opc_1arg(0x033_u16, *bra),
            LdBraRb(bra, rb) => opc_2arg(0x04_u16, *bra, *rb),
            LdRaBrb(ra, brb) => opc_2arg(0x05_u16, *ra, *brb),
            LdiBraRb(bra, rb) => opc_2arg(0x06_u16, *bra, *rb),
            LddBraRb(bra, rb) => opc_2arg(0x07_u16, *bra, *rb),
            LdiRaBrb(ra, brb) => opc_2arg(0x08_u16, *ra, *brb),
            LddRaBrb(ra, brb) => opc_2arg(0x09_u16, *ra, *brb),
            AddRaRb(ra, rb) => opc_2arg(0x10_u16, *ra, *rb),
            AddBraBrb(bra, brb) => opc_2arg_off(0x10_u16, *bra, *brb, NUM_REGS as u16),
            AddVraVrb(vra, vrb) => opc_2arg(0x11_u16, *vra, *vrb),
            AdcRaRb(ra, rb) => opc_2arg(0x12_u16, *ra, *rb),
            AdcBraBrb(bra, brb) => opc_2arg_off(0x12_u16, *bra, *brb, NUM_REGS as u16),
            AdcVraVrb(vra, vrb) => opc_2arg(0x13_u16, *vra, *vrb),
            SubRaRb(ra, rb) => opc_2arg(0x14_u16, *ra, *rb),
            SubBraBrb(bra, brb) => opc_2arg_off(0x14_u16, *bra, *brb, NUM_REGS as u16),
            SubVraVrb(vra, vrb) => opc_2arg(0x15_u16, *vra, *vrb),
            SbbRaRb(ra, rb) => opc_2arg(0x16_u16, *ra, *rb),
            SbbBraBrb(bra, brb) => opc_2arg_off(0x16_u16, *bra, *brb, NUM_REGS as u16),
            SbbVraVrb(vra, vrb) => opc_2arg(0x17_u16, *vra, *vrb),
            AddRaImm16(ra) => opc_1arg(0x180_u16, *ra),
            AdcRaImm16(ra) => opc_1arg(0x181_u16, *ra),
            AddBraImm32(bra) => opc_1arg(0x182_u16, *bra),
            AdcBraImm32(bra) => opc_1arg(0x183_u16, *bra),
            AddVraImm8(vra) => opc_1arg(0x184_u16, *vra),
            AdcVraImm8(vra) => opc_1arg(0x185_u16, *vra),
            SubRaImm16(ra) => opc_1arg(0x186_u16, *ra),
            SbbRaImm16(ra) => opc_1arg(0x187_u16, *ra),
            SubBraImm32(bra) => opc_1arg(0x188_u16, *bra),
            SbbBraImm32(bra) => opc_1arg(0x189_u16, *bra),
            SubVraImm8(vra) => opc_1arg(0x18A_u16, *vra),
            SbbVraImm8(vra) => opc_1arg(0x18B_u16, *vra),
            AddRaBrb(ra, brb) => opc_2arg(0x19_u16, *ra, *brb),
            AdcRaBrb(ra, brb) => opc_2arg(0x1A_u16, *ra, *brb),
            SubRaBrb(ra, brb) => opc_2arg(0x1B_u16, *ra, *brb),
            SbbRaBrb(ra, brb) => opc_2arg(0x1C_u16, *ra, *brb),
            TcpRa(ra) => opc_1arg(0x1D0_u16, *ra),
            TcpBra(bra) => opc_1arg(0x1D1_u16, *bra),
            TcpVra(vra) => opc_1arg(0x1D2_u16, *vra),
            IncRa(ra) => opc_1arg(0x1D3_u16, *ra),
            IncBra(bra) => opc_1arg(0x1D4_u16, *bra),
            IncVra(vra) => opc_1arg(0x1D5_u16, *vra),
            DecRa(ra) => opc_1arg(0x1D6_u16, *ra),
            DecBra(bra) => opc_1arg(0x1D7_u16, *bra),
            DecVra(vra) => opc_1arg(0x1D8_u16, *vra),
            PssRa(ra) => opc_1arg(0x1D9_u16, *ra),
            PssBra(bra) => opc_1arg(0x1DA_u16, *bra),
            PssVra(vra) => opc_1arg(0x1DB_u16, *vra),
            PssImm16 => 0x1DC0,
            PssImm32 => 0x1DC1,
            PssImm8 => 0x1DC2,
            AndRaRb(ra, rb) => opc_2arg(0x1E_u16, *ra, *rb),
            AndBraBrb(bra, brb) => opc_2arg(0x1F_u16, *bra, *brb),
            AndVraVrb(vra, vrb) => opc_2arg(0x20_u16, *vra, *vrb),
            AndRaBrb(ra, brb) => opc_2arg(0x21_u16, *ra, *brb),
            OrRaRb(ra, rb) => opc_2arg(0x22_u16, *ra, *rb),
            OrBraBrb(bra, brb) => opc_2arg(0x23_u16, *bra, *brb),
            OrVraVrb(vra, vrb) => opc_2arg(0x24_u16, *vra, *vrb),
            OrRaBrb(ra, brb) => opc_2arg(0x25_u16, *ra, *brb),
            XorRaRb(ra, rb) => opc_2arg(0x26_u16, *ra, *rb),
            XorBraBrb(bra, brb) => opc_2arg(0x27_u16, *bra, *brb),
            XorVraVrb(vra, vrb) => opc_2arg(0x28_u16, *vra, *vrb),
            XorRaBrb(ra, brb) => opc_2arg(0x29_u16, *ra, *brb),
            AndRaImm16(ra) => opc_1arg(0x2A0_u16, *ra),
            AndBraImm32(bra) => opc_1arg(0x2A1_u16, *bra),
            AndVraImm8(vra) => opc_1arg(0x2A2_u16, *vra),
            OrRaImm16(ra) => opc_1arg(0x2A3_u16, *ra),
            OrBraImm32(bra) => opc_1arg(0x2A4_u16, *bra),
            OrVraImm8(vra) => opc_1arg(0x2A5_u16, *vra),
            XorRaImm16(ra) => opc_1arg(0x2A6_u16, *ra),
            XorBraImm32(bra) => opc_1arg(0x2A7_u16, *bra),
            XorVraImm8(vra) => opc_1arg(0x2A8_u16, *vra),
            NotRa(ra) => opc_1arg(0x2A9_u16, *ra),
            NotBra(bra) => opc_1arg(0x2AA_u16, *bra),
            NotVra(vra) => opc_1arg(0x2AB_u16, *vra),
            AsrRaB(ra, b) => opc_2arg(0x2B_u16, *ra, *b),
            AsrBraB(bra, b) => opc_2arg(0x2C_u16, *bra, *b),
            AsrVraB(vra, b) => opc_2arg(0x2D_u16, *vra, *b),
            AslRaB(ra, b) => opc_2arg(0x2E_u16, *ra, *b),
            AslBraB(bra, b) => opc_2arg(0x2F_u16, *bra, *b),
            AslVraB(vra, b) => opc_2arg(0x30_u16, *vra, *b),
            LsrRaB(ra, b) => opc_2arg(0x31_u16, *ra, *b),
            LsrBraB(bra, b) => opc_2arg(0x32_u16, *bra, *b),
            LsrVraB(vra, b) => opc_2arg(0x33_u16, *vra, *b),
            RtrRaB(ra, b) => opc_2arg(0x34_u16, *ra, *b),
            RtrBraB(bra, b) => opc_2arg(0x35_u16, *bra, *b),
            RtrVraB(vra, b) => opc_2arg(0x36_u16, *vra, *b),
            RtlRaB(ra, b) => opc_2arg(0x37_u16, *ra, *b),
            RtlBraB(bra, b) => opc_2arg(0x38_u16, *bra, *b),
            RtlVraB(vra, b) => opc_2arg(0x39_u16, *vra, *b),
            RcrRaB(ra, b) => opc_2arg(0x3A_u16, *ra, *b),
            RcrBraB(bra, b) => opc_2arg(0x3B_u16, *bra, *b),
            RcrVraB(vra, b) => opc_2arg(0x3C_u16, *vra, *b),
            RclRaB(ra, b) => opc_2arg(0x3D_u16, *ra, *b),
            RclBraB(bra, b) => opc_2arg(0x3E_u16, *bra, *b),
            RclVraB(vra, b) => opc_2arg(0x3F_u16, *vra, *b),
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
            AdcRaRb(..) => 2,
            AdcBraBrb(..) => 2,
            AdcVraVrb(..) => 2,
            SubRaRb(..) => 2,
            SubVraVrb(..) => 2,
            SubBraBrb(..) => 2,
            SbbRaRb(..) => 2,
            SbbBraBrb(..) => 2,
            SbbVraVrb(..) => 2,
            AddRaImm16(..) => 3,
            AdcRaImm16(..) => 3,
            AddBraImm32(..) => 4,
            AdcBraImm32(..) => 4,
            AddVraImm8(..) => 3,
            AdcVraImm8(..) => 3,
            SubRaImm16(..) => 3,
            SbbRaImm16(..) => 3,
            SubBraImm32(..) => 4,
            SbbBraImm32(..) => 4,
            SubVraImm8(..) => 3,
            SbbVraImm8(..) => 3,
            AddRaBrb(..) => 3,
            AdcRaBrb(..) => 3,
            SubRaBrb(..) => 3,
            SbbRaBrb(..) => 3,
            TcpRa(..) => 2,
            TcpBra(..) => 2,
            TcpVra(..) => 2,
            IncRa(..) => 2,
            IncBra(..) => 2,
            IncVra(..) => 2,
            DecRa(..) => 2,
            DecBra(..) => 2,
            DecVra(..) => 2,
            PssRa(..) => 2,
            PssBra(..) => 2,
            PssVra(..) => 2,
            PssImm16 => 3,
            PssImm32 => 4,
            PssImm8 => 3,
            AndRaRb(..) => 2,
            AndBraBrb(..) => 2,
            AndVraVrb(..) => 2,
            AndRaBrb(..) => 3,
            OrRaRb(..) => 2,
            OrBraBrb(..) => 2,
            OrVraVrb(..) => 2,
            OrRaBrb(..) => 3,
            XorRaRb(..) => 2,
            XorBraBrb(..) => 2,
            XorVraVrb(..) => 2,
            XorRaBrb(..) => 3,
            AndRaImm16(..) => 3,
            AndBraImm32(..) => 4,
            AndVraImm8(..) => 3,
            OrRaImm16(..) => 3,
            OrBraImm32(..) => 4,
            OrVraImm8(..) => 3,
            XorRaImm16(..) => 3,
            XorBraImm32(..) => 4,
            XorVraImm8(..) => 3,
            NotRa(..) => 2,
            NotBra(..) => 2,
            NotVra(..) => 2,
            AsrRaB(..) => 2,
            AsrBraB(..) => 2,
            AsrVraB(..) => 2,
            AslRaB(..) => 2,
            AslBraB(..) => 2,
            AslVraB(..) => 2,
            LsrRaB(..) => 2,
            LsrBraB(..) => 2,
            LsrVraB(..) => 2,
            RtrRaB(..) => 2,
            RtrBraB(..) => 2,
            RtrVraB(..) => 2,
            RtlRaB(..) => 2,
            RtlBraB(..) => 2,
            RtlVraB(..) => 2,
            RcrRaB(..) => 2,
            RcrBraB(..) => 2,
            RcrVraB(..) => 2,
            RclRaB(..) => 2,
            RclBraB(..) => 2,
            RclVraB(..) => 2,
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
                AdcRaRb(ra, rb) => format!("ADC {ra},{rb}"),
                AdcBraBrb(bra, brb) => format!("ADC {bra},{brb}"),
                AdcVraVrb(vra, vrb) => format!("ADC {vra},{vrb}"),
                SubRaRb(ra, rb) => format!("SUB {ra},{rb}"),
                SubBraBrb(bra, brb) => format!("SUB {bra},{brb}"),
                SubVraVrb(vra, vrb) => format!("SUB {vra},{vrb}"),
                SbbRaRb(ra, rb) => format!("SBB {ra},{rb}"),
                SbbBraBrb(bra, brb) => format!("SBB {bra},{brb}"),
                SbbVraVrb(vra, vrb) => format!("SBB {vra},{vrb}"),
                AddRaImm16(ra) => format!("ADD {ra},imm16"),
                AdcRaImm16(ra) => format!("ADC {ra},imm16"),
                AddBraImm32(bra) => format!("ADD {bra},imm32"),
                AdcBraImm32(bra) => format!("ADC {bra},imm32"),
                AddVraImm8(vra) => format!("ADD {vra},imm8"),
                AdcVraImm8(vra) => format!("ADC {vra},imm8"),
                SubRaImm16(ra) => format!("SUB {ra},imm16"),
                SbbRaImm16(ra) => format!("SBB {ra},imm16"),
                SubBraImm32(bra) => format!("SUB {bra},imm32"),
                SbbBraImm32(bra) => format!("SBB {bra},imm32"),
                SubVraImm8(vra) => format!("SUB {vra},imm8"),
                SbbVraImm8(vra) => format!("SBB {vra},imm8"),
                AddRaBrb(ra, brb) => format!("ADD {ra}[{brb}]"),
                AdcRaBrb(ra, brb) => format!("ADC {ra}[{brb}]"),
                SubRaBrb(ra, brb) => format!("SUB {ra}[{brb}]"),
                SbbRaBrb(ra, brb) => format!("SBB {ra}[{brb}]"),
                TcpRa(ra) => format!("TCP {ra}"),
                TcpBra(bra) => format!("TCP {bra}"),
                TcpVra(vra) => format!("TCP {vra}"),
                IncRa(ra) => format!("INC {ra}"),
                IncBra(bra) => format!("INC {bra}"),
                IncVra(vra) => format!("INC {vra}"),
                DecRa(ra) => format!("DEC {ra}"),
                DecBra(bra) => format!("DEC {bra}"),
                DecVra(vra) => format!("DEC {vra}"),
                PssRa(ra) => format!("PSS {ra}"),
                PssBra(bra) => format!("PSS {bra}"),
                PssVra(vra) => format!("PSS {vra}"),
                PssImm16 => String::from("PSS imm16"),
                PssImm32 => String::from("PSS imm32"),
                PssImm8 => String::from("PSS imm8"),
                AndRaRb(ra, rb) => format!("AND {ra},{rb}"),
                AndBraBrb(bra, brb) => format!("AND {bra},{brb}"),
                AndVraVrb(vra, vrb) => format!("AND {vra},{vrb}"),
                AndRaBrb(ra, brb) => format!("AND {ra},[{brb}]"),
                OrRaRb(ra, rb) => format!("OR {ra},{rb}"),
                OrBraBrb(bra, brb) => format!("OR {bra},{brb}"),
                OrVraVrb(vra, vrb) => format!("OR {vra},{vrb}"),
                OrRaBrb(ra, brb) => format!("OR {ra},[{brb}]"),
                XorRaRb(ra, rb) => format!("XOR {ra},{rb}"),
                XorBraBrb(bra, brb) => format!("XOR {bra},{brb}"),
                XorVraVrb(vra, vrb) => format!("XOR {vra},{vrb}"),
                XorRaBrb(ra, brb) => format!("XOR {ra},[{brb}]"),
                AndRaImm16(ra) => format!("AND {ra},imm16"),
                AndBraImm32(bra) => format!("AND {bra},imm32"),
                AndVraImm8(vra) => format!("AND {vra},imm8"),
                OrRaImm16(ra) => format!("OR {ra},imm16"),
                OrBraImm32(bra) => format!("OR {bra},imm32"),
                OrVraImm8(vra) => format!("OR {vra},imm8"),
                XorRaImm16(ra) => format!("XOR {ra},imm16"),
                XorBraImm32(bra) => format!("XOR {bra},imm32"),
                XorVraImm8(vra) => format!("XOR {vra},imm8"),
                NotRa(ra) => format!("NOT {ra}"),
                NotBra(bra) => format!("NOT {bra}"),
                NotVra(vra) => format!("NOT {vra}"),
                AsrRaB(ra, b) => format!("ASR {ra},{b}"),
                AsrBraB(bra, b) => format!("ASR {bra},{b}"),
                AsrVraB(vra, b) => format!("ASR {vra},{b}"),
                AslRaB(ra, b) => format!("ASL {ra},{b}"),
                AslBraB(bra, b) => format!("ASL {bra},{b}"),
                AslVraB(vra, b) => format!("ASL {vra},{b}"),
                LsrRaB(ra, b) => format!("LSR {ra},{b}"),
                LsrBraB(bra, b) => format!("LSR {bra},{b}"),
                LsrVraB(vra, b) => format!("LSR {vra},{b}"),
                RtrRaB(ra, b) => format!("RTR {ra},{b}"),
                RtrBraB(bra, b) => format!("RTR {bra},{b}"),
                RtrVraB(vra, b) => format!("RTR {vra},{b}"),
                RtlRaB(ra, b) => format!("RTL {ra},{b}"),
                RtlBraB(bra, b) => format!("RTL {bra},{b}"),
                RtlVraB(vra, b) => format!("RTL {vra},{b}"),
                RcrRaB(ra, b) => format!("RCR {ra},{b}"),
                RcrBraB(bra, b) => format!("RCR {bra},{b}"),
                RcrVraB(vra, b) => format!("RCR {vra},{b}"),
                RclRaB(ra, b) => format!("RCL {ra},{b}"),
                RclBraB(bra, b) => format!("RCL {bra},{b}"),
                RclVraB(vra, b) => format!("RCL {vra},{b}"),
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
        AddRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Add),
        AddBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Add),
        AddVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Add),
        AdcRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Adc),
        AdcBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Adc),
        AdcVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Adc),
        SubRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Sub),
        SubBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Sub),
        SubVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Sub),
        SbbRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Sbb),
        SbbBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Sbb),
        SbbVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Sbb),
        AddRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, Add),
        AdcRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, Adc),
        AddBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, Add),
        AdcBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, Adc),
        AddVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, Add),
        AdcVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, Adc),
        SubRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, Sub),
        SbbRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, Sbb),
        SubBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, Sub),
        SbbBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, Sbb),
        SubVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, Sub),
        SbbVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, Sbb),
        AddRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, Add),
        AdcRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, Adc),
        SubRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, Sub),
        SbbRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, Sbb),
        TcpRa(ra) => alu_ra_rb(cpu, ra, ra, Tcp),
        TcpBra(bra) => alu_bra_brb(cpu, bra, bra, Tcp),
        TcpVra(vra) => alu_vra_vrb(cpu, vra, vra, Tcp),
        IncRa(ra) => alu_ra_rb(cpu, ra, ra, Inc),
        IncBra(bra) => alu_bra_brb(cpu, bra, bra, Inc),
        IncVra(vra) => alu_vra_vrb(cpu, vra, vra, Inc),
        DecRa(ra) => alu_ra_rb(cpu, ra, ra, Dec),
        DecBra(bra) => alu_bra_brb(cpu, bra, bra, Dec),
        DecVra(vra) => alu_vra_vrb(cpu, vra, vra, Dec),
        PssRa(ra) => pss_ra(cpu, ra),
        PssBra(bra) => pss_bra(cpu, bra),
        PssVra(vra) => pss_vra(cpu, vra),
        PssImm16 => pss_imm16(cpu, ram),
        PssImm32 => pss_imm32(cpu, ram),
        PssImm8 => pss_imm8(cpu, ram),
        AndRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, And),
        AndBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, And),
        AndVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, And),
        AndRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, And),
        OrRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Or),
        OrBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Or),
        OrVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Or),
        OrRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, Or),
        XorRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Xor),
        XorBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Xor),
        XorVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Xor),
        XorRaBrb(ra, brb) => alu_ra_brb(cpu, ram, ra, brb, Xor),
        AndRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, And),
        AndBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, And),
        AndVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, And),
        OrRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, Or),
        OrBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, Or),
        OrVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, Or),
        XorRaImm16(ra) => alu_ra_imm16(cpu, ram, ra, Xor),
        XorBraImm32(bra) => alu_bra_imm32(cpu, ram, bra, Xor),
        XorVraImm8(vra) => alu_vra_imm8(cpu, ram, vra, Xor),
        NotRa(ra) => alu_ra_rb(cpu, ra, ra, Not),
        NotBra(bra) => alu_bra_brb(cpu, bra, bra, Not),
        NotVra(vra) => alu_vra_vrb(cpu, vra, vra, Not),
        AsrRaB(ra, b) => alu_ra_b(cpu, ra, b, Asr),
        AsrBraB(bra, b) => alu_bra_b(cpu, bra, b, Asr),
        AsrVraB(vra, b) => alu_vra_b(cpu, vra, b, Asr),
        AslRaB(ra, b) => alu_ra_b(cpu, ra, b, Asl),
        AslBraB(bra, b) => alu_bra_b(cpu, bra, b, Asl),
        AslVraB(vra, b) => alu_vra_b(cpu, vra, b, Asl),
        LsrRaB(ra, b) => alu_ra_b(cpu, ra, b, Lsr),
        LsrBraB(bra, b) => alu_bra_b(cpu, bra, b, Lsr),
        LsrVraB(vra, b) => alu_vra_b(cpu, vra, b, Lsr),
        RtrRaB(ra, b) => alu_ra_b(cpu, ra, b, Rtr),
        RtrBraB(bra, b) => alu_bra_b(cpu, bra, b, Rtr),
        RtrVraB(vra, b) => alu_vra_b(cpu, vra, b, Rtr),
        RtlRaB(ra, b) => alu_ra_b(cpu, ra, b, Rtl),
        RtlBraB(bra, b) => alu_bra_b(cpu, bra, b, Rtl),
        RtlVraB(vra, b) => alu_vra_b(cpu, vra, b, Rtl),
        RcrRaB(ra, b) => alu_ra_b(cpu, ra, b, Rcr),
        RcrBraB(bra, b) => alu_bra_b(cpu, bra, b, Rcr),
        RcrVraB(vra, b) => alu_vra_b(cpu, vra, b, Rcr),
        RclRaB(ra, b) => alu_ra_b(cpu, ra, b, Rcl),
        RclBraB(bra, b) => alu_bra_b(cpu, bra, b, Rcl),
        RclVraB(vra, b) => alu_vra_b(cpu, vra, b, Rcl),
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

fn opc_1arg<T: Into<u16>, U: Into<u16>>(instr: T, a: U) -> u16 {
    (instr.into() << 4) | a.into()
}

fn opc_2arg<T: Into<u16>, U: Into<u16>, V: Into<u16>>(instr: T, a: U, b: V) -> u16 {
    (instr.into() << 8) | (a.into() << 4) | b.into()
}

fn opc_2arg_off<T: Into<u16>, U: Into<u16>, V: Into<u16>>(
    instr: T,
    a: U,
    b: V,
    offset: u16,
) -> u16 {
    opc_2arg(instr, a.into() + offset, b.into() + offset)
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
        1 => cpu.read_word_at_addr(ram, cpu.breg(brb)),
        2 => cpu.set_reg(ra, cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldi_bra_rb(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => {
            ram.write_word(cpu.breg(bra), cpu.last_word);
            dbl_inc_addr(cpu, bra);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldd_bra_rb(cpu: &mut Cpu, ram: &mut Ram, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => {
            ram.write_word(cpu.breg(bra), cpu.last_word);
            dbl_dec_addr(cpu, bra);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldi_ra_brb(cpu: &mut Cpu, ram: &mut Ram, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(ram, cpu.breg(brb)),
        2 => {
            cpu.set_reg(ra, cpu.last_word);
            dbl_inc_addr(cpu, brb);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldd_ra_brb(cpu: &mut Cpu, ram: &mut Ram, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(ram, cpu.breg(brb)),
        2 => {
            cpu.set_reg(ra, cpu.last_word);
            dbl_dec_addr(cpu, brb);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_ra_rb(cpu: &mut Cpu, ra: Reg16, rb: Reg16, operation: AluOp) {
    match cpu.step_num {
        1 => {
            let a = cpu.reg(ra);
            let b = cpu.reg(rb);
            let result = alu(cpu, operation, a, b);
            cpu.set_reg(ra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_bra_brb(cpu: &mut Cpu, bra: Reg32, brb: Reg32, operation: AluOp) {
    match cpu.step_num {
        1 => {
            let a = cpu.breg(bra);
            let b = cpu.breg(brb);
            let result = alu(cpu, operation, a, b);
            cpu.set_breg(bra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_vra_vrb(cpu: &mut Cpu, vra: Reg8, vrb: Reg8, operation: AluOp) {
    match cpu.step_num {
        1 => {
            let a = cpu.vreg(vra);
            let b = cpu.vreg(vrb);
            let result = alu(cpu, operation, a, b);
            cpu.set_vreg(vra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_ra_imm16(cpu: &mut Cpu, ram: &Ram, ra: Reg16, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => {
            let a = cpu.reg(ra);
            let b = cpu.last_word;
            let result = alu(cpu, operation, a, b);
            cpu.set_reg(ra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_bra_imm32(cpu: &mut Cpu, ram: &Ram, bra: Reg32, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => cpu.read_next_word(ram),
        3 => {
            let a = cpu.breg(bra);
            let b = get_dword_from_last(cpu);
            let result = alu(cpu, operation, a, b);
            cpu.set_breg(bra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_vra_imm8(cpu: &mut Cpu, ram: &Ram, vra: Reg8, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_next_byte(ram),
        2 => {
            let a = cpu.vreg(vra);
            let b = cpu.last_byte;
            let result = alu(cpu, operation, a, b);
            cpu.set_vreg(vra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_ra_brb(cpu: &mut Cpu, ram: &Ram, ra: Reg16, brb: Reg32, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(ram, cpu.breg(brb)),
        2 => {
            let a = cpu.reg(ra);
            let b = cpu.last_word;
            let result = alu(cpu, operation, a, b);
            cpu.set_reg(ra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_ra(cpu: &mut Cpu, ra: Reg16) {
    match cpu.step_num {
        1 => {
            let a = cpu.reg(ra);
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_bra(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => {
            let a = cpu.breg(bra);
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_vra(cpu: &mut Cpu, vra: Reg8) {
    match cpu.step_num {
        1 => {
            let a = cpu.vreg(vra);
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_imm16(cpu: &mut Cpu, ram: &Ram) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => {
            let a = cpu.last_word;
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_imm32(cpu: &mut Cpu, ram: &Ram) {
    match cpu.step_num {
        1 => cpu.read_next_word(ram),
        2 => cpu.read_next_word(ram),
        3 => {
            let a = get_dword_from_last(cpu);
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_imm8(cpu: &mut Cpu, ram: &Ram) {
    match cpu.step_num {
        1 => cpu.read_next_byte(ram),
        2 => {
            let a = cpu.last_byte;
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_ra_b<T: Into<u16>>(cpu: &mut Cpu, ra: Reg16, b: T, operation: AluOp) {
    match cpu.step_num {
        1 => {
            let a = cpu.reg(ra);
            let result = alu(cpu, operation, a, b.into());
            cpu.set_reg(ra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_bra_b<T: Into<u32>>(cpu: &mut Cpu, bra: Reg32, b: T, operation: AluOp) {
    match cpu.step_num {
        1 => {
            let a = cpu.breg(bra);
            let result = alu(cpu, operation, a, b.into());
            cpu.set_breg(bra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_vra_b<T: Into<u8>>(cpu: &mut Cpu, vra: Reg8, b: T, operation: AluOp) {
    match cpu.step_num {
        1 => {
            let a = cpu.vreg(vra);
            let result = alu(cpu, operation, a, b.into());
            cpu.set_vreg(vra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

#[cfg(test)]
mod instruction_tests;
