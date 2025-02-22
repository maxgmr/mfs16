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
    Ldr,
    Ldi,
    Ldd,
    Vld,
    Vldi,
    Vldd,
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
    Cmp,
    Bit,
    Stb,
    Rsb,
    Tgb,
    Swp,
    Szf,
    Rzf,
    Tzf,
    Scf,
    Rcf,
    Tcf,
    Sof,
    Rof,
    Tof,
    Spf,
    Rpf,
    Tpf,
    Snf,
    Rnf,
    Tnf,
    Saf,
    Raf,
    Mulu,
    Muli,
    Divu,
    Divi,
    Rand,
    Jp,
    Jr,
    Jpz,
    Jnz,
    Jpc,
    Jnc,
    Jpo,
    Jno,
    Jpp,
    Jnp,
    Jpn,
    Jnn,
    Call,
    Clz,
    Cnz,
    Clc,
    Cnc,
    Clo,
    Cno,
    Clp,
    Cnp,
    Cln,
    Cnn,
    Ret,
    Rtz,
    Rnz,
    Rtc,
    Rnc,
    Rto,
    Rno,
    Rtp,
    Rnp,
    Rtn,
    Rnn,
    Reti,
    Push,
    Pop,
    Peek,
    Clv,
    Stop,
    Ei,
    Di,
    Halt,
}
impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "nop" => Ok(Operation::Nop),
            "ld" => Ok(Operation::Ld),
            "ldr" => Ok(Operation::Ldr),
            "ldi" => Ok(Operation::Ldi),
            "ldd" => Ok(Operation::Ldd),
            "vld" => Ok(Operation::Vld),
            "vldi" => Ok(Operation::Vldi),
            "vldd" => Ok(Operation::Vldd),
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
            "cmp" => Ok(Operation::Cmp),
            "bit" => Ok(Operation::Bit),
            "stb" => Ok(Operation::Stb),
            "rsb" => Ok(Operation::Rsb),
            "tgb" => Ok(Operation::Tgb),
            "swp" => Ok(Operation::Swp),
            "szf" => Ok(Operation::Szf),
            "rzf" => Ok(Operation::Rzf),
            "tzf" => Ok(Operation::Tzf),
            "scf" => Ok(Operation::Scf),
            "rcf" => Ok(Operation::Rcf),
            "tcf" => Ok(Operation::Tcf),
            "sof" => Ok(Operation::Sof),
            "rof" => Ok(Operation::Rof),
            "tof" => Ok(Operation::Tof),
            "spf" => Ok(Operation::Spf),
            "rpf" => Ok(Operation::Rpf),
            "tpf" => Ok(Operation::Tpf),
            "snf" => Ok(Operation::Snf),
            "rnf" => Ok(Operation::Rnf),
            "tnf" => Ok(Operation::Tnf),
            "saf" => Ok(Operation::Saf),
            "raf" => Ok(Operation::Raf),
            "mulu" => Ok(Operation::Mulu),
            "muli" => Ok(Operation::Muli),
            "divu" => Ok(Operation::Divu),
            "divi" => Ok(Operation::Divi),
            "rand" => Ok(Operation::Rand),
            "jp" => Ok(Operation::Jp),
            "jr" => Ok(Operation::Jr),
            "jpz" => Ok(Operation::Jpz),
            "jnz" => Ok(Operation::Jnz),
            "jpc" => Ok(Operation::Jpc),
            "jnc" => Ok(Operation::Jnc),
            "jpo" => Ok(Operation::Jpo),
            "jno" => Ok(Operation::Jno),
            "jpp" => Ok(Operation::Jpp),
            "jnp" => Ok(Operation::Jnp),
            "jpn" => Ok(Operation::Jpn),
            "jnn" => Ok(Operation::Jnn),
            "call" => Ok(Operation::Call),
            "clz" => Ok(Operation::Clz),
            "cnz" => Ok(Operation::Cnz),
            "clc" => Ok(Operation::Clc),
            "cnc" => Ok(Operation::Cnc),
            "clo" => Ok(Operation::Clo),
            "cno" => Ok(Operation::Cno),
            "clp" => Ok(Operation::Clp),
            "cnp" => Ok(Operation::Cnp),
            "cln" => Ok(Operation::Cln),
            "cnn" => Ok(Operation::Cnn),
            "ret" => Ok(Operation::Ret),
            "rtz" => Ok(Operation::Rtz),
            "rnz" => Ok(Operation::Rnz),
            "rtc" => Ok(Operation::Rtc),
            "rnc" => Ok(Operation::Rnc),
            "rto" => Ok(Operation::Rto),
            "rno" => Ok(Operation::Rno),
            "rtp" => Ok(Operation::Rtp),
            "rnp" => Ok(Operation::Rnp),
            "rtn" => Ok(Operation::Rtn),
            "rnn" => Ok(Operation::Rnn),
            "reti" => Ok(Operation::Reti),
            "push" => Ok(Operation::Push),
            "pop" => Ok(Operation::Pop),
            "peek" => Ok(Operation::Peek),
            "clv" => Ok(Operation::Clv),
            "stop" => Ok(Operation::Stop),
            "ei" => Ok(Operation::Ei),
            "di" => Ok(Operation::Di),
            "halt" => Ok(Operation::Halt),
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
                Operation::Ldr => "LDR",
                Operation::Ldi => "LDI",
                Operation::Ldd => "LDD",
                Operation::Vld => "VLD",
                Operation::Vldi => "VLDI",
                Operation::Vldd => "VLDD",
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
                Operation::Cmp => "CMP",
                Operation::Bit => "BIT",
                Operation::Stb => "STB",
                Operation::Rsb => "RSB",
                Operation::Tgb => "TGB",
                Operation::Swp => "SWP",
                Operation::Szf => "SZF",
                Operation::Rzf => "RZF",
                Operation::Tzf => "TZF",
                Operation::Scf => "SCF",
                Operation::Rcf => "RCF",
                Operation::Tcf => "TCF",
                Operation::Sof => "SOF",
                Operation::Rof => "ROF",
                Operation::Tof => "TOF",
                Operation::Spf => "SPF",
                Operation::Rpf => "RPF",
                Operation::Tpf => "TPF",
                Operation::Snf => "SNF",
                Operation::Rnf => "RNF",
                Operation::Tnf => "TNF",
                Operation::Saf => "SAF",
                Operation::Raf => "RAF",
                Operation::Mulu => "MULU",
                Operation::Muli => "MULI",
                Operation::Divu => "DIVU",
                Operation::Divi => "DIVI",
                Operation::Rand => "RAND",
                Operation::Jp => "JP",
                Operation::Jr => "JR",
                Operation::Jpz => "JPZ",
                Operation::Jnz => "JNZ",
                Operation::Jpc => "JPC",
                Operation::Jnc => "JNC",
                Operation::Jpo => "JPO",
                Operation::Jno => "JNO",
                Operation::Jpp => "JPP",
                Operation::Jnp => "JNP",
                Operation::Jpn => "JPN",
                Operation::Jnn => "JNN",
                Operation::Call => "CALL",
                Operation::Clz => "CLZ",
                Operation::Cnz => "CNZ",
                Operation::Clc => "CLC",
                Operation::Cnc => "CNC",
                Operation::Clo => "CLO",
                Operation::Cno => "CNO",
                Operation::Clp => "CLP",
                Operation::Cnp => "CNP",
                Operation::Cln => "CLN",
                Operation::Cnn => "CNN",
                Operation::Ret => "RET",
                Operation::Rtz => "RTZ",
                Operation::Rnz => "RNZ",
                Operation::Rtc => "RTC",
                Operation::Rnc => "RNC",
                Operation::Rto => "RTO",
                Operation::Rno => "RNO",
                Operation::Rtp => "RTP",
                Operation::Rnp => "RNP",
                Operation::Rtn => "RTN",
                Operation::Rnn => "RNN",
                Operation::Reti => "RETI",
                Operation::Push => "PUSH",
                Operation::Pop => "POP",
                Operation::Peek => "PEEK",
                Operation::Stop => "STOP",
                Operation::Clv => "CLV",
                Operation::Ei => "EI",
                Operation::Di => "DI",
                Operation::Halt => "HALT",
            }
        )
    }
}

