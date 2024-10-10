use std::{default::Default, fmt::Display};

use crate::helpers::{combine_u8_be, split_word};

use Reg16::*;
use Reg8::*;

/// Enum to access the individual 16-bit CPU registers.
#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    /// Register A.
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
impl Display for Reg16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                A => "A",
                B => "B",
                C => "C",
                D => "D",
                E => "E",
                H => "H",
                L => "L",
            }
        )
    }
}

/// Enum to access the 8-bit virtual CPU registers.
#[derive(Debug, Copy, Clone)]
pub enum Reg8 {
    /// High bit of register A.
    AH,
    /// Low bit of register A.
    AL,
    /// High bit of register B.
    BH,
    /// Low bit of register B.
    BL,
    /// High bit of register C.
    CH,
    /// Low bit of register C.
    CL,
    /// High bit of register D.
    DH,
    /// Low bit of register D.
    DL,
    /// High bit of register E.
    EH,
    /// Low bit of register E.
    EL,
    /// High bit of register H.
    HH,
    /// Low bit of register H.
    HL,
    /// High bit of register L.
    LH,
    /// Low bit of register L.
    LL,
}
impl Reg8 {
    /// Return true iff the given [Reg8] points to the high byte of its register.
    fn is_high(&self) -> bool {
        matches!(self, AH | BH | CH | DH | HH | LH)
    }
}
impl Display for Reg8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AH => "AH",
                AL => "AL",
                BH => "BH",
                BL => "BL",
                CH => "CH",
                CL => "CL",
                DH => "DH",
                DL => "DL",
                EH => "EH",
                EL => "EL",
                HH => "HH",
                HL => "HL",
                LH => "LH",
                LL => "LL",
            }
        )
    }
}

/// The registers of the CPU.
#[derive(Debug, Clone, PartialEq)]
pub struct Registers {
    /// High bit of register A.
    ah: u8,
    /// Low bit of register A.
    al: u8,
    /// High bit of register B.
    bh: u8,
    /// Low bit of register B.
    bl: u8,
    /// High bit of register C.
    ch: u8,
    /// Low bit of register C.
    cl: u8,
    /// High bit of register D.
    dh: u8,
    /// Low bit of register D.
    dl: u8,
    /// High bit of register E.
    eh: u8,
    /// Low bit of register E.
    el: u8,
    /// High bit of register H.
    hh: u8,
    /// Low bit of register H.
    hl: u8,
    /// High bit of register L.
    lh: u8,
    /// Low bit of register L.
    ll: u8,
}
impl Registers {
    /// Create new [Registers] set to the given values.
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, h: u16, l: u16) -> Self {
        let (ah, al) = split_word(a);
        let (bh, bl) = split_word(b);
        let (ch, cl) = split_word(c);
        let (dh, dl) = split_word(d);
        let (eh, el) = split_word(e);
        let (hh, hl) = split_word(h);
        let (lh, ll) = split_word(l);
        Self {
            ah,
            al,
            bh,
            bl,
            ch,
            cl,
            dh,
            dl,
            eh,
            el,
            hh,
            hl,
            lh,
            ll,
        }
    }

    /// Get the value of a 16-bit register.
    pub fn reg(&self, reg: Reg16) -> u16 {
        match reg {
            A => combine_u8_be(self.ah, self.al),
            B => combine_u8_be(self.bh, self.bl),
            C => combine_u8_be(self.ch, self.cl),
            D => combine_u8_be(self.dh, self.dl),
            E => combine_u8_be(self.eh, self.el),
            H => combine_u8_be(self.hh, self.hl),
            L => combine_u8_be(self.lh, self.ll),
        }
    }

    /// Set the value of a 16-bit register.
    pub fn set_reg(&mut self, reg: Reg16, val: u16) {
        let split_val = split_word(val);
        match reg {
            A => (self.ah, self.al) = split_val,
            B => (self.bh, self.bl) = split_val,
            C => (self.ch, self.cl) = split_val,
            D => (self.dh, self.dl) = split_val,
            E => (self.eh, self.el) = split_val,
            H => (self.hh, self.hl) = split_val,
            L => (self.lh, self.ll) = split_val,
        }
    }

    /// Get the value of an 8-bit virtual register.
    pub fn vreg(&self, vreg: Reg8) -> u8 {
        match vreg {
            AH => self.ah,
            AL => self.al,
            BH => self.bh,
            BL => self.bl,
            CH => self.ch,
            CL => self.cl,
            DH => self.dh,
            DL => self.dl,
            EH => self.eh,
            EL => self.el,
            HH => self.hh,
            HL => self.hl,
            LH => self.lh,
            LL => self.ll,
        }
    }

    /// Set the value of an 8-bit virtual register.
    pub fn set_vreg(&mut self, vreg: Reg8, val: u8) {
        match vreg {
            AH => self.ah = val,
            AL => self.al = val,
            BH => self.bh = val,
            BL => self.bl = val,
            CH => self.ch = val,
            CL => self.cl = val,
            DH => self.dh = val,
            DL => self.dl = val,
            EH => self.eh = val,
            EL => self.el = val,
            HH => self.hh = val,
            HL => self.hl = val,
            LH => self.lh = val,
            LL => self.ll = val,
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
            "A: {:#06X}, B: {:#06X}, C: {:#06X}, D: {:#06X}, E: {:#06X}, H: {:#06X}, L: {:#06X}",
            combine_u8_be(self.ah, self.al),
            combine_u8_be(self.bh, self.bl),
            combine_u8_be(self.ch, self.cl),
            combine_u8_be(self.dh, self.dl),
            combine_u8_be(self.eh, self.el),
            combine_u8_be(self.hh, self.hl),
            combine_u8_be(self.lh, self.ll),
        )
    }
}
