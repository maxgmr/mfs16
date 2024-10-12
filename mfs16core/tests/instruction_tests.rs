use mfs16core::{Computer, Flag::*, Reg16::*, Reg8::*};
use pretty_assertions::assert_eq;

#[test]
fn test_ld() {
    let mut c = Computer::default();
    let flags = c.cpu.flags.clone();
    // LD A,B
    c.ram.write_word(0x00_0000, 0x0101);
    c.cpu.regs.set_reg(A, 0x1234);
    c.cpu.regs.set_reg(B, 0x5678);

    assert_eq!(c.cpu.regs.reg(A), 0x1234);
    assert_eq!(c.cpu.regs.reg(B), 0x5678);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.regs.reg(A), 0x1234);
    assert_eq!(c.cpu.regs.reg(B), 0x5678);

    // Perform operation
    c.cycle();
    assert_eq!(c.cpu.regs.reg(A), 0x5678);
    assert_eq!(c.cpu.regs.reg(B), 0x5678);
    assert_eq!(c.cpu.flags, flags);

    // LD HH,EL
    c.ram.write_word(0x00_0002, 0x02A9);
    c.cpu.regs.set_vreg(HH, 0xFF);
    c.cpu.regs.set_vreg(EL, 0x34);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.regs.vreg(HH), 0xFF);
    assert_eq!(c.cpu.regs.vreg(EL), 0x34);

    // Perform operation
    c.cycle();
    assert_eq!(c.cpu.regs.vreg(HH), 0x34);
    assert_eq!(c.cpu.regs.vreg(EL), 0x34);
    assert_eq!(c.cpu.flags, flags);
}
