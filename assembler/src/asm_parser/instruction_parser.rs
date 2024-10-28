use std::{fmt::Display, str::FromStr};

use color_eyre::eyre::{self, eyre, Report};

use mfs16core::{
    Instruction::{self, *},
    Reg16, Reg32, Reg8,
};
use Operand::*;

/// All the possible keywords denoting the beginning of an [Instruction].
#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Nop,
    Ld,
    Ldi,
    Ldd,
    Add,
    Adc,
    Sub,
    Sbb,
    Tcp,
    Inc,
    Dec,
    Pss,
    And,
    Or,
    Xor,
    Not,
    Asr,
    Asl,
    Lsr,
    Rtr,
    Rtl,
    Rcr,
    Rcl,
}
impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "nop" => Ok(Operation::Nop),
            "ld" => Ok(Operation::Ld),
            "ldi" => Ok(Operation::Ldi),
            "ldd" => Ok(Operation::Ldd),
            "add" => Ok(Operation::Add),
            "adc" => Ok(Operation::Adc),
            "sub" => Ok(Operation::Sub),
            "sbb" => Ok(Operation::Sbb),
            "tcp" => Ok(Operation::Tcp),
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            "pss" => Ok(Operation::Pss),
            "and" => Ok(Operation::And),
            "or" => Ok(Operation::Or),
            "xor" => Ok(Operation::Xor),
            "not" => Ok(Operation::Not),
            "asr" => Ok(Operation::Asr),
            "asl" => Ok(Operation::Asl),
            "lsr" => Ok(Operation::Lsr),
            "rtr" => Ok(Operation::Rtr),
            "rtl" => Ok(Operation::Rtl),
            "rcr" => Ok(Operation::Rcr),
            "rcl" => Ok(Operation::Rcl),
            _ => Err(eyre!(
                "Input `{}` does not match an instruction operation.",
                s
            )),
        }
    }
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::Nop => "NOP",
                Operation::Ld => "LD",
                Operation::Ldi => "LDI",
                Operation::Ldd => "LDD",
                Operation::Add => "ADD",
                Operation::Adc => "ADC",
                Operation::Sub => "SUB",
                Operation::Sbb => "SBB",
                Operation::Tcp => "TCP",
                Operation::Inc => "INC",
                Operation::Dec => "DEC",
                Operation::Pss => "PSS",
                Operation::And => "AND",
                Operation::Or => "OR",
                Operation::Xor => "XOR",
                Operation::Not => "NOT",
                Operation::Asr => "ASR",
                Operation::Asl => "ASL",
                Operation::Lsr => "LSR",
                Operation::Rtr => "RTR",
                Operation::Rtl => "RTL",
                Operation::Rcr => "RCR",
                Operation::Rcl => "RCL",
            }
        )
    }
}

/// All the possible [Instruction] operand types.
pub enum Operand {
    /// e.g., `A`
    Reg(Reg16),
    /// e.g., `HL`
    Breg(Reg32),
    /// e.g., `E0`
    Vreg(Reg8),
    /// e.g., `[DE]`
    BregDeref(Reg32),
    /// e.g., `12:b`
    Byte(u8),
    /// e.g., `0x1234:w`
    Word(u16),
    /// e.g., `0x1234_5678:d`
    DWord(u32),
    /// e.g., `0x0123_4567_89AB_CDEF`
    QWord(u64),
    /// e.g., `[0x0012_3456:d]`
    DWordDeref(u32),
    /// The program counter.
    ProgramCounter,
    /// The stack pointer.
    StackPointer,
    /// No operand
    None,
}
impl Operand {
    /// Return true iff this [Operand] is not variant [Operand::None].
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}
impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Reg(reg) => format!("{reg}"),
                Self::Breg(breg) => format!("{breg}"),
                Self::Vreg(vreg) => format!("{vreg}"),
                Self::BregDeref(breg) => format!("[{breg}]"),
                Self::Byte(b) => format!("{b}"),
                Self::Word(w) => format!("{w}"),
                Self::DWord(d) => format!("{d}"),
                Self::DWordDeref(d) => format!("[{d}]"),
                Self::QWord(q) => format!("{q}"),
                Self::ProgramCounter => "PC".to_owned(),
                Self::StackPointer => "SP".to_owned(),
                Self::None => "None".to_owned(),
            }
        )
    }
}

