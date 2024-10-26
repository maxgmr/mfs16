//! All functionality related to the CPU registers.

use std::{default::Default, fmt::Display};

use super::Cpu;
use crate::helpers::{combine_u16_be, combine_u8_be, split_dword, split_word};

use Reg16::*;
use Reg32::*;
use Reg8::*;

/// Enum to access the individual 16-bit CPU registers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Reg16 {
    /// Register A.
    #[default]
    A,
    /// Register B.
    B,
    /// Register C.
    C,
    /// Register D.
    D,
    /// Register E.
    E,
    /// Register H.
    H,
    /// Register L.
    L,
}
impl Reg16 {
    /// Get the [Reg16] corresponding to the given nibble, panicking if the nibble does not
    /// correspond to any variant.
    pub fn from_nib(nibble: u8) -> Reg16 {
        match nibble {
            0x0 => A,
            0x1 => B,
            0x2 => C,
            0x3 => D,
            0x4 => E,
            0x5 => H,
            0x6 => L,
            _ => panic!("Nibble {:#04X} does not match any 16-bit register.", nibble),
        }
    }
}
impl Display for Reg16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}
impl TryFrom<&str> for Reg16 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(A),
            "B" => Ok(B),
            "C" => Ok(C),
            "D" => Ok(D),
            "E" => Ok(E),
            "H" => Ok(H),
            "L" => Ok(L),
            _ => Err(format!("String `{value}` does not correspond to a Reg16.")),
        }
    }
}
#[allow(clippy::from_over_into)]
impl Into<u16> for Reg16 {
    fn into(self) -> u16 {
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            H => 5,
            L => 6,
        }
    }
}

/// Enum to access the 32-bit "big" virtual CPU registers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Reg32 {
    /// Register B & C.
    #[default]
    BC,
    /// Register D & E.
    DE,
    /// Register H & L.
    HL,
}
impl Reg32 {
    /// Get the [Reg32] corresponding to the given nibble, panicking if the nibble does not
    /// correspond to any variant.
    pub fn from_nib(nibble: u8) -> Reg32 {
        match nibble {
            0x0 => BC,
            0x1 => DE,
            0x2 => HL,
            _ => panic!("Nibble {:#04X} does not match any 32-bit register.", nibble),
        }
    }
}
impl Display for Reg32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}
impl TryFrom<&str> for Reg32 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BC" => Ok(BC),
            "DE" => Ok(DE),
            "HL" => Ok(HL),
            _ => Err(format!("String `{value}` does not correspond to a Reg32.")),
        }
    }
}
#[allow(clippy::from_over_into)]
impl Into<u16> for Reg32 {
    fn into(self) -> u16 {
        match self {
            BC => 0,
            DE => 1,
            HL => 2,
        }
    }
}

/// Enum to access the 8-bit virtual CPU registers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Reg8 {
    /// High bit of register A.
    #[default]
    A1,
    /// Low bit of register A.
    A0,
    /// High bit of register B.
    B1,
    /// Low bit of register B.
    B0,
    /// High bit of register C.
    C1,
    /// Low bit of register C.
    C0,
    /// High bit of register D.
    D1,
    /// Low bit of register D.
    D0,
    /// High bit of register E.
    E1,
    /// Low bit of register E.
    E0,
    /// High bit of register H.
    H1,
    /// Low bit of register H.
    H0,
    /// High bit of register L.
    L1,
    /// Low bit of register L.
    L0,
}
impl Reg8 {
    /// Get the [Reg8] corresponding to the given nibble, panicking if the nibble does not
    /// correspond to any variant.
    pub fn from_nib(nibble: u8) -> Reg8 {
        match nibble {
            0x0 => A1,
            0x1 => A0,
            0x2 => B1,
            0x3 => B0,
            0x4 => C1,
            0x5 => C0,
            0x6 => D1,
            0x7 => D0,
            0x8 => E1,
            0x9 => E0,
            0xA => H1,
            0xB => H0,
            0xC => L1,
            0xD => L0,
            _ => panic!("Nibble {:#04X} does not match any 8-bit register.", nibble),
        }
    }
}
impl Display for Reg8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
    }
}
impl TryFrom<&str> for Reg8 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A1" => Ok(A1),
            "A0" => Ok(A0),
            "B1" => Ok(B1),
            "B0" => Ok(B0),
            "C1" => Ok(C1),
            "C0" => Ok(C0),
            "D1" => Ok(D1),
            "D0" => Ok(D0),
            "E1" => Ok(E1),
            "E0" => Ok(E0),
            "H1" => Ok(H1),
            "H0" => Ok(H0),
            "L1" => Ok(L1),
            "L0" => Ok(L0),
            _ => Err(format!("String `{value}` does not correspond to a Reg8.")),
        }
    }
}
#[allow(clippy::from_over_into)]
impl Into<u16> for Reg8 {
    fn into(self) -> u16 {
        match self {
            A1 => 0,
            A0 => 1,
            B1 => 2,
            B0 => 3,
            C1 => 4,
            C0 => 5,
            D1 => 6,
            D0 => 7,
            E1 => 8,
            E0 => 9,
            H1 => 10,
            H0 => 11,
            L1 => 12,
            L0 => 13,
        }
    }
}

