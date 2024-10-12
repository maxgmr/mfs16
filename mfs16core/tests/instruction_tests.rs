use mfs16core::{Computer, Flag::*, Pc, Reg16::*, Reg32::*, Reg8::*};
use pretty_assertions::assert_eq;

#[test]
fn test_ld() {
    let mut c = Computer::default();
    let flags = c.cpu.flags.clone();
    // LD A,B
    c.ram.write_word(0x00_0000, 0x0101);
    c.cpu.regs.set_reg(A, 0x1234);
    c.cpu.regs.set_reg(B, 0x5678);

    assert_eq!(c.cpu.reg(A), 0x1234);
    assert_eq!(c.cpu.reg(B), 0x5678);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.reg(A), 0x1234);
    assert_eq!(c.cpu.reg(B), 0x5678);

    // Perform operation
    c.cycle();
    assert_eq!(c.cpu.reg(A), 0x5678);
    assert_eq!(c.cpu.reg(B), 0x5678);
    assert_eq!(c.cpu.flags, flags);

    // LD H1,E0
    c.ram.write_word(0x00_0002, 0x02A9);
    c.cpu.set_vreg(H1, 0xFF);
    c.cpu.set_vreg(E0, 0x34);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.vreg(H1), 0xFF);
    assert_eq!(c.cpu.vreg(E0), 0x34);

    // Perform operation
    c.cycle();
    assert_eq!(c.cpu.vreg(H1), 0x34);
    assert_eq!(c.cpu.vreg(E0), 0x34);
    assert_eq!(c.cpu.flags, flags);

    // LD E,0x3865
    c.ram.write_word(0x00_0004, 0x0304);
    c.ram.write_word(0x00_0006, 0x3865);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.reg(E), 0x0034);

    // Read immediate word
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(E), 0x0034);

    // Write immediate word into register
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(E), 0x3865);

    // LD E1,0x9E
    c.ram.write_word(0x00_0008, 0x0328);
    c.ram.write_byte(0x00_000A, 0x9E);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.vreg(E1), 0x38);

    // Read immediate byte
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000B));
    assert_eq!(c.cpu.vreg(E1), 0x38);

    // Write immediate byte into register
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000B));
    assert_eq!(c.cpu.vreg(E1), 0x9E);

    // LD [DE],L
    c.cpu.set_breg(DE, 0x00FE_DCBA);
    c.cpu.set_reg(L, 0x1234);
    c.ram.write_word(0x00_000B, 0x0416);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000D));
    assert_eq!(c.ram.read_word(0x00FE_DCBA), 0x0000);

    // Get memory location
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000D));
    assert_eq!(c.ram.read_word(0x00FE_DCBA), 0x0000);

    // Write L to memory location
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000D));
    assert_eq!(c.ram.read_word(0x00FE_DCBA), 0x1234);

    // LD HL,DE
    c.cpu.set_breg(HL, 0x0000_0000);
    c.cpu.set_breg(DE, 0xDEAD_BEEF);
    c.ram.write_word(0x00_000D, 0x0198);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000F));
    assert_eq!(c.cpu.breg(HL), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000F));
    assert_eq!(c.cpu.breg(HL), 0xDEAD_BEEF);
    assert_eq!(c.cpu.breg(DE), 0xDEAD_BEEF);

    // LD BC,0x0DEF_ACED
    c.cpu.set_breg(BC, 0x0000_0000);
    c.ram.write_word(0x00_000F, 0x0310);
    c.ram.write_word(0x00_0011, 0xACED);
    c.ram.write_word(0x00_0013, 0x0DEF);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0011));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);

    // Read first word
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0013));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);

    // Read second word
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);

    // Write to BC
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.breg(BC), 0x0DEF_ACED);

    // LD [DE],0xBABE
    c.cpu.set_breg(DE, 0x67_89AB);
    c.ram.write_word(0x67_89AB, 0x0000);
    c.ram.write_word(0x00_0015, 0x0331);
    c.ram.write_word(0x00_0017, 0xBABE);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0017));
    assert_eq!(c.ram.read_word(0x67_89AB), 0x0000);

    // Read immediate word
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0019));
    assert_eq!(c.ram.read_word(0x67_89AB), 0x0000);

    // Write immediate word to pointed-to value
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0019));
    assert_eq!(c.ram.read_word(0x67_89AB), 0xBABE);
}
