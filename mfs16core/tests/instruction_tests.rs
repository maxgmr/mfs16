use mfs16core::{Computer, Flag::*, Reg16::*, Reg8::*};

#[test]
fn test_ld_ra_rb() {
    let mut c = Computer::default();
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
}