/// The registers of the CPU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registers {
    /// High bit of register A.
    a1: u8,
    /// Low bit of register A.
    a0: u8,
    /// High bit of register B.
    b1: u8,
    /// Low bit of register B.
    b0: u8,
    /// High bit of register C.
    c1: u8,
    /// Low bit of register C.
    c0: u8,
    /// High bit of register D.
    d1: u8,
    /// Low bit of register D.
    d0: u8,
    /// High bit of register E.
    e1: u8,
    /// Low bit of register E.
    e0: u8,
    /// High bit of register H.
    h1: u8,
    /// Low bit of register H.
    h0: u8,
    /// High bit of register L.
    l1: u8,
    /// Low bit of register L.
    l0: u8,
}
impl Registers {
    /// Create new [Registers] set to the given values.
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, h: u16, l: u16) -> Self {
        let (a1, a0) = split_word(a);
        let (b1, b0) = split_word(b);
        let (c1, c0) = split_word(c);
        let (d1, d0) = split_word(d);
        let (e1, e0) = split_word(e);
        let (h1, h0) = split_word(h);
        let (l1, l0) = split_word(l);
        Self {
            a1,
            a0,
            b1,
            b0,
            c1,
            c0,
            d1,
            d0,
            e1,
            e0,
            h1,
            h0,
            l1,
            l0,
        }
    }

    /// Get the value of a 16-bit register.
    pub fn reg(&self, reg: Reg16) -> u16 {
        match reg {
            A => combine_u8_be(self.a1, self.a0),
            B => combine_u8_be(self.b1, self.b0),
            C => combine_u8_be(self.c1, self.c0),
            D => combine_u8_be(self.d1, self.d0),
            E => combine_u8_be(self.e1, self.e0),
            H => combine_u8_be(self.h1, self.h0),
            L => combine_u8_be(self.l1, self.l0),
        }
    }

    /// Set the value of a 16-bit register.
    pub fn set_reg(&mut self, reg: Reg16, val: u16) {
        let split_val = split_word(val);
        match reg {
            A => (self.a1, self.a0) = split_val,
            B => (self.b1, self.b0) = split_val,
            C => (self.c1, self.c0) = split_val,
            D => (self.d1, self.d0) = split_val,
            E => (self.e1, self.e0) = split_val,
            H => (self.h1, self.h0) = split_val,
            L => (self.l1, self.l0) = split_val,
        }
    }

    /// Get the value of a 16-bit big register.
    pub fn breg(&self, breg: Reg32) -> u32 {
        match breg {
            BC => combine_u16_be(self.reg(B), self.reg(C)),
            DE => combine_u16_be(self.reg(D), self.reg(E)),
            HL => combine_u16_be(self.reg(H), self.reg(L)),
        }
    }

    /// Set the value of a 32-bit big register.
    pub fn set_breg(&mut self, breg: Reg32, val: u32) {
        let (hw, lw) = split_dword(val);
        match breg {
            BC => {
                self.set_reg(B, hw);
                self.set_reg(C, lw);
            }
            DE => {
                self.set_reg(D, hw);
                self.set_reg(E, lw);
            }
            HL => {
                self.set_reg(H, hw);
                self.set_reg(L, lw);
            }
        }
    }

    /// Get the value of an 8-bit virtual register.
    pub fn vreg(&self, vreg: Reg8) -> u8 {
        match vreg {
            A1 => self.a1,
            A0 => self.a0,
            B1 => self.b1,
            B0 => self.b0,
            C1 => self.c1,
            C0 => self.c0,
            D1 => self.d1,
            D0 => self.d0,
            E1 => self.e1,
            E0 => self.e0,
            H1 => self.h1,
            H0 => self.h0,
            L1 => self.l1,
            L0 => self.l0,
        }
    }

    /// Set the value of an 8-bit virtual register.
    pub fn set_vreg(&mut self, vreg: Reg8, val: u8) {
        match vreg {
            A1 => self.a1 = val,
            A0 => self.a0 = val,
            B1 => self.b1 = val,
            B0 => self.b0 = val,
            C1 => self.c1 = val,
            C0 => self.c0 = val,
            D1 => self.d1 = val,
            D0 => self.d0 = val,
            E1 => self.e1 = val,
            E0 => self.e0 = val,
            H1 => self.h1 = val,
            H0 => self.h0 = val,
            L1 => self.l1 = val,
            L0 => self.l0 = val,
        }
    }
}
impl Default for Registers {
    /// Default: all registers initialised to 0x0000.
    fn default() -> Self {
        Self::new(0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000)
    }
}
impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A:{:#06X} B:{:#06X} C:{:#06X} D:{:#06X} E:{:#06X} H:{:#06X} L:{:#06X}",
            combine_u8_be(self.a1, self.a0),
            combine_u8_be(self.b1, self.b0),
            combine_u8_be(self.c1, self.c0),
            combine_u8_be(self.d1, self.d0),
            combine_u8_be(self.e1, self.e0),
            combine_u8_be(self.h1, self.h0),
            combine_u8_be(self.l1, self.l0),
        )
    }
}

