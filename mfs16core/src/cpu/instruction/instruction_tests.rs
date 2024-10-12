use pretty_assertions::assert_eq;

use super::{Flag::*, Reg16::*, Reg8::*, *};

/// Simulate a [Cpu] at the given step num
fn new_test_cpu(step_num: u32) -> Cpu {
    Cpu {
        step_num,
        ..Cpu::default()
    }
}

#[test]
fn test_ld_ra_rb() {
    let mut cpu = new_test_cpu(1);
    cpu.regs.set_reg(A, 0x1234);
    cpu.regs.set_reg(B, 0x5678);

    ld_ra_rb(&mut cpu, A, B);

    assert_eq!(cpu.regs.reg(A), cpu.regs.reg(B));
    assert_eq!(cpu.regs.reg(A), 0x5678);
    assert_eq!(cpu.regs.reg(B), 0x5678);
}