/// Match an [Operation] and its two [Operand]s (if any) to their corresponding bytes.
pub fn instr_to_bytes(
    operation: &Operation,
    operand_1: &Operand,
    operand_2: &Operand,
) -> eyre::Result<Vec<u8>> {
    match (operation, operand_1, operand_2) {
        (Operation::Nop, None, None) => Ok(i2b(Nop)),
        (Operation::Ld, Reg(ra), Reg(rb)) => Ok(i2b(LdRaRb(*ra, *rb))),
        (Operation::Ld, Breg(bra), Breg(brb)) => Ok(i2b(LdBraBrb(*bra, *brb))),
        (Operation::Ld, StackPointer, DWord(d)) => Ok(i2b_imm32(LdSpImm32, *d)),
        (Operation::Ld, DWordDeref(d), StackPointer) => Ok(i2b_imm32(LdImm32Sp, *d)),
        (Operation::Ld, StackPointer, Breg(bra)) => Ok(i2b(LdSpBra(*bra))),
        (Operation::Ld, Vreg(vra), Vreg(vrb)) => Ok(i2b(LdVraVrb(*vra, *vrb))),
        (Operation::Ld, Reg(ra), Word(w)) => Ok(i2b_imm16(LdRaImm16(*ra), *w)),
        (Operation::Ld, Breg(bra), DWord(d)) => Ok(i2b_imm32(LdBraImm32(*bra), *d)),
        (Operation::Ld, Vreg(vra), Byte(b)) => Ok(i2b_imm8(LdVraImm8(*vra), *b)),
        (Operation::Ld, BregDeref(bra), Word(w)) => Ok(i2b_imm16(LdBraImm16(*bra), *w)),
        (Operation::Ld, BregDeref(bra), Reg(rb)) => Ok(i2b(LdBraRb(*bra, *rb))),
        (Operation::Ld, Reg(ra), BregDeref(brb)) => Ok(i2b(LdRaBrb(*ra, *brb))),
        (Operation::Ldi, BregDeref(bra), Reg(rb)) => Ok(i2b(LdiBraRb(*bra, *rb))),
        (Operation::Ldd, BregDeref(bra), Reg(rb)) => Ok(i2b(LddBraRb(*bra, *rb))),
        (Operation::Ldi, Reg(ra), BregDeref(brb)) => Ok(i2b(LdiRaBrb(*ra, *brb))),
        (Operation::Ldd, Reg(ra), BregDeref(brb)) => Ok(i2b(LddRaBrb(*ra, *brb))),
        (Operation::Add, Reg(ra), Reg(rb)) => Ok(i2b(AddRaRb(*ra, *rb))),
        (Operation::Add, Breg(bra), Breg(brb)) => Ok(i2b(AddBraBrb(*bra, *brb))),
        (Operation::Add, Vreg(vra), Vreg(vrb)) => Ok(i2b(AddVraVrb(*vra, *vrb))),
        (Operation::Adc, Reg(ra), Reg(rb)) => Ok(i2b(AdcRaRb(*ra, *rb))),
        (Operation::Adc, Breg(bra), Breg(brb)) => Ok(i2b(AdcBraBrb(*bra, *brb))),
        (Operation::Adc, Vreg(vra), Vreg(vrb)) => Ok(i2b(AdcVraVrb(*vra, *vrb))),
        (Operation::Sub, Reg(ra), Reg(rb)) => Ok(i2b(SubRaRb(*ra, *rb))),
        (Operation::Sub, Breg(bra), Breg(brb)) => Ok(i2b(SubBraBrb(*bra, *brb))),
        (Operation::Sub, Vreg(vra), Vreg(vrb)) => Ok(i2b(SubVraVrb(*vra, *vrb))),
        (Operation::Sbb, Reg(ra), Reg(rb)) => Ok(i2b(SbbRaRb(*ra, *rb))),
        (Operation::Sbb, Breg(bra), Breg(brb)) => Ok(i2b(SbbBraBrb(*bra, *brb))),
        (Operation::Sbb, Vreg(vra), Vreg(vrb)) => Ok(i2b(SbbVraVrb(*vra, *vrb))),
        (Operation::Add, Reg(ra), Word(w)) => Ok(i2b_imm16(AddRaImm16(*ra), *w)),
        (Operation::Adc, Reg(ra), Word(w)) => Ok(i2b_imm16(AdcRaImm16(*ra), *w)),
        (Operation::Add, Breg(bra), DWord(d)) => Ok(i2b_imm32(AddBraImm32(*bra), *d)),
        (Operation::Adc, Breg(bra), DWord(d)) => Ok(i2b_imm32(AdcBraImm32(*bra), *d)),
        (Operation::Add, Vreg(vra), Byte(b)) => Ok(i2b_imm8(AddVraImm8(*vra), *b)),
        (Operation::Adc, Vreg(vra), Byte(b)) => Ok(i2b_imm8(AdcVraImm8(*vra), *b)),
        (Operation::Sub, Reg(ra), Word(w)) => Ok(i2b_imm16(SubRaImm16(*ra), *w)),
        (Operation::Sbb, Reg(ra), Word(w)) => Ok(i2b_imm16(SbbRaImm16(*ra), *w)),
        (Operation::Sub, Breg(bra), DWord(d)) => Ok(i2b_imm32(SubBraImm32(*bra), *d)),
        (Operation::Sbb, Breg(bra), DWord(d)) => Ok(i2b_imm32(SbbBraImm32(*bra), *d)),
        (Operation::Sub, Vreg(vra), Byte(b)) => Ok(i2b_imm8(SubVraImm8(*vra), *b)),
        (Operation::Sbb, Vreg(vra), Byte(b)) => Ok(i2b_imm8(SbbVraImm8(*vra), *b)),
        (Operation::Add, Reg(ra), BregDeref(brb)) => Ok(i2b(AddRaBrb(*ra, *brb))),
        (Operation::Adc, Reg(ra), BregDeref(brb)) => Ok(i2b(AdcRaBrb(*ra, *brb))),
        (Operation::Sub, Reg(ra), BregDeref(brb)) => Ok(i2b(SubRaBrb(*ra, *brb))),
        (Operation::Sbb, Reg(ra), BregDeref(brb)) => Ok(i2b(SbbRaBrb(*ra, *brb))),
        (Operation::Tcp, Reg(ra), None) => Ok(i2b(TcpRa(*ra))),
        (Operation::Tcp, Breg(bra), None) => Ok(i2b(TcpBra(*bra))),
        (Operation::Tcp, Vreg(vra), None) => Ok(i2b(TcpVra(*vra))),
        (Operation::Inc, Reg(ra), None) => Ok(i2b(IncRa(*ra))),
        (Operation::Inc, Breg(bra), None) => Ok(i2b(IncBra(*bra))),
        (Operation::Inc, Vreg(vra), None) => Ok(i2b(IncVra(*vra))),
        (Operation::Dec, Reg(ra), None) => Ok(i2b(DecRa(*ra))),
        (Operation::Dec, Breg(bra), None) => Ok(i2b(DecBra(*bra))),
        (Operation::Dec, Vreg(vra), None) => Ok(i2b(DecVra(*vra))),
        (Operation::Pss, Reg(ra), None) => Ok(i2b(PssRa(*ra))),
        (Operation::Pss, Breg(bra), None) => Ok(i2b(PssBra(*bra))),
        (Operation::Pss, Vreg(vra), None) => Ok(i2b(PssVra(*vra))),
        (Operation::Pss, Word(w), None) => Ok(i2b_imm16(PssImm16, *w)),
        (Operation::Pss, DWord(d), None) => Ok(i2b_imm32(PssImm32, *d)),
        (Operation::Pss, Byte(b), None) => Ok(i2b_imm8(PssImm8, *b)),
        (Operation::And, Reg(ra), Reg(rb)) => Ok(i2b(AndRaRb(*ra, *rb))),
        (Operation::And, Breg(bra), Breg(brb)) => Ok(i2b(AndBraBrb(*bra, *brb))),
        (Operation::And, Vreg(vra), Vreg(vrb)) => Ok(i2b(AndVraVrb(*vra, *vrb))),
        (Operation::And, Reg(ra), BregDeref(brb)) => Ok(i2b(AndRaBrb(*ra, *brb))),
        (Operation::Or, Reg(ra), Reg(rb)) => Ok(i2b(OrRaRb(*ra, *rb))),
        (Operation::Or, Breg(bra), Breg(brb)) => Ok(i2b(OrBraBrb(*bra, *brb))),
        (Operation::Or, Vreg(vra), Vreg(vrb)) => Ok(i2b(OrVraVrb(*vra, *vrb))),
        (Operation::Or, Reg(ra), BregDeref(brb)) => Ok(i2b(OrRaBrb(*ra, *brb))),
        (Operation::Xor, Reg(ra), Reg(rb)) => Ok(i2b(XorRaRb(*ra, *rb))),
        (Operation::Xor, Breg(bra), Breg(brb)) => Ok(i2b(XorBraBrb(*bra, *brb))),
        (Operation::Xor, Vreg(vra), Vreg(vrb)) => Ok(i2b(XorVraVrb(*vra, *vrb))),
        (Operation::Xor, Reg(ra), BregDeref(brb)) => Ok(i2b(XorRaBrb(*ra, *brb))),
        (Operation::And, Reg(ra), Word(w)) => Ok(i2b_imm16(AndRaImm16(*ra), *w)),
        (Operation::And, Breg(bra), DWord(d)) => Ok(i2b_imm32(AndBraImm32(*bra), *d)),
        (Operation::And, Vreg(vra), Byte(b)) => Ok(i2b_imm8(AndVraImm8(*vra), *b)),
        (Operation::Or, Reg(ra), Word(w)) => Ok(i2b_imm16(OrRaImm16(*ra), *w)),
        (Operation::Or, Breg(bra), DWord(d)) => Ok(i2b_imm32(OrBraImm32(*bra), *d)),
        (Operation::Or, Vreg(vra), Byte(b)) => Ok(i2b_imm8(OrVraImm8(*vra), *b)),
        (Operation::Xor, Reg(ra), Word(w)) => Ok(i2b_imm16(XorRaImm16(*ra), *w)),
        (Operation::Xor, Breg(bra), DWord(d)) => Ok(i2b_imm32(XorBraImm32(*bra), *d)),
        (Operation::Xor, Vreg(vra), Byte(b)) => Ok(i2b_imm8(XorVraImm8(*vra), *b)),
        (Operation::Not, Reg(ra), None) => Ok(i2b(NotRa(*ra))),
        (Operation::Not, Breg(bra), None) => Ok(i2b(NotBra(*bra))),
        (Operation::Not, Vreg(vra), None) => Ok(i2b(NotVra(*vra))),
        (Operation::Asr, Reg(ra), Byte(b)) => Ok(i2b(AsrRaB(*ra, *b))),
        (Operation::Asr, Breg(bra), Byte(b)) => Ok(i2b(AsrBraB(*bra, *b))),
        (Operation::Asr, Vreg(vra), Byte(b)) => Ok(i2b(AsrVraB(*vra, *b))),
        (Operation::Asl, Reg(ra), Byte(b)) => Ok(i2b(AslRaB(*ra, *b))),
        (Operation::Asl, Breg(bra), Byte(b)) => Ok(i2b(AslBraB(*bra, *b))),
        (Operation::Asl, Vreg(vra), Byte(b)) => Ok(i2b(AslVraB(*vra, *b))),
        (Operation::Lsr, Reg(ra), Byte(b)) => Ok(i2b(LsrRaB(*ra, *b))),
        (Operation::Lsr, Breg(bra), Byte(b)) => Ok(i2b(LsrBraB(*bra, *b))),
        (Operation::Lsr, Vreg(vra), Byte(b)) => Ok(i2b(LsrVraB(*vra, *b))),
        (Operation::Rtr, Reg(ra), Byte(b)) => Ok(i2b(RtrRaB(*ra, *b))),
        (Operation::Rtr, Breg(bra), Byte(b)) => Ok(i2b(RtrBraB(*bra, *b))),
        (Operation::Rtr, Vreg(vra), Byte(b)) => Ok(i2b(RtrVraB(*vra, *b))),
        (Operation::Rtl, Reg(ra), Byte(b)) => Ok(i2b(RtlRaB(*ra, *b))),
        (Operation::Rtl, Breg(bra), Byte(b)) => Ok(i2b(RtlBraB(*bra, *b))),
        (Operation::Rtl, Vreg(vra), Byte(b)) => Ok(i2b(RtlVraB(*vra, *b))),
        (Operation::Rcr, Reg(ra), Byte(b)) => Ok(i2b(RcrRaB(*ra, *b))),
        (Operation::Rcr, Breg(bra), Byte(b)) => Ok(i2b(RcrBraB(*bra, *b))),
        (Operation::Rcr, Vreg(vra), Byte(b)) => Ok(i2b(RcrVraB(*vra, *b))),
        (Operation::Rcl, Reg(ra), Byte(b)) => Ok(i2b(RclRaB(*ra, *b))),
        (Operation::Rcl, Breg(bra), Byte(b)) => Ok(i2b(RclBraB(*bra, *b))),
        (Operation::Rcl, Vreg(vra), Byte(b)) => Ok(i2b(RclVraB(*vra, *b))),
        _ => Err(eyre!(
            "`{}, {}` are invalid operand(s) for {}.",
            operand_1,
            operand_2,
            operation
        )),
    }
}

macro_rules! i2b_imm {
    ($(($t:ty, $fn_name:ident)),*) => {
       $(
           fn $fn_name(instr: Instruction, imm: $t) -> Vec<u8> {
                i2b(instr).into_iter().chain(imm.to_le_bytes()).collect()
           }
       )+
    };
}
i2b_imm!((u8, i2b_imm8), (u16, i2b_imm16), (u32, i2b_imm32));

fn i2b(instr: Instruction) -> Vec<u8> {
    instr.into_opcode().to_le_bytes().to_vec()
}