/// Implementors of this trait can be used as labels to identify [Cpu] registers.
pub trait Reg {
    /// The type of the value held by the register.
    type ValueType;
    /// Get the register of the given [Cpu] matching the given value.
    fn get(&self, cpu: &Cpu) -> Self::ValueType;
    /// Set the register of the given [Cpu] to the given value.
    fn set(&self, cpu: &mut Cpu, val: Self::ValueType);
}
macro_rules! impl_reg {
    ($(($t:ty, $g_fn:ident, $s_fn:ident, $vt:ty)),+) => {
        $(
            impl Reg for $t {
                type ValueType = $vt;

                fn get(&self, cpu: &Cpu) -> Self::ValueType {
                    cpu.$g_fn(*self)
                }

                fn set(&self, cpu: &mut Cpu, val: Self::ValueType) {
                    cpu.$s_fn(*self, val)
                }
            }
        )+
    };
}
impl_reg!(
    (Reg16, reg, set_reg, u16),
    (Reg32, breg, set_breg, u32),
    (Reg8, vreg, set_vreg, u8)
);

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_trait() {
        let mut cpu = Cpu::default();
        A0.set(&mut cpu, 0x12);
        assert_eq!(A0.get(&cpu), 0x12);
    }

    #[test]
    fn test_registers() {
        #[allow(clippy::too_many_arguments)]
        fn check_regs(regs: &Registers, a: u16, b: u16, c: u16, d: u16, e: u16, h: u16, l: u16) {
            let (a1, a0) = split_word(a);
            let (b1, b0) = split_word(b);
            let (c1, c0) = split_word(c);
            let (d1, d0) = split_word(d);
            let (e1, e0) = split_word(e);
            let (h1, h0) = split_word(h);
            let (l1, l0) = split_word(l);

            assert_eq!(regs.reg(A), a);
            assert_eq!(regs.reg(B), b);
            assert_eq!(regs.reg(C), c);
            assert_eq!(regs.reg(D), d);
            assert_eq!(regs.reg(E), e);
            assert_eq!(regs.reg(H), h);
            assert_eq!(regs.reg(L), l);

            assert_eq!(regs.vreg(A1), a1);
            assert_eq!(regs.vreg(A0), a0);
            assert_eq!(regs.vreg(B1), b1);
            assert_eq!(regs.vreg(B0), b0);
            assert_eq!(regs.vreg(C1), c1);
            assert_eq!(regs.vreg(C0), c0);
            assert_eq!(regs.vreg(D1), d1);
            assert_eq!(regs.vreg(D0), d0);
            assert_eq!(regs.vreg(E1), e1);
            assert_eq!(regs.vreg(E0), e0);
            assert_eq!(regs.vreg(H1), h1);
            assert_eq!(regs.vreg(H0), h0);
            assert_eq!(regs.vreg(L1), l1);
            assert_eq!(regs.vreg(L0), l0);

            assert_eq!(regs.breg(BC), combine_u16_be(b, c));
            assert_eq!(regs.breg(DE), combine_u16_be(d, e));
            assert_eq!(regs.breg(HL), combine_u16_be(h, l));
        }

        let start_a = 0xFFFF;
        let start_b = 0xEEEE;
        let start_c = 0xDDDD;
        let start_d = 0xCCCC;
        let start_e = 0xBBBB;
        let start_h = 0xAAAA;
        let start_l = 0x9999;
        let mut regs_1 = Registers::new(
            start_a, start_b, start_c, start_d, start_e, start_h, start_l,
        );
        check_regs(
            &regs_1, start_a, start_b, start_c, start_d, start_e, start_h, start_l,
        );

        let a = 0xFEDC;
        let b = 0xBA98;
        let c = 0x7654;
        let d = 0x3210;
        let e = 0x0123;
        let h = 0x4567;
        let l = 0x89AB;

        regs_1.set_reg(A, a);
        regs_1.set_reg(B, b);
        regs_1.set_reg(C, c);
        regs_1.set_reg(D, d);
        regs_1.set_reg(E, e);
        regs_1.set_reg(H, h);
        regs_1.set_reg(L, l);

        check_regs(&regs_1, a, b, c, d, e, h, l);

        let mut regs_2 = Registers::default();
        check_regs(&regs_2, 0, 0, 0, 0, 0, 0, 0);

        let (a1, a0) = split_word(a);
        let (b1, b0) = split_word(b);
        let (c1, c0) = split_word(c);
        let (d1, d0) = split_word(d);
        let (e1, e0) = split_word(e);
        let (h1, h0) = split_word(h);
        let (l1, l0) = split_word(l);

        regs_2.set_vreg(A1, a1);
        assert_eq!(regs_2.reg(A), combine_u8_be(a1, 0x00));
        regs_2.set_vreg(A0, a0);
        regs_2.set_vreg(B0, b0);
        assert_eq!(regs_2.reg(B), combine_u8_be(0x00, b0));
        regs_2.set_vreg(B1, b1);
        regs_2.set_vreg(C1, c1);
        regs_2.set_vreg(C0, c0);
        regs_2.set_vreg(D1, d1);
        regs_2.set_vreg(D0, d0);
        regs_2.set_vreg(E1, e1);
        regs_2.set_vreg(E0, e0);
        regs_2.set_vreg(H1, h1);
        regs_2.set_vreg(H0, h0);
        regs_2.set_vreg(L1, l1);
        regs_2.set_vreg(L0, l0);

        check_regs(&regs_2, a, b, c, d, e, h, l);

        assert_eq!(regs_1, regs_2);

        let a = 0x0000;
        let b = 0x1234;
        let c = 0x5678;
        let d = 0x9ABC;
        let e = 0xDEF0;
        let h = 0x1928;
        let l = 0x3746;

        let mut regs_3 = Registers::default();
        regs_3.set_breg(BC, 0x1234_5678);
        regs_3.set_breg(DE, 0x9ABC_DEF0);
        regs_3.set_breg(HL, 0x1928_3746);

        check_regs(&regs_3, a, b, c, d, e, h, l);
    }
}
