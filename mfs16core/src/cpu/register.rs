//! All functionality related to the CPU registers.

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
impl Reg16 {
    /// Get the [Reg16] corresponding to the given nibble, panicking if the nibble does not
    /// correspond to any variant.
    pub fn from_nibble(nibble: u8) -> Reg16 {
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
    /// Get the [Reg8] corresponding to the given nibble, panicking if the nibble does not
    /// correspond to any variant.
    pub fn from_nibble(nibble: u8) -> Reg8 {
        match nibble {
            0x0 => AH,
            0x1 => AL,
            0x2 => BH,
            0x3 => BL,
            0x4 => CH,
            0x5 => CL,
            0x6 => DH,
            0x7 => DL,
            0x8 => EH,
            0x9 => EL,
            0xA => HH,
            0xB => HL,
            0xC => LH,
            0xD => LL,
            _ => panic!("Nibble {:#04X} does not match any 8-bit register.", nibble),
        }
    }
}
impl Display for Reg8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self,)
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
            "A:{:#06X} B:{:#06X} C:{:#06X} D:{:#06X} E:{:#06X} H:{:#06X} L:{:#06X}",
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_registers() {
        #[allow(clippy::too_many_arguments)]
        fn check_regs(regs: &Registers, a: u16, b: u16, c: u16, d: u16, e: u16, h: u16, l: u16) {
            let (ah, al) = split_word(a);
            let (bh, bl) = split_word(b);
            let (ch, cl) = split_word(c);
            let (dh, dl) = split_word(d);
            let (eh, el) = split_word(e);
            let (hh, hl) = split_word(h);
            let (lh, ll) = split_word(l);

            assert_eq!(regs.reg(A), a);
            assert_eq!(regs.reg(B), b);
            assert_eq!(regs.reg(C), c);
            assert_eq!(regs.reg(D), d);
            assert_eq!(regs.reg(E), e);
            assert_eq!(regs.reg(H), h);
            assert_eq!(regs.reg(L), l);

            assert_eq!(regs.vreg(AH), ah);
            assert_eq!(regs.vreg(AL), al);
            assert_eq!(regs.vreg(BH), bh);
            assert_eq!(regs.vreg(BL), bl);
            assert_eq!(regs.vreg(CH), ch);
            assert_eq!(regs.vreg(CL), cl);
            assert_eq!(regs.vreg(DH), dh);
            assert_eq!(regs.vreg(DL), dl);
            assert_eq!(regs.vreg(EH), eh);
            assert_eq!(regs.vreg(EL), el);
            assert_eq!(regs.vreg(HH), hh);
            assert_eq!(regs.vreg(HL), hl);
            assert_eq!(regs.vreg(LH), lh);
            assert_eq!(regs.vreg(LL), ll);
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

        let (ah, al) = split_word(a);
        let (bh, bl) = split_word(b);
        let (ch, cl) = split_word(c);
        let (dh, dl) = split_word(d);
        let (eh, el) = split_word(e);
        let (hh, hl) = split_word(h);
        let (lh, ll) = split_word(l);

        regs_2.set_vreg(AH, ah);
        assert_eq!(regs_2.reg(A), combine_u8_be(ah, 0x00));
        regs_2.set_vreg(AL, al);
        regs_2.set_vreg(BL, bl);
        assert_eq!(regs_2.reg(B), combine_u8_be(0x00, bl));
        regs_2.set_vreg(BH, bh);
        regs_2.set_vreg(CH, ch);
        regs_2.set_vreg(CL, cl);
        regs_2.set_vreg(DH, dh);
        regs_2.set_vreg(DL, dl);
        regs_2.set_vreg(EH, eh);
        regs_2.set_vreg(EL, el);
        regs_2.set_vreg(HH, hh);
        regs_2.set_vreg(HL, hl);
        regs_2.set_vreg(LH, lh);
        regs_2.set_vreg(LL, ll);

        check_regs(&regs_2, a, b, c, d, e, h, l);

        assert_eq!(regs_1, regs_2);
    }
}
