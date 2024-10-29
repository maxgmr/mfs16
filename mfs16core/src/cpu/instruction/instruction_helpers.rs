use super::*;

/// Perform the current step of the current CPU instruction.
pub fn step(cpu: &mut Cpu, ram: &mut Ram) {
    match cpu.instr {
        Nop => {}
        LdRaRb(ra, rb) => ld_ra_rb(cpu, ra, rb),
        LdBraBrb(bra, brb) => ld_bra_brb(cpu, bra, brb),
        LdSpImm32 => ld_sp_imm32(cpu, ram),
        LdImm32Sp => ld_imm32_sp(cpu, ram),
        LdSpBra(bra) => ld_sp_bra(cpu, bra),
        LdBraSp(bra) => ld_bra_sp(cpu, bra),
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

/// Assemble an instruction and its two arguments into an opcode.
pub fn opc_2arg<T: Into<u16>, U: Into<u16>, V: Into<u16>>(instr: T, a: U, b: V) -> u16 {
    (instr.into() << 8) | (a.into() << 4) | b.into()
}

/// Assemble an instruction and its two arguments, offset by a certain value, into an opcode.
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

fn ld_bra_sp(cpu: &mut Cpu, bra: Reg32) {
    match cpu.step_num {
        1 => cpu.set_breg(bra, cpu.sp),
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
