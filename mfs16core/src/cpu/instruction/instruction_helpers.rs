use super::*;
use crate::{
    helpers::{change_bit, combine_u8_le, split_word, test_bit, BitOp},
    Addr, Flag, Flags,
};

/// Perform the current step of the current CPU instruction.
pub fn step(cpu: &mut Cpu, mmu: &mut Mmu) {
    match cpu.instr {
        Nop => {}
        LdRaRb(ra, rb) => ld_ra_rb(cpu, ra, rb),
        LdBraBrb(bra, brb) => ld_bra_brb(cpu, bra, brb),
        LdSpImm32 => ld_sp_imm32(cpu, mmu),
        LdImm32Sp => ld_imm32_sp(cpu, mmu),
        LdSpBra(bra) => ld_sp_bra(cpu, bra),
        LdBraSp(bra) => ld_bra_sp(cpu, bra),
        LdVraVrb(vra, vrb) => ld_vra_vrb(cpu, vra, vrb),
        LdRaImm16(ra) => ld_ra_imm16(cpu, mmu, ra),
        LdBraImm32(bra) => ld_bra_imm32(cpu, mmu, bra),
        LdVraImm8(vra) => ld_vra_imm8(cpu, mmu, vra),
        LdBraImm16(bra) => ld_bra_imm16(cpu, mmu, bra),
        LdBraRb(bra, rb) => ld_bra_rb(cpu, mmu, bra, rb),
        LdRaBrb(ra, brb) => ld_ra_brb(cpu, mmu, ra, brb),
        LdrRaImm32(ra) => ldr_ra_imm32(cpu, mmu, ra),
        LdiBraRb(bra, rb) => ldi_bra_rb(cpu, mmu, bra, rb),
        LddBraRb(bra, rb) => ldd_bra_rb(cpu, mmu, bra, rb),
        LdiRaBrb(ra, brb) => ldi_ra_brb(cpu, mmu, ra, brb),
        LddRaBrb(ra, brb) => ldd_ra_brb(cpu, mmu, ra, brb),
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
        AddRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, Add),
        AdcRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, Adc),
        AddBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, Add),
        AdcBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, Adc),
        AddVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, Add),
        AdcVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, Adc),
        SubRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, Sub),
        SbbRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, Sbb),
        SubBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, Sub),
        SbbBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, Sbb),
        SubVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, Sub),
        SbbVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, Sbb),
        AddRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, Add),
        AdcRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, Adc),
        SubRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, Sub),
        SbbRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, Sbb),
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
        PssImm16 => pss_imm16(cpu, mmu),
        PssImm32 => pss_imm32(cpu, mmu),
        PssImm8 => pss_imm8(cpu, mmu),
        AndRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, And),
        AndBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, And),
        AndVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, And),
        AndRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, And),
        OrRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Or),
        OrBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Or),
        OrVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Or),
        OrRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, Or),
        XorRaRb(ra, rb) => alu_ra_rb(cpu, ra, rb, Xor),
        XorBraBrb(bra, brb) => alu_bra_brb(cpu, bra, brb, Xor),
        XorVraVrb(vra, vrb) => alu_vra_vrb(cpu, vra, vrb, Xor),
        XorRaBrb(ra, brb) => alu_ra_brb(cpu, mmu, ra, brb, Xor),
        AndRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, And),
        AndBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, And),
        AndVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, And),
        OrRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, Or),
        OrBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, Or),
        OrVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, Or),
        XorRaImm16(ra) => alu_ra_imm16(cpu, mmu, ra, Xor),
        XorBraImm32(bra) => alu_bra_imm32(cpu, mmu, bra, Xor),
        XorVraImm8(vra) => alu_vra_imm8(cpu, mmu, vra, Xor),
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
        CmpRaRb(ra, rb) => cmp_ra_rb(cpu, ra, rb),
        CmpBraBrb(bra, brb) => cmp_bra_brb(cpu, bra, brb),
        CmpVraVrb(vra, vrb) => cmp_vra_vrb(cpu, vra, vrb),
        CmpRaImm16(ra) => cmp_ra_imm16(cpu, mmu, ra),
        CmpBraImm32(bra) => cmp_bra_imm32(cpu, mmu, bra),
        CmpVraImm8(vra) => cmp_vra_imm8(cpu, mmu, vra),
        CmpImm16Ra(ra) => cmp_imm16_ra(cpu, mmu, ra),
        CmpImm32Bra(bra) => cmp_imm32_bra(cpu, mmu, bra),
        CmpImm8Vra(vra) => cmp_imm8_vra(cpu, mmu, vra),
        CmpRaBrb(ra, brb) => cmp_ra_brb(cpu, mmu, ra, brb),
        CmpBraRb(bra, rb) => cmp_bra_rb(cpu, mmu, bra, rb),
        BitRaB(ra, b) => bit_ra_b(cpu, ra, b),
        BitBraB(bra, b) => bit_bra_b(cpu, mmu, bra, b),
        StbRaB(ra, b) => bit_op_ra_b(cpu, ra, b, BitOp::Set),
        StbBraB(bra, b) => bit_op_bra_b(cpu, mmu, bra, b, BitOp::Set),
        RsbRaB(ra, b) => bit_op_ra_b(cpu, ra, b, BitOp::Reset),
        RsbBraB(bra, b) => bit_op_bra_b(cpu, mmu, bra, b, BitOp::Reset),
        TgbRaB(ra, b) => bit_op_ra_b(cpu, ra, b, BitOp::Toggle),
        TgbBraB(bra, b) => bit_op_bra_b(cpu, mmu, bra, b, BitOp::Toggle),
        SwpRa(ra) => swp_ra(cpu, ra),
        SwpBra(bra) => swp_bra(cpu, mmu, bra),
        Szf => set_flag(cpu, Flag::Zero),
        Rzf => reset_flag(cpu, Flag::Zero),
        Tzf => toggle_flag(cpu, Flag::Zero),
        Scf => set_flag(cpu, Flag::Carry),
        Rcf => reset_flag(cpu, Flag::Carry),
        Tcf => toggle_flag(cpu, Flag::Carry),
        Sof => set_flag(cpu, Flag::Overflow),
        Rof => reset_flag(cpu, Flag::Overflow),
        Tof => toggle_flag(cpu, Flag::Overflow),
        Spf => set_flag(cpu, Flag::Parity),
        Rpf => reset_flag(cpu, Flag::Parity),
        Tpf => toggle_flag(cpu, Flag::Parity),
        Snf => set_flag(cpu, Flag::Negative),
        Rnf => reset_flag(cpu, Flag::Negative),
        Tnf => toggle_flag(cpu, Flag::Negative),
        Saf => set_all_flags(cpu),
        Raf => reset_all_flags(cpu),
        JpImm32 => jp_imm32(cpu, mmu),
        JrImm32 => jr_imm32(cpu, mmu),
        JpzImm32 => cond_jump_imm32(cpu, mmu, Flag::Zero, true),
        JnzImm32 => cond_jump_imm32(cpu, mmu, Flag::Zero, false),
        JpcImm32 => cond_jump_imm32(cpu, mmu, Flag::Carry, true),
        JncImm32 => cond_jump_imm32(cpu, mmu, Flag::Carry, false),
        JpoImm32 => cond_jump_imm32(cpu, mmu, Flag::Overflow, true),
        JnoImm32 => cond_jump_imm32(cpu, mmu, Flag::Overflow, false),
        JppImm32 => cond_jump_imm32(cpu, mmu, Flag::Parity, true),
        JnpImm32 => cond_jump_imm32(cpu, mmu, Flag::Parity, false),
        JpnImm32 => cond_jump_imm32(cpu, mmu, Flag::Negative, true),
        JnnImm32 => cond_jump_imm32(cpu, mmu, Flag::Negative, false),
        JpBra(bra) => jp_bra(cpu, bra),
        JrBra(bra) => jr_bra(cpu, bra),
        JpzBra(bra) => cond_jump_bra(cpu, bra, Flag::Zero, true),
        JnzBra(bra) => cond_jump_bra(cpu, bra, Flag::Zero, false),
        JpcBra(bra) => cond_jump_bra(cpu, bra, Flag::Carry, true),
        JncBra(bra) => cond_jump_bra(cpu, bra, Flag::Carry, false),
        JpoBra(bra) => cond_jump_bra(cpu, bra, Flag::Overflow, true),
        JnoBra(bra) => cond_jump_bra(cpu, bra, Flag::Overflow, false),
        JppBra(bra) => cond_jump_bra(cpu, bra, Flag::Parity, true),
        JnpBra(bra) => cond_jump_bra(cpu, bra, Flag::Parity, false),
        JpnBra(bra) => cond_jump_bra(cpu, bra, Flag::Negative, true),
        JnnBra(bra) => cond_jump_bra(cpu, bra, Flag::Negative, false),
        CallImm32 => call_imm32(cpu, mmu),
        ClzImm32 => cond_call_imm32(cpu, mmu, Flag::Zero, true),
        CnzImm32 => cond_call_imm32(cpu, mmu, Flag::Zero, false),
        ClcImm32 => cond_call_imm32(cpu, mmu, Flag::Carry, true),
        CncImm32 => cond_call_imm32(cpu, mmu, Flag::Carry, false),
        CloImm32 => cond_call_imm32(cpu, mmu, Flag::Overflow, true),
        CnoImm32 => cond_call_imm32(cpu, mmu, Flag::Overflow, false),
        ClpImm32 => cond_call_imm32(cpu, mmu, Flag::Parity, true),
        CnpImm32 => cond_call_imm32(cpu, mmu, Flag::Parity, false),
        ClnImm32 => cond_call_imm32(cpu, mmu, Flag::Negative, true),
        CnnImm32 => cond_call_imm32(cpu, mmu, Flag::Negative, false),
        CallBra(bra) => call_bra(cpu, mmu, bra),
        Ret => ret(cpu, mmu),
        Rtz => cond_ret(cpu, mmu, Flag::Zero, true),
        Rnz => cond_ret(cpu, mmu, Flag::Zero, false),
        Rtc => cond_ret(cpu, mmu, Flag::Carry, true),
        Rnc => cond_ret(cpu, mmu, Flag::Carry, false),
        Rto => cond_ret(cpu, mmu, Flag::Overflow, true),
        Rno => cond_ret(cpu, mmu, Flag::Overflow, false),
        Rtp => cond_ret(cpu, mmu, Flag::Parity, true),
        Rnp => cond_ret(cpu, mmu, Flag::Parity, false),
        Rtn => cond_ret(cpu, mmu, Flag::Negative, true),
        Rnn => cond_ret(cpu, mmu, Flag::Negative, false),
        ClzBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Zero, true),
        CnzBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Zero, false),
        ClcBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Carry, true),
        CncBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Carry, false),
        CloBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Overflow, true),
        CnoBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Overflow, false),
        ClpBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Parity, true),
        CnpBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Parity, false),
        ClnBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Negative, true),
        CnnBra(bra) => cond_call_bra(cpu, mmu, bra, Flag::Negative, false),
        PushBra(bra) => push_bra(cpu, mmu, bra),
        PopBra(bra) => pop_bra(cpu, mmu, bra),
        PeekBra(bra) => peek_bra(cpu, mmu, bra),
        PushImm32 => push_imm32(cpu, mmu),
        Halt => halt(cpu),
    }
}