/// All the possible [Instruction] operand types.
#[derive(Debug, Clone, PartialEq)]
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
        (Operation::Ld, Breg(bra), StackPointer) => Ok(i2b(LdBraSp(*bra))),
        (Operation::Ld, Vreg(vra), Vreg(vrb)) => Ok(i2b(LdVraVrb(*vra, *vrb))),
        (Operation::Ld, Reg(ra), Word(w)) => Ok(i2b_imm16(LdRaImm16(*ra), *w)),
        (Operation::Ld, Breg(bra), DWord(d)) => Ok(i2b_imm32(LdBraImm32(*bra), *d)),
        (Operation::Ld, Vreg(vra), Byte(b)) => Ok(i2b_imm8(LdVraImm8(*vra), *b)),
        (Operation::Ld, BregDeref(bra), Word(w)) => Ok(i2b_imm16(LdBraImm16(*bra), *w)),
        (Operation::Ld, BregDeref(bra), Reg(rb)) => Ok(i2b(LdBraRb(*bra, *rb))),
        (Operation::Ld, Reg(ra), BregDeref(brb)) => Ok(i2b(LdRaBrb(*ra, *brb))),
        (Operation::Ldr, Reg(ra), DWordDeref(d)) => Ok(i2b_imm32(LdrRaImm32(*ra), *d)),
        (Operation::Ldi, BregDeref(bra), Reg(rb)) => Ok(i2b(LdiBraRb(*bra, *rb))),
        (Operation::Ldd, BregDeref(bra), Reg(rb)) => Ok(i2b(LddBraRb(*bra, *rb))),
        (Operation::Ldi, Reg(ra), BregDeref(brb)) => Ok(i2b(LdiRaBrb(*ra, *brb))),
        (Operation::Ldd, Reg(ra), BregDeref(brb)) => Ok(i2b(LddRaBrb(*ra, *brb))),
        (Operation::Ldi, BregDeref(bra), Word(w)) => Ok(i2b_imm16(LdiBraImm16(*bra), *w)),
        (Operation::Ldd, BregDeref(bra), Word(w)) => Ok(i2b_imm16(LddBraImm16(*bra), *w)),
        (Operation::Ld, DWordDeref(d), Reg(ra)) => Ok(i2b_imm32(LdImm32Ra(*ra), *d)),
        (Operation::Ld, Reg(ra), DWordDeref(d)) => Ok(i2b_imm32(LdRaImm32(*ra), *d)),
        (Operation::Vld, BregDeref(bra), Breg(brb)) => Ok(i2b(VldBraBrb(*bra, *brb))),
        (Operation::Vldi, BregDeref(bra), Breg(brb)) => Ok(i2b(VldiBraBrb(*bra, *brb))),
        (Operation::Vldd, BregDeref(bra), Breg(brb)) => Ok(i2b(VlddBraBrb(*bra, *brb))),
        (Operation::Vld, BregDeref(bra), DWord(d)) => Ok(i2b_imm32(VldBraImm32(*bra), *d)),
        (Operation::Vldi, BregDeref(bra), DWord(d)) => Ok(i2b_imm32(VldiBraImm32(*bra), *d)),
        (Operation::Vldd, BregDeref(bra), DWord(d)) => Ok(i2b_imm32(VlddBraImm32(*bra), *d)),
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
        (Operation::Cmp, Reg(ra), Reg(rb)) => Ok(i2b(CmpRaRb(*ra, *rb))),
        (Operation::Cmp, Breg(bra), Breg(brb)) => Ok(i2b(CmpBraBrb(*bra, *brb))),
        (Operation::Cmp, Vreg(vra), Vreg(vrb)) => Ok(i2b(CmpVraVrb(*vra, *vrb))),
        (Operation::Cmp, Reg(ra), Word(w)) => Ok(i2b_imm16(CmpRaImm16(*ra), *w)),
        (Operation::Cmp, Breg(bra), DWord(d)) => Ok(i2b_imm32(CmpBraImm32(*bra), *d)),
        (Operation::Cmp, Vreg(vra), Byte(b)) => Ok(i2b_imm8(CmpVraImm8(*vra), *b)),
        (Operation::Cmp, Word(w), Reg(ra)) => Ok(i2b_imm16(CmpImm16Ra(*ra), *w)),
        (Operation::Cmp, DWord(d), Breg(bra)) => Ok(i2b_imm32(CmpImm32Bra(*bra), *d)),
        (Operation::Cmp, Byte(b), Vreg(vra)) => Ok(i2b_imm8(CmpImm8Vra(*vra), *b)),
        (Operation::Cmp, Reg(ra), BregDeref(brb)) => Ok(i2b(CmpRaBrb(*ra, *brb))),
        (Operation::Cmp, BregDeref(bra), Reg(rb)) => Ok(i2b(CmpBraRb(*bra, *rb))),
        (Operation::Bit, Reg(ra), Byte(b)) => Ok(i2b(BitRaB(*ra, *b))),
        (Operation::Bit, BregDeref(bra), Byte(b)) => Ok(i2b(BitBraB(*bra, *b))),
        (Operation::Stb, Reg(ra), Byte(b)) => Ok(i2b(StbRaB(*ra, *b))),
        (Operation::Stb, BregDeref(bra), Byte(b)) => Ok(i2b(StbBraB(*bra, *b))),
        (Operation::Rsb, Reg(ra), Byte(b)) => Ok(i2b(RsbRaB(*ra, *b))),
        (Operation::Rsb, BregDeref(bra), Byte(b)) => Ok(i2b(RsbBraB(*bra, *b))),
        (Operation::Tgb, Reg(ra), Byte(b)) => Ok(i2b(TgbRaB(*ra, *b))),
        (Operation::Tgb, BregDeref(bra), Byte(b)) => Ok(i2b(TgbBraB(*bra, *b))),
        (Operation::Swp, Reg(ra), None) => Ok(i2b(SwpRa(*ra))),
        (Operation::Swp, BregDeref(bra), None) => Ok(i2b(SwpBra(*bra))),
        (Operation::Szf, None, None) => Ok(i2b(Szf)),
        (Operation::Rzf, None, None) => Ok(i2b(Rzf)),
        (Operation::Tzf, None, None) => Ok(i2b(Tzf)),
        (Operation::Scf, None, None) => Ok(i2b(Scf)),
        (Operation::Rcf, None, None) => Ok(i2b(Rcf)),
        (Operation::Tcf, None, None) => Ok(i2b(Tcf)),
        (Operation::Sof, None, None) => Ok(i2b(Sof)),
        (Operation::Rof, None, None) => Ok(i2b(Rof)),
        (Operation::Tof, None, None) => Ok(i2b(Tof)),
        (Operation::Spf, None, None) => Ok(i2b(Spf)),
        (Operation::Rpf, None, None) => Ok(i2b(Rpf)),
        (Operation::Tpf, None, None) => Ok(i2b(Tpf)),
        (Operation::Snf, None, None) => Ok(i2b(Snf)),
        (Operation::Rnf, None, None) => Ok(i2b(Rnf)),
        (Operation::Tnf, None, None) => Ok(i2b(Tnf)),
        (Operation::Saf, None, None) => Ok(i2b(Saf)),
        (Operation::Raf, None, None) => Ok(i2b(Raf)),
        (Operation::Mulu, Reg(ra), Reg(rb)) => Ok(i2b(MuluRaRb(*ra, *rb))),
        (Operation::Muli, Reg(ra), Reg(rb)) => Ok(i2b(MuliRaRb(*ra, *rb))),
        (Operation::Divu, Reg(ra), Reg(rb)) => Ok(i2b(DivuRaRb(*ra, *rb))),
        (Operation::Divi, Reg(ra), Reg(rb)) => Ok(i2b(DiviRaRb(*ra, *rb))),
        (Operation::Mulu, Breg(bra), Breg(brb)) => Ok(i2b(MuluBraBrb(*bra, *brb))),
        (Operation::Muli, Breg(bra), Breg(brb)) => Ok(i2b(MuliBraBrb(*bra, *brb))),
        (Operation::Divu, Breg(bra), Breg(brb)) => Ok(i2b(DivuBraBrb(*bra, *brb))),
        (Operation::Divi, Breg(bra), Breg(brb)) => Ok(i2b(DiviBraBrb(*bra, *brb))),
        (Operation::Mulu, Vreg(vra), Vreg(vrb)) => Ok(i2b(MuluVraVrb(*vra, *vrb))),
        (Operation::Muli, Vreg(vra), Vreg(vrb)) => Ok(i2b(MuliVraVrb(*vra, *vrb))),
        (Operation::Divu, Vreg(vra), Vreg(vrb)) => Ok(i2b(DivuVraVrb(*vra, *vrb))),
        (Operation::Divi, Vreg(vra), Vreg(vrb)) => Ok(i2b(DiviVraVrb(*vra, *vrb))),
        (Operation::Mulu, Reg(ra), BregDeref(brb)) => Ok(i2b(MuluRaBrb(*ra, *brb))),
        (Operation::Muli, Reg(ra), BregDeref(brb)) => Ok(i2b(MuliRaBrb(*ra, *brb))),
        (Operation::Divu, Reg(ra), BregDeref(brb)) => Ok(i2b(DivuRaBrb(*ra, *brb))),
        (Operation::Divi, Reg(ra), BregDeref(brb)) => Ok(i2b(DiviRaBrb(*ra, *brb))),
        (Operation::Mulu, Reg(ra), Word(w)) => Ok(i2b_imm16(MuluRaImm16(*ra), *w)),
        (Operation::Muli, Reg(ra), Word(w)) => Ok(i2b_imm16(MuliRaImm16(*ra), *w)),
        (Operation::Divu, Reg(ra), Word(w)) => Ok(i2b_imm16(DivuRaImm16(*ra), *w)),
        (Operation::Divi, Reg(ra), Word(w)) => Ok(i2b_imm16(DiviRaImm16(*ra), *w)),
        (Operation::Mulu, Breg(bra), DWord(d)) => Ok(i2b_imm32(MuluBraImm32(*bra), *d)),
        (Operation::Muli, Breg(bra), DWord(d)) => Ok(i2b_imm32(MuliBraImm32(*bra), *d)),
        (Operation::Divu, Breg(bra), DWord(d)) => Ok(i2b_imm32(DivuBraImm32(*bra), *d)),
        (Operation::Divi, Breg(bra), DWord(d)) => Ok(i2b_imm32(DiviBraImm32(*bra), *d)),
        (Operation::Mulu, Vreg(vra), Byte(b)) => Ok(i2b_imm8(MuluVraImm8(*vra), *b)),
        (Operation::Muli, Vreg(vra), Byte(b)) => Ok(i2b_imm8(MuliVraImm8(*vra), *b)),
        (Operation::Divu, Vreg(vra), Byte(b)) => Ok(i2b_imm8(DivuVraImm8(*vra), *b)),
        (Operation::Divi, Vreg(vra), Byte(b)) => Ok(i2b_imm8(DiviVraImm8(*vra), *b)),
        (Operation::Rand, Reg(ra), None) => Ok(i2b(RandRa(*ra))),
        (Operation::Rand, Breg(bra), None) => Ok(i2b(RandBra(*bra))),
        (Operation::Rand, Vreg(vra), None) => Ok(i2b(RandVra(*vra))),
        (Operation::Jp, DWord(d), None) => Ok(i2b_imm32(JpImm32, *d)),
        (Operation::Jr, DWord(d), None) => Ok(i2b_imm32(JrImm32, *d)),
        (Operation::Jpz, DWord(d), None) => Ok(i2b_imm32(JpzImm32, *d)),
        (Operation::Jnz, DWord(d), None) => Ok(i2b_imm32(JnzImm32, *d)),
        (Operation::Jpc, DWord(d), None) => Ok(i2b_imm32(JpcImm32, *d)),
        (Operation::Jnc, DWord(d), None) => Ok(i2b_imm32(JncImm32, *d)),
        (Operation::Jpo, DWord(d), None) => Ok(i2b_imm32(JpoImm32, *d)),
        (Operation::Jno, DWord(d), None) => Ok(i2b_imm32(JnoImm32, *d)),
        (Operation::Jpp, DWord(d), None) => Ok(i2b_imm32(JppImm32, *d)),
        (Operation::Jnp, DWord(d), None) => Ok(i2b_imm32(JnpImm32, *d)),
        (Operation::Jpn, DWord(d), None) => Ok(i2b_imm32(JpnImm32, *d)),
        (Operation::Jnn, DWord(d), None) => Ok(i2b_imm32(JnnImm32, *d)),
        (Operation::Jp, Breg(bra), None) => Ok(i2b(JpBra(*bra))),
        (Operation::Jr, Breg(bra), None) => Ok(i2b(JrBra(*bra))),
        (Operation::Jpz, Breg(bra), None) => Ok(i2b(JpzBra(*bra))),
        (Operation::Jnz, Breg(bra), None) => Ok(i2b(JnzBra(*bra))),
        (Operation::Jpc, Breg(bra), None) => Ok(i2b(JpcBra(*bra))),
        (Operation::Jnc, Breg(bra), None) => Ok(i2b(JncBra(*bra))),
        (Operation::Jpo, Breg(bra), None) => Ok(i2b(JpoBra(*bra))),
        (Operation::Jno, Breg(bra), None) => Ok(i2b(JnoBra(*bra))),
        (Operation::Jpp, Breg(bra), None) => Ok(i2b(JppBra(*bra))),
        (Operation::Jnp, Breg(bra), None) => Ok(i2b(JnpBra(*bra))),
        (Operation::Jpn, Breg(bra), None) => Ok(i2b(JpnBra(*bra))),
        (Operation::Jnn, Breg(bra), None) => Ok(i2b(JnnBra(*bra))),
        (Operation::Call, DWord(d), None) => Ok(i2b_imm32(CallImm32, *d)),
        (Operation::Clz, DWord(d), None) => Ok(i2b_imm32(ClzImm32, *d)),
        (Operation::Cnz, DWord(d), None) => Ok(i2b_imm32(CnzImm32, *d)),
        (Operation::Clc, DWord(d), None) => Ok(i2b_imm32(ClcImm32, *d)),
        (Operation::Cnc, DWord(d), None) => Ok(i2b_imm32(CncImm32, *d)),
        (Operation::Clo, DWord(d), None) => Ok(i2b_imm32(CloImm32, *d)),
        (Operation::Cno, DWord(d), None) => Ok(i2b_imm32(CnoImm32, *d)),
        (Operation::Clp, DWord(d), None) => Ok(i2b_imm32(ClpImm32, *d)),
        (Operation::Cnp, DWord(d), None) => Ok(i2b_imm32(CnpImm32, *d)),
        (Operation::Cln, DWord(d), None) => Ok(i2b_imm32(ClnImm32, *d)),
        (Operation::Cnn, DWord(d), None) => Ok(i2b_imm32(CnnImm32, *d)),
        (Operation::Call, Breg(bra), None) => Ok(i2b(CallBra(*bra))),
        (Operation::Ret, None, None) => Ok(i2b(Ret)),
        (Operation::Rtz, None, None) => Ok(i2b(Rtz)),
        (Operation::Rnz, None, None) => Ok(i2b(Rnz)),
        (Operation::Rtc, None, None) => Ok(i2b(Rtc)),
        (Operation::Rnc, None, None) => Ok(i2b(Rnc)),
        (Operation::Rto, None, None) => Ok(i2b(Rto)),
        (Operation::Rno, None, None) => Ok(i2b(Rno)),
        (Operation::Rtp, None, None) => Ok(i2b(Rtp)),
        (Operation::Rnp, None, None) => Ok(i2b(Rnp)),
        (Operation::Rtn, None, None) => Ok(i2b(Rtn)),
        (Operation::Rnn, None, None) => Ok(i2b(Rnn)),
        (Operation::Reti, None, None) => Ok(i2b(Reti)),
        (Operation::Clz, Breg(bra), None) => Ok(i2b(ClzBra(*bra))),
        (Operation::Cnz, Breg(bra), None) => Ok(i2b(CnzBra(*bra))),
        (Operation::Clc, Breg(bra), None) => Ok(i2b(ClcBra(*bra))),
        (Operation::Cnc, Breg(bra), None) => Ok(i2b(CncBra(*bra))),
        (Operation::Clo, Breg(bra), None) => Ok(i2b(CloBra(*bra))),
        (Operation::Cno, Breg(bra), None) => Ok(i2b(CnoBra(*bra))),
        (Operation::Clp, Breg(bra), None) => Ok(i2b(ClpBra(*bra))),
        (Operation::Cnp, Breg(bra), None) => Ok(i2b(CnpBra(*bra))),
        (Operation::Cln, Breg(bra), None) => Ok(i2b(ClnBra(*bra))),
        (Operation::Cnn, Breg(bra), None) => Ok(i2b(CnnBra(*bra))),
        (Operation::Push, Breg(bra), None) => Ok(i2b(PushBra(*bra))),
        (Operation::Pop, Breg(bra), None) => Ok(i2b(PopBra(*bra))),
        (Operation::Peek, Breg(bra), None) => Ok(i2b(PeekBra(*bra))),
        (Operation::Push, DWord(d), None) => Ok(i2b_imm32(PushImm32, *d)),
        (Operation::Clv, None, None) => Ok(i2b(Clv)),
        (Operation::Stop, None, None) => Ok(i2b(Stop)),
        (Operation::Ei, None, None) => Ok(i2b(Ei)),
        (Operation::Di, None, None) => Ok(i2b(Di)),
        (Operation::Halt, None, None) => Ok(i2b(Halt)),
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
