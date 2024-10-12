use pretty_assertions::assert_eq;

use super::{super::pc::Pc, Flag::*, Reg16::*, Reg8::*, *};

/// Simulate a [Cpu] after reading first instruction
fn new_test_cpu() -> Cpu {
    Cpu {
        step_num: 1,
        pc: Pc::new(0x00_0002),
        ..Cpu::default()
    }
}

#[test]
fn test_ld_ra_rb() {
    let mut cpu = new_test_cpu();
    cpu.set_reg(A, 0x1234);
    cpu.set_reg(B, 0x5678);

    ld_ra_rb(&mut cpu, A, B);

    assert_eq!(cpu.reg(A), cpu.regs.reg(B));
    assert_eq!(cpu.reg(A), 0x5678);
    assert_eq!(cpu.reg(B), 0x5678);
}

#[test]
fn test_ld_vra_vrb() {
    let mut cpu = new_test_cpu();
    cpu.set_vreg(AL, 0x12);
    cpu.set_vreg(LH, 0x34);

    ld_vra_vrb(&mut cpu, LH, AL);

    assert_eq!(cpu.vreg(LH), 0x12);
    assert_eq!(cpu.vreg(AL), 0x12);
}

#[test]
fn test_ld_ra_imm() {
    let mut cpu = new_test_cpu();
    let mut ram = Ram::default();
    ram.write_word(cpu.pc.into(), 0xFEDC);

    ld_ra_imm16(&mut cpu, &ram, B);
    assert_eq!(cpu.pc, Pc::new(0x00_0004));
    assert_eq!(cpu.last_word, 0xFEDC);
    assert_eq!(cpu.reg(B), 0x0000);
    cpu.step_num += 1;

    ld_ra_imm16(&mut cpu, &ram, B);
    assert_eq!(cpu.pc, Pc::new(0x00_0004));
    assert_eq!(cpu.reg(B), 0xFEDC);
    cpu.step_num = 1;

    ram.write_byte(cpu.pc.into(), 0x8D);

    ld_vra_imm8(&mut cpu, &ram, CH);
    assert_eq!(cpu.pc, Pc::new(0x00_0005));
    assert_eq!(cpu.last_byte, 0x8D);
    assert_eq!(cpu.vreg(CH), 0x00);
    cpu.step_num += 1;

    ld_vra_imm8(&mut cpu, &ram, CH);
    assert_eq!(cpu.pc, Pc::new(0x00_0005));
    assert_eq!(cpu.vreg(CH), 0x8D);
}