/// Panic with an error message when an unexpected step is encountered.
pub fn invalid_step_panic(instr: Instruction, step_num: u32) {
    panic!(
        "Invalid step number {} for instruction {} ({} steps)",
        step_num,
        instr,
        instr.num_steps()
    );
}

/// Assemble an instruction and its argument into an opcode.
pub fn opc_1arg<T: Into<u16>, U: Into<u16>>(instr: T, a: U) -> u16 {
    (instr.into() << 4) | a.into()
}

/// Assemble an instruction and its argument, offset by a given value, into an opcode.
pub fn opc_1arg_off<T: Into<u16>, U: Into<u16>>(instr: T, a: U, offset: u16) -> u16 {
    opc_1arg(instr, a.into() + offset)
}

/// Assemble an instruction and its two arguments into an opcode.
pub fn opc_2arg<T: Into<u16>, U: Into<u16>, V: Into<u16>>(instr: T, a: U, b: V) -> u16 {
    (instr.into() << 8) | (a.into() << 4) | b.into()
}

/// Assemble an instruction and its two arguments, offset by a given value, into an opcode.
pub fn opc_2arg_off<T: Into<u16>, U: Into<u16>, V: Into<u16>>(
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

fn ld_sp_imm32(cpu: &mut Cpu, mmu: &mut Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.sp = Addr::new_default_range(get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_imm32_sp(cpu: &mut Cpu, mmu: &mut Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => write_dword_to_last(cpu, mmu, cpu.sp.address()),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_sp_bra(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.sp = Addr::new_default_range(cpu.breg(bra)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_sp(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.set_breg(bra, cpu.sp.address()),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_vra_vrb(cpu: &mut Cpu, vra: Reg8, vrb: Reg8) {
    match cpu.step_num {
        1 => cpu.set_vreg(vra, cpu.vreg(vrb)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_ra_imm16(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.set_reg(ra, cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_imm16(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => mmu.write_word(cpu.breg(bra), cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_imm32(cpu: &mut Cpu, mmu: &Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.set_breg(bra, get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_vra_imm8(cpu: &mut Cpu, mmu: &Mmu, vra: Reg8) {
    match cpu.step_num {
        1 => cpu.read_next_byte(mmu),
        2 => cpu.set_vreg(vra, cpu.last_byte),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_bra_rb(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => mmu.write_word(cpu.breg(bra), cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ld_ra_brb(cpu: &mut Cpu, mmu: &mut Mmu, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(brb)),
        2 => cpu.set_reg(ra, cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldr_ra_imm32(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => {
            let mut addr = Addr::new_default_range(get_dword_from_last(cpu));
            addr.wrapping_add(cpu.breg(Reg32::HL));
            cpu.read_word_at_addr(mmu, addr.into());
        }
        4 => cpu.set_reg(ra, cpu.last_word),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldi_bra_rb(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => {
            mmu.write_word(cpu.breg(bra), cpu.last_word);
            dbl_inc_addr(cpu, bra);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldd_bra_rb(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.update_last_word(cpu.reg(rb)),
        2 => {
            mmu.write_word(cpu.breg(bra), cpu.last_word);
            dbl_dec_addr(cpu, bra);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldi_ra_brb(cpu: &mut Cpu, mmu: &mut Mmu, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(brb)),
        2 => {
            cpu.set_reg(ra, cpu.last_word);
            dbl_inc_addr(cpu, brb);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ldd_ra_brb(cpu: &mut Cpu, mmu: &mut Mmu, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(brb)),
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

fn alu_ra_imm16(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => {
            let a = cpu.reg(ra);
            let b = cpu.last_word;
            let result = alu(cpu, operation, a, b);
            cpu.set_reg(ra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_bra_imm32(cpu: &mut Cpu, mmu: &Mmu, bra: Reg32, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => {
            let a = cpu.breg(bra);
            let b = get_dword_from_last(cpu);
            let result = alu(cpu, operation, a, b);
            cpu.set_breg(bra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_vra_imm8(cpu: &mut Cpu, mmu: &Mmu, vra: Reg8, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_next_byte(mmu),
        2 => {
            let a = cpu.vreg(vra);
            let b = cpu.last_byte;
            let result = alu(cpu, operation, a, b);
            cpu.set_vreg(vra, result);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn alu_ra_brb(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16, brb: Reg32, operation: AluOp) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(brb)),
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

fn pss_imm16(cpu: &mut Cpu, mmu: &Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => {
            let a = cpu.last_word;
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_imm32(cpu: &mut Cpu, mmu: &Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => {
            let a = get_dword_from_last(cpu);
            alu(cpu, Pss, a, 0);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pss_imm8(cpu: &mut Cpu, mmu: &Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_byte(mmu),
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

fn cmp_ra_rb(cpu: &mut Cpu, ra: Reg16, rb: Reg16) {
    match cpu.step_num {
        1 => {
            let a = cpu.reg(ra);
            let b = cpu.reg(rb);
            alu::<u16>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_bra_brb(cpu: &mut Cpu, bra: Reg32, brb: Reg32) {
    match cpu.step_num {
        1 => {
            let a = cpu.breg(bra);
            let b = cpu.breg(brb);
            alu::<u32>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_vra_vrb(cpu: &mut Cpu, vra: Reg8, vrb: Reg8) {
    match cpu.step_num {
        1 => {
            let a = cpu.vreg(vra);
            let b = cpu.vreg(vrb);
            alu::<u8>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_ra_imm16(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => {
            let a = cpu.reg(ra);
            let b = cpu.last_word;
            alu::<u16>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_bra_imm32(cpu: &mut Cpu, mmu: &Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => {
            let a = cpu.breg(bra);
            let b = get_dword_from_last(cpu);
            alu::<u32>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_vra_imm8(cpu: &mut Cpu, mmu: &Mmu, vra: Reg8) {
    match cpu.step_num {
        1 => cpu.read_next_byte(mmu),
        2 => {
            let a = cpu.vreg(vra);
            let b = cpu.last_byte;
            alu::<u8>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_imm16_ra(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => {
            let a = cpu.last_word;
            let b = cpu.reg(ra);
            alu::<u16>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_imm32_bra(cpu: &mut Cpu, mmu: &Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => {
            let a = get_dword_from_last(cpu);
            let b = cpu.breg(bra);
            alu::<u32>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_imm8_vra(cpu: &mut Cpu, mmu: &Mmu, vra: Reg8) {
    match cpu.step_num {
        1 => cpu.read_next_byte(mmu),
        2 => {
            let a = cpu.last_byte;
            let b = cpu.vreg(vra);
            alu::<u8>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_ra_brb(cpu: &mut Cpu, mmu: &Mmu, ra: Reg16, brb: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(brb)),
        2 => {
            let a = cpu.reg(ra);
            let b = cpu.last_word;
            alu::<u16>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cmp_bra_rb(cpu: &mut Cpu, mmu: &Mmu, bra: Reg32, rb: Reg16) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(bra)),
        2 => {
            let a = cpu.last_word;
            let b = cpu.reg(rb);
            alu::<u16>(cpu, Sub, a, b);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn bit_ra_b(cpu: &mut Cpu, ra: Reg16, b: u8) {
    match cpu.step_num {
        1 => cpu.change_flag(Flag::Zero, !test_bit(cpu.reg(ra), b as u16)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn bit_bra_b(cpu: &mut Cpu, mmu: &Mmu, bra: Reg32, b: u8) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(bra)),
        2 => cpu.change_flag(Flag::Zero, !test_bit(cpu.last_word, b as u16)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn bit_op_ra_b(cpu: &mut Cpu, ra: Reg16, b: u8, bit_op: BitOp) {
    match cpu.step_num {
        1 => cpu.set_reg(ra, change_bit(cpu.reg(ra), b, bit_op)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn bit_op_bra_b(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32, b: u8, bit_op: BitOp) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(bra)),
        2 => mmu.write_word(cpu.breg(bra), change_bit(cpu.last_word, b, bit_op)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn swp_ra(cpu: &mut Cpu, ra: Reg16) {
    match cpu.step_num {
        1 => {
            let (msb, lsb) = split_word(cpu.reg(ra));
            cpu.set_reg(ra, combine_u8_le(msb, lsb));
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn swp_bra(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.read_word_at_addr(mmu, cpu.breg(bra)),
        2 => {
            let (msb, lsb) = split_word(cpu.last_word);
            mmu.write_word(cpu.breg(bra), combine_u8_le(msb, lsb));
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn set_flag(cpu: &mut Cpu, flag: Flag) {
    match cpu.step_num {
        1 => cpu.set_flag(flag),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn reset_flag(cpu: &mut Cpu, flag: Flag) {
    match cpu.step_num {
        1 => cpu.reset_flag(flag),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn toggle_flag(cpu: &mut Cpu, flag: Flag) {
    match cpu.step_num {
        1 => cpu.change_flag(flag, !cpu.flag(flag)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn set_all_flags(cpu: &mut Cpu) {
    match cpu.step_num {
        1 => cpu.flags = Flags::from_string("ZCOPN"),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn reset_all_flags(cpu: &mut Cpu) {
    match cpu.step_num {
        1 => cpu.flags = Flags::from_string(""),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn jp_imm32(cpu: &mut Cpu, mmu: &Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.jump(get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn jr_imm32(cpu: &mut Cpu, mmu: &Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.relative_jump(get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cond_jump_imm32(cpu: &mut Cpu, mmu: &Mmu, flag: Flag, expected: bool) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.check_conditional(flag, expected),
        4 => {
            if cpu.last_conditional_satisfied {
                cpu.jump(get_dword_from_last(cpu))
            }
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn jp_bra(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.jump(cpu.breg(bra)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn jr_bra(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.relative_jump(cpu.breg(bra)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cond_jump_bra(cpu: &mut Cpu, bra: Reg32, flag: Flag, expected: bool) {
    match cpu.step_num {
        1 => cpu.check_conditional(flag, expected),
        2 => {
            if cpu.last_conditional_satisfied {
                cpu.jump(cpu.breg(bra));
            }
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn call_imm32(cpu: &mut Cpu, mmu: &mut Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.push_stack(mmu, cpu.pc.address()),
        4 => cpu.jump(get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cond_call_imm32(cpu: &mut Cpu, mmu: &mut Mmu, flag: Flag, expected: bool) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.check_conditional(flag, expected),
        4 => {
            if cpu.last_conditional_satisfied {
                cpu.push_stack(mmu, cpu.pc.address());
            }
        }
        5 => {
            if cpu.last_conditional_satisfied {
                cpu.jump(get_dword_from_last(cpu));
            }
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn call_bra(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.push_stack(mmu, cpu.pc.address()),
        2 => cpu.jump(cpu.breg(bra)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn ret(cpu: &mut Cpu, mmu: &mut Mmu) {
    match cpu.step_num {
        1 => cpu.pc = Addr::new_default_range(cpu.pop_stack(mmu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cond_ret(cpu: &mut Cpu, mmu: &mut Mmu, flag: Flag, expected: bool) {
    match cpu.step_num {
        1 => cpu.check_conditional(flag, expected),
        2 => {
            if cpu.last_conditional_satisfied {
                cpu.pc = Addr::new_default_range(cpu.pop_stack(mmu));
            }
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn cond_call_bra(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32, flag: Flag, expected: bool) {
    match cpu.step_num {
        1 => cpu.check_conditional(flag, expected),
        2 => {
            if cpu.last_conditional_satisfied {
                cpu.push_stack(mmu, cpu.pc.address());
            }
        }
        3 => {
            if cpu.last_conditional_satisfied {
                cpu.jump(cpu.breg(bra));
            }
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn push_bra(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.push_stack(mmu, cpu.breg(bra)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn pop_bra(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => {
            let popped_val = cpu.pop_stack(mmu);
            cpu.set_breg(bra, popped_val);
        }
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn peek_bra(cpu: &mut Cpu, mmu: &mut Mmu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.set_breg(bra, mmu.read_dword(cpu.sp.address())),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn push_imm32(cpu: &mut Cpu, mmu: &mut Mmu) {
    match cpu.step_num {
        1 => cpu.read_next_word(mmu),
        2 => cpu.read_next_word(mmu),
        3 => cpu.push_stack(mmu, get_dword_from_last(cpu)),
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}

fn halt(cpu: &mut Cpu) {
    match cpu.step_num {
        1 => cpu.is_halted = true,
        _ => invalid_step_panic(cpu.instr, cpu.step_num),
    }
}
