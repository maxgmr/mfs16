use mfs16core::{Computer, Flag::*, Flags, Pc, Reg16::*, Reg32::*, Reg8::*};
use pretty_assertions::assert_eq;

#[test]
fn test_add() {
    let mut c = Computer::default();
    let reset_flags = Flags::from_string("");
    c.cpu.flags.reset_all();

    // ADD A1,A0
    // 0b1111_0010 + 0b0000_0101 = 0b1111_0111, zcopN
    c.cpu.set_vreg(A1, 0b1111_0010);
    c.cpu.set_vreg(A0, 0b0000_0101);
    c.ram.write_word(0x00_0000, 0x1101);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.vreg(A1), 0b1111_0010);
    assert_eq!(c.cpu.vreg(A0), 0b0000_0101);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.vreg(A1), 0b1111_0111);
    assert_eq!(c.cpu.vreg(A0), 0b0000_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // ADD B1,B0
    // 0b1111_1111 + 0b0000_0001 = 0b0000_0000, ZCoPn
    c.cpu.flags.reset_all();
    c.cpu.set_vreg(B1, 0b1111_1111);
    c.cpu.set_vreg(B0, 0b0000_0001);
    c.ram.write_word(0x00_0002, 0x1123);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.vreg(B1), 0b1111_1111);
    assert_eq!(c.cpu.vreg(B0), 0b0000_0001);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.vreg(B1), 0b0000_0000);
    assert_eq!(c.cpu.vreg(B0), 0b0000_0001);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCoPn"));

    // ADD C1,C0
    // 0b0111_0000 + 0b0010_0000 = 0b1001_0000, zcOPN
    c.cpu.flags.reset_all();
    c.cpu.set_vreg(C1, 0b0111_0000);
    c.cpu.set_vreg(C0, 0b0010_0000);
    c.ram.write_word(0x00_0004, 0x1145);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(C1), 0b0111_0000);
    assert_eq!(c.cpu.vreg(C0), 0b0010_0000);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(C1), 0b1001_0000);
    assert_eq!(c.cpu.vreg(C0), 0b0010_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOPN"));

    // ADD D1,D0
    // 0b1000_0001 + 0b1000_0001 = 0b0000_0010, zCOPn
    c.cpu.flags.reset_all();
    c.cpu.set_vreg(D1, 0b1000_0001);
    c.cpu.set_vreg(D0, 0b1000_0001);
    c.ram.write_word(0x00_0006, 0x1167);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.vreg(D1), 0b1000_0001);
    assert_eq!(c.cpu.vreg(D0), 0b1000_0001);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.vreg(D1), 0b0000_0010);
    assert_eq!(c.cpu.vreg(D0), 0b1000_0001);
    assert_eq!(c.cpu.flags, Flags::from_string("zCOPn"));

    // ADD A,B
    // 0b1111_1111_1111_0010 + 0b0000_0000_0000_0101 = 0b1111_1111_1111_0111, zcopN
    c.cpu.flags.reset_all();
    c.cpu.set_reg(A, 0b1111_1111_1111_0010);
    c.cpu.set_reg(B, 0b0000_0000_0000_0101);
    c.ram.write_word(0x00_0008, 0x1001);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.reg(A), 0b1111_1111_1111_0010);
    assert_eq!(c.cpu.reg(B), 0b0000_0000_0000_0101);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.reg(A), 0b1111_1111_1111_0111);
    assert_eq!(c.cpu.reg(B), 0b0000_0000_0000_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // ADD C,D
    // 0b1111_1111_1111_1111 + 0b0000_0000_0000_0001 = 0b0000_0000_0000_0000, ZCoPn
    c.cpu.flags.reset_all();
    c.cpu.set_reg(C, 0b1111_1111_1111_1111);
    c.cpu.set_reg(D, 0b0000_0000_0000_0001);
    c.ram.write_word(0x00_000A, 0x1023);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(C), 0b1111_1111_1111_1111);
    assert_eq!(c.cpu.reg(D), 0b0000_0000_0000_0001);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(C), 0b0000_0000_0000_0000);
    assert_eq!(c.cpu.reg(D), 0b0000_0000_0000_0001);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCoPn"));

    // ADD E,H
    // 0b0111_0000_0000_0000 + 0b0010_0000_0000_0000 = 0b1001_0000_0000_0000, zcOPN
    c.cpu.flags.reset_all();
    c.cpu.set_reg(E, 0b0111_0000_0000_0000);
    c.cpu.set_reg(H, 0b0010_0000_0000_0000);
    c.ram.write_word(0x00_000C, 0x1045);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.reg(E), 0b0111_0000_0000_0000);
    assert_eq!(c.cpu.reg(H), 0b0010_0000_0000_0000);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.reg(E), 0b1001_0000_0000_0000);
    assert_eq!(c.cpu.reg(H), 0b0010_0000_0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOPN"));

    // ADD L,L
    // 0b1000_0000_0000_0001 + 0b1000_0000_0000_0001 = 0b0000_0000_0000_0010, zCOPn
    c.cpu.flags.reset_all();
    c.cpu.set_reg(L, 0b1000_0000_0000_0001);
    c.ram.write_word(0x00_000E, 0x1066);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.reg(L), 0b1000_0000_0000_0001);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.reg(L), 0b0000_0000_0000_0010);
    assert_eq!(c.cpu.flags, Flags::from_string("zCOPn"));

    // ADD BC,DE
    c.cpu.set_breg(BC, 0xFFFE_1DC0);
    c.cpu.set_breg(DE, 0x0001_86A0);
    c.ram.write_word(0x00_0010, 0x1078);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0xFFFE_1DC0);
    assert_eq!(c.cpu.breg(DE), 0x0001_86A0);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0xFFFF_A460);
    assert_eq!(c.cpu.breg(DE), 0x0001_86A0);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // ADC A1,D0 (no carry)
    c.cpu.flags.reset_all();
    c.cpu.set_vreg(A1, 0x01);
    c.cpu.set_vreg(D0, 0x01);
    c.ram.write_word(0x00_0012, 0x1307);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.vreg(A1), 0x01);
    assert_eq!(c.cpu.flags, reset_flags);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.vreg(A1), 0x02);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));

    // ADC L,C (with carry)
    c.cpu.set_reg(L, 0xFFFF);
    c.cpu.set_reg(C, 0x0001);
    c.cpu.set_flag(Carry);
    c.ram.write_word(0x00_0014, 0x1262);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0016));
    assert_eq!(c.cpu.reg(L), 0xFFFF);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0016));
    assert_eq!(c.cpu.reg(L), 0x0001);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopn"));

    // ADC DE,BC (with carry)
    c.cpu.set_breg(DE, 0x1234_5678);
    c.cpu.set_breg(BC, 0x8765_4320);
    c.ram.write_word(0x00_0016, 0x1287);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0018));
    assert_eq!(c.cpu.breg(DE), 0x1234_5678);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0018));
    assert_eq!(c.cpu.breg(DE), 0x9999_9999);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));
}
