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

    // Test weird overflow + carry case with ADC H1,H0
    // (Unsigned)       127 + 1 + 1 =  129 (No carry)
    // (Signed)         127 + 1 + 1 = -127 (Overflow!)
    c.cpu.flags.reset_all();
    c.cpu.set_flag(Carry);
    c.cpu.set_vreg(H1, 0b0111_1111);
    c.cpu.set_vreg(H0, 0b0000_0001);
    c.ram.write_word(0x00_0018, 0x13AB);

    // Read instruction
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.vreg(H1), 0b1000_0001);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpN"));

    // Test weird overflow + carry case with ADC H0,H1
    // (Unsigned)       1 + 127 + 1 =  129 (No carry)
    // (Signed)         1 + 127 + 1 = -127 (Overflow!)
    c.cpu.flags.reset_all();
    c.cpu.set_flag(Carry);
    c.cpu.set_vreg(H1, 0b0111_1111);
    c.cpu.set_vreg(H0, 0b0000_0001);
    c.ram.write_word(0x00_001A, 0x13BA);

    // Read instruction
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.vreg(H0), 0b1000_0001);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpN"));

    // ADD L0,0x54
    c.cpu.set_vreg(L0, 0x45);
    c.cpu.set_flag(Carry);
    c.ram.write_word(0x00_001C, 0x184D);
    c.ram.write_byte(0x00_001E, 0x54);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001E));
    assert_eq!(c.cpu.vreg(L0), 0x45);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001F));
    assert_eq!(c.cpu.vreg(L0), 0x45);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001F));
    assert_eq!(c.cpu.vreg(L0), 0x99);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpN"));

    // ADC L0,0x03
    c.cpu.set_flag(Carry);
    c.ram.write_word(0x00_001F, 0x185D);
    c.ram.write_byte(0x00_0021, 0x03);

    // Read instruction
    c.cycle();

    // Read imm8
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0022));
    assert_eq!(c.cpu.vreg(L0), 0x9D);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // ADD A,0x1234
    c.cpu.set_reg(A, 0x8765);
    c.ram.write_word(0x00_0022, 0x1800);
    c.ram.write_word(0x00_0024, 0x1234);

    // Read instruction
    c.cycle();

    // Read imm16
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0026));
    assert_eq!(c.cpu.reg(A), 0x9999);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // ADC A,0x0000
    c.cpu.set_flag(Carry);
    c.ram.write_word(0x00_0026, 0x1810);
    c.ram.write_word(0x00_0028, 0x0000);

    // Read instruction
    c.cycle();

    // Read imm16
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002A));
    assert_eq!(c.cpu.reg(A), 0x999A);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // ADD BC,0x7654_3210
    c.cpu.set_breg(BC, 0x2345_6789);
    c.ram.write_word(0x00_002A, 0x1820);
    c.ram.write_word(0x00_002C, 0x3210);
    c.ram.write_word(0x00_002E, 0x7654);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002C));
    assert_eq!(c.cpu.breg(BC), 0x2345_6789);

    // Read imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002E));
    assert_eq!(c.cpu.breg(BC), 0x2345_6789);
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0030));
    assert_eq!(c.cpu.breg(BC), 0x2345_6789);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0030));
    assert_eq!(c.cpu.breg(BC), 0x9999_9999);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpN"));

    // ADC BC,0x6666_6666;
    c.cpu.set_flag(Carry);
    c.ram.write_word(0x00_0030, 0x1830);
    c.ram.write_word(0x00_0032, 0x6666);
    c.ram.write_word(0x00_0034, 0x6666);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0032));
    assert_eq!(c.cpu.breg(BC), 0x9999_9999);

    // Read imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0034));
    assert_eq!(c.cpu.breg(BC), 0x9999_9999);
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0036));
    assert_eq!(c.cpu.breg(BC), 0x9999_9999);

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0036));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCoPn"));

    // ADD L,[DE]
    c.cpu.set_breg(DE, 0x0065_6565);
    c.ram.write_word(0x65_6565, 0x1234);
    c.cpu.set_reg(L, 0xFFFF);
    c.ram.write_word(0x00_0036, 0x1961);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0038));
    assert_eq!(c.cpu.reg(L), 0xFFFF);

    // Read [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0038));
    assert_eq!(c.cpu.reg(L), 0xFFFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0038));
    assert_eq!(c.cpu.reg(L), 0x1233);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopn"));

    // ADC L,[BC]
    c.cpu.set_breg(BC, 0x004B_4B4B);
    c.ram.write_word(0x004B_4B4B, 0x0000);
    c.ram.write_word(0x00_0038, 0x1A60);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_003A));
    assert_eq!(c.cpu.reg(L), 0x1233);

    // Read [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_003A));
    assert_eq!(c.cpu.reg(L), 0x1233);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_003A));
    assert_eq!(c.cpu.reg(L), 0x1234);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));
}

#[test]
fn test_sub() {
    let mut c = Computer::default();
    c.cpu.flags.reset_all();

    // Test weird overflow + carry case with SBB H1,H0
    // (Unsigned)        129 - 1 - 1 =  127 (No carry)
    // (Signed)         -127 - 1 - 1 = -129 (Overflow!)
    c.cpu.flags.reset_all();
    c.cpu.set_flag(Carry);
    c.cpu.set_vreg(H1, 0b1000_0001);
    c.cpu.set_vreg(H0, 0b0000_0001);
    c.ram.write_word(0x00_0000, 0x17AB);

    // Read instruction
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.vreg(H1), 0b0111_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // Test weird overflow + carry case with SBB H0,H1
    // (Unsigned)       255 - 127 - 1 = 127 (No carry)
    // (Signed)         -1 - 127 - 1 = -129 (Overflow!)
    c.cpu.flags.reset_all();
    c.cpu.set_flag(Carry);
    c.cpu.set_vreg(H1, 0b0111_1111);
    c.cpu.set_vreg(H0, 0b1111_1111);
    c.ram.write_word(0x00_0002, 0x17BA);

    // Read instruction
    c.cycle();

    // Do operation, set flags
    c.cycle();
    assert_eq!(c.cpu.vreg(H0), 0b0111_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // SUB A,A
    c.cpu.flags.reset_all();
    c.cpu.set_reg(A, 0x1234);
    c.ram.write_word(0x00_0004, 0x1400);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.reg(A), 0x1234);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.reg(A), 0x0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // SBB B,C
    c.cpu.flags.reset_all();
    c.cpu.flags.set_flag(Carry);
    c.cpu.set_reg(B, 0x294A);
    c.cpu.set_reg(C, 0x2826);
    c.ram.write_word(0x00_0006, 0x1612);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0x294A);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0x0123);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // SUB D0,D1
    c.cpu.set_vreg(D0, 0x00);
    c.cpu.set_vreg(D1, 0x01);
    c.ram.write_word(0x00_0008, 0x1576);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.vreg(D0), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.vreg(D0), 0xFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopN"));

    // SBB D0,D1
    c.ram.write_word(0x00_000A, 0x1776);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.vreg(D0), 0xFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.vreg(D0), 0xFD);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // SUB BC, HL
    c.cpu.set_breg(BC, 0x8000_0000);
    c.cpu.set_breg(HL, 0x0000_0001);
    c.ram.write_word(0x00_000C, 0x1479);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.breg(BC), 0x8000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.breg(BC), 0x7FFF_FFFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // SBB BC, HL
    c.ram.write_word(0x00_000E, 0x1679);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.breg(BC), 0x7FFF_FFFF);
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.breg(BC), 0x7FFF_FFFE);
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));

    // SUB C,0x1234
    c.cpu.set_reg(C, 0x1234);
    c.ram.write_word(0x00_0010, 0x1862);
    c.ram.write_word(0x00_0012, 0x1234);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.reg(C), 0x1234);
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));

    // Read imm16
    c.cycle();
    assert_eq!(c.cpu.reg(C), 0x1234);
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.reg(C), 0x0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // SBB C,0x1111
    c.ram.write_word(0x00_0014, 0x1872);
    c.ram.write_word(0x00_0016, 0x1111);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0016));
    assert_eq!(c.cpu.reg(C), 0x0000);

    // Read imm16
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0018));
    assert_eq!(c.cpu.reg(C), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0018));
    assert_eq!(c.cpu.reg(C), 0xEEEF);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopN"));

    // SUB A1,0x01
    c.cpu.set_vreg(A1, 0x82);
    c.ram.write_word(0x00_0018, 0x18A0);
    c.ram.write_byte(0x00_001A, 0x01);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001A));
    assert_eq!(c.cpu.vreg(A1), 0x82);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001B));
    assert_eq!(c.cpu.vreg(A1), 0x82);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001B));
    assert_eq!(c.cpu.vreg(A1), 0x81);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // SBB A1,0x02
    c.ram.write_word(0x00_001B, 0x18B0);
    c.ram.write_byte(0x00_001D, 0x02);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001D));
    assert_eq!(c.cpu.vreg(A1), 0x81);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001E));
    assert_eq!(c.cpu.vreg(A1), 0x81);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001E));
    assert_eq!(c.cpu.vreg(A1), 0x7F);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // SUB HL,0x1234_5678
    c.cpu.set_breg(HL, 0x789A_BCDE);
    c.ram.write_word(0x00_001E, 0x1892);
    c.ram.write_word(0x00_0020, 0x5678);
    c.ram.write_word(0x00_0022, 0x1234);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0020));
    assert_eq!(c.cpu.breg(HL), 0x789A_BCDE);

    // Read word 1 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0022));
    assert_eq!(c.cpu.breg(HL), 0x789A_BCDE);

    // Read word 0 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0024));
    assert_eq!(c.cpu.breg(HL), 0x789A_BCDE);

    // Do operation
    // 六六六六六六六六!
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0024));
    assert_eq!(c.cpu.breg(HL), 0x6666_6666);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));

    // SBB HL,0x0000_0000 with carry
    c.cpu.set_flag(Carry);
    c.ram.write_word(0x00_0024, 0x1892);
    c.ram.write_word(0x00_0026, 0x0000);
    c.ram.write_word(0x00_0028, 0x0000);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0026));
    assert_eq!(c.cpu.breg(HL), 0x6666_6666);

    // Read word 1 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0028));
    assert_eq!(c.cpu.breg(HL), 0x6666_6666);

    // Read word 0 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002A));
    assert_eq!(c.cpu.breg(HL), 0x6666_6666);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002A));
    assert_eq!(c.cpu.breg(HL), 0x6666_6665);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // SUB L,[DE]
    c.cpu.set_flag(Carry);
    c.cpu.set_breg(DE, 0x0065_6565);
    c.ram.write_word(0x65_6565, 0x0124);
    c.cpu.set_reg(L, 0x8123);
    c.ram.write_word(0x00_002A, 0x1B61);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002C));
    assert_eq!(c.cpu.reg(L), 0x8123);

    // Read [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002C));
    assert_eq!(c.cpu.reg(L), 0x8123);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002C));
    assert_eq!(c.cpu.reg(L), 0x7FFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // ADC L,[BC]
    c.cpu.set_breg(BC, 0x004B_4B4B);
    c.ram.write_word(0x004B_4B4B, 0x0000);
    c.ram.write_word(0x00_002C, 0x1C60);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002E));
    assert_eq!(c.cpu.reg(L), 0x7FFF);

    // Read [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002E));
    assert_eq!(c.cpu.reg(L), 0x7FFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002E));
    assert_eq!(c.cpu.reg(L), 0x7FFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));
}

#[test]
fn test_tcp() {
    let mut c = Computer::default();
    c.cpu.flags.reset_all();

    // TCP E
    c.cpu.set_reg(E, 0xFFFF);
    c.ram.write_word(0x00_0000, 0x1D04);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(E), 0xFFFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(E), 0x0001);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopn"));

    // TCP BC
    c.cpu.set_breg(BC, 0xEDCB_A988);
    c.ram.write_word(0x00_0002, 0x1D10);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(BC), 0xEDCB_A988);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.flags, Flags::from_string("zCoPn"));

    // TCP D1
    c.cpu.set_vreg(D1, 0x01);
    c.ram.write_word(0x00_0004, 0x1D26);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(D1), 0x01);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(D1), 0xFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopN"));

    // TCP D0 where D0 == 0
    c.cpu.set_vreg(D0, 0x00);
    c.ram.write_word(0x00_0006, 0x1D27);

    // Read instr + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.vreg(D0), 0x00);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));
}

#[test]
fn test_inc_dec() {
    let mut c = Computer::default();
    c.cpu.flags.reset_all();

    // DEC A
    c.cpu.flags = Flags::from_string("ZcOPn");
    c.cpu.set_reg(A, 0x0000);
    c.ram.write_word(0x00_0000, 0x1D60);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(A), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(A), 0xFFFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // INC A
    c.cpu.flags = Flags::from_string("zcopn");
    c.ram.write_word(0x00_0002, 0x1D30);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.reg(A), 0xFFFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.reg(A), 0x0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // INC HL
    c.cpu.flags = Flags::from_string("zcopn");
    c.cpu.set_breg(HL, 0x7FFF_FFFF);
    c.ram.write_word(0x00_0004, 0x1D42);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.breg(HL), 0x7FFF_FFFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.breg(HL), 0x8000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));

    // DEC HL
    c.cpu.flags = Flags::from_string("zcOpn");
    c.ram.write_word(0x00_0006, 0x1D72);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.breg(HL), 0x8000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.breg(HL), 0x7FFF_FFFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // INC B1
    c.cpu.flags = Flags::from_string("zcopn");
    c.cpu.set_vreg(B1, 0x12);
    c.ram.write_word(0x00_0008, 0x1D52);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.vreg(B1), 0x12);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.vreg(B1), 0x13);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // DEC B1
    c.ram.write_word(0x00_000A, 0x1D82);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.vreg(B1), 0x13);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.vreg(B1), 0x12);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));
}

#[test]
fn test_pss() {
    let mut c = Computer::default();
    let starting_flags = c.cpu.flags.clone();

    // PSS H
    c.cpu.set_reg(H, 0x0000);
    c.ram.write_word(0x00_0000, 0x1D95);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(H), 0x0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // PSS BC
    c.cpu.set_breg(BC, 0x1234_6567);
    c.cpu.flags = starting_flags.clone();
    c.ram.write_word(0x00_0002, 0x1DA0);
    // Ensure that carry and overflow flags are untouched
    c.cpu.set_flag(Carry);
    c.cpu.set_flag(Overflow);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(BC), 0x1234_6567);
    assert_eq!(c.cpu.flags, Flags::from_string("zCOpn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(BC), 0x1234_6567);
    assert_eq!(c.cpu.flags, Flags::from_string("zCOpn"));

    // PSS D0
    c.cpu.set_vreg(D0, 0x80);
    c.cpu.flags = starting_flags.clone();
    c.ram.write_word(0x00_0004, 0x1DB7);
    c.cpu.set_flag(Overflow);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(D0), 0x80);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(D0), 0x80);
    assert_eq!(c.cpu.flags, Flags::from_string("zcOPN"));

    // PSS imm16
    c.cpu.flags = starting_flags.clone();
    c.ram.write_word(0x00_0006, 0x1DC0);
    c.ram.write_word(0x00_0008, 0x9ABC);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.flags, starting_flags);

    // Read imm16
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // PSS imm32
    c.ram.write_word(0x00_000A, 0x1DC1);
    c.ram.write_word(0x00_000C, 0xABCD);
    c.ram.write_word(0x00_000E, 0x1234);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // Read word 0 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // Read word 1 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // PSS imm8
    c.ram.write_word(0x00_0010, 0x1DC2);
    c.ram.write_byte(0x00_0012, 0x99);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0013));
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0013));
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));
}

#[test]
fn test_and() {
    let mut c = Computer::default();
    let starting_flags = c.cpu.flags.clone();

    // AND A,B
    c.cpu.flags = Flags::from_string("ZCOPN");
    c.cpu.set_reg(A, 0b1010_1010_1010_1010);
    c.cpu.set_reg(B, 0b1100_1111_0000_0101);
    c.ram.write_word(0x00_0000, 0x1E01);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(A), 0b1010_1010_1010_1010);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(A), 0b1000_1010_0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // AND DE,HL
    c.cpu.flags = starting_flags.clone();
    c.cpu
        .set_breg(DE, 0b1111_1111_1111_1111_0000_0000_0000_0000);
    c.cpu
        .set_breg(HL, 0b0000_0000_0000_0000_1111_1111_1111_1111);
    c.ram.write_word(0x00_0002, 0x1F12);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(DE), 0b1111_1111_1111_1111_0000_0000_0000_0000);
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(DE), 0);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // AND C1,C0
    c.cpu.flags = starting_flags.clone();
    c.cpu.set_vreg(C1, 0b1111_1000);
    c.cpu.set_vreg(C0, 0b1010_1100);
    c.ram.write_word(0x00_0004, 0x2045);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(C1), 0b1111_1000);
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(C1), 0b1010_1000);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // AND B,[HL]
    c.cpu.flags = Flags::from_string("ZCOpn");
    c.cpu.set_reg(B, 0b1111_1111_0000_0000);
    c.cpu.set_breg(HL, 0xFE_DCBA);
    c.ram.write_word(0xFE_DCBA, 0b0101_0101_0101_0101);
    c.ram.write_word(0x00_0006, 0x2112);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b1111_1111_0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOpn"));

    // Get [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b1111_1111_0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOpn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b0101_0101_0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));

    // AND D,0b0000_0000_1111_1111
    c.cpu.set_reg(D, 0b1111_1111_1111_1111);
    c.ram.write_word(0x00_0008, 0x2A03);
    c.ram.write_word(0x00_000A, 0b0000_0000_1111_1111);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.reg(D), 0xFFFF);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(D), 0xFFFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(D), 0x00FF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // AND BC,0x0F0F_F0F0
    c.cpu.set_breg(BC, 0xFFFF_FFFF);
    c.ram.write_word(0x00_000C, 0x2A10);
    c.ram.write_word(0x00_000E, 0xF0F0);
    c.ram.write_word(0x00_0010, 0x0F0F);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.breg(BC), 0xFFFF_FFFF);

    // Read word 0 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.breg(BC), 0xFFFF_FFFF);

    // Read word 1 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0xFFFF_FFFF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0x0F0F_F0F0);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPn"));

    // AND L0,0x81
    c.cpu.set_vreg(L0, 0b0000_1111);
    c.ram.write_word(0x00_0012, 0x2A2D);
    c.ram.write_byte(0x00_0014, 0x81);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.vreg(L0), 0b0000_1111);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.vreg(L0), 0b0000_1111);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.vreg(L0), 0b0000_0001);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));
}

#[test]
fn test_or() {
    let mut c = Computer::default();
    let starting_flags = c.cpu.flags.clone();

    // OR B,A
    c.cpu.flags = Flags::from_string("ZCOPN");
    c.cpu.set_reg(A, 0b1010_1010_1010_1010);
    c.cpu.set_reg(B, 0b1100_1111_0000_0101);
    c.ram.write_word(0x00_0000, 0x2210);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(B), 0b1100_1111_0000_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(B), 0b1110_1111_1010_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // OR DE,HL
    c.cpu.flags = starting_flags.clone();
    c.cpu
        .set_breg(DE, 0b1111_1111_1111_1111_0000_0000_0000_0000);
    c.cpu
        .set_breg(HL, 0b0000_0000_0000_0000_1111_1111_1111_1111);
    c.ram.write_word(0x00_0002, 0x2312);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(DE), 0b1111_1111_1111_1111_0000_0000_0000_0000);
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(DE), 0xFFFF_FFFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // OR C1,C0
    c.cpu.flags = starting_flags.clone();
    c.cpu.set_vreg(C1, 0b0000_0000);
    c.cpu.set_vreg(C0, 0b0000_0000);
    c.ram.write_word(0x00_0004, 0x2445);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // OR B,[HL]
    c.cpu.flags = Flags::from_string("ZCOpn");
    c.cpu.set_reg(B, 0b0011_0000_0000_1111);
    c.cpu.set_breg(HL, 0xFE_DCBA);
    c.ram.write_word(0xFE_DCBA, 0b1000_0100_0000_0011);
    c.ram.write_word(0x00_0006, 0x2512);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b0011_0000_0000_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOpn"));

    // Get [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b0011_0000_0000_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOpn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b1011_0100_0000_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // OR D,0b0000_0000_1111_1111
    c.cpu.set_reg(D, 0x0000);
    c.ram.write_word(0x00_0008, 0x2A33);
    c.ram.write_word(0x00_000A, 0b0000_0000_1111_1111);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.reg(D), 0x0000);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(D), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(D), 0x00FF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // OR BC,0x0F0F_F0F0
    c.cpu.set_breg(BC, 0x1010_0101);
    c.ram.write_word(0x00_000C, 0x2A40);
    c.ram.write_word(0x00_000E, 0xF0F0);
    c.ram.write_word(0x00_0010, 0x0F0F);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.breg(BC), 0x1010_0101);

    // Read word 0 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.breg(BC), 0x1010_0101);

    // Read word 1 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0x1010_0101);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0x1F1F_F1F1);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // OR L0,0x81
    c.cpu.set_vreg(L0, 0b0000_1111);
    c.ram.write_word(0x00_0012, 0x2A5D);
    c.ram.write_byte(0x00_0014, 0x81);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.vreg(L0), 0b0000_1111);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.vreg(L0), 0b0000_1111);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.vreg(L0), 0b1000_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));
}

#[test]
fn test_xor() {
    let mut c = Computer::default();
    let starting_flags = c.cpu.flags.clone();

    // XOR B,A
    c.cpu.flags = Flags::from_string("ZCOPN");
    c.cpu.set_reg(A, 0b1010_1010_1010_1010);
    c.cpu.set_reg(B, 0b1100_1111_0000_0101);
    c.ram.write_word(0x00_0000, 0x2610);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(B), 0b1100_1111_0000_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(B), 0b0110_0101_1010_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // XOR DE,HL
    c.cpu.flags = starting_flags.clone();
    c.cpu
        .set_breg(DE, 0b1111_1111_1111_1111_0000_0000_0000_0000);
    c.cpu
        .set_breg(HL, 0b0000_0000_0000_0000_1111_1111_1111_1111);
    c.ram.write_word(0x00_0002, 0x2712);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(DE), 0b1111_1111_1111_1111_0000_0000_0000_0000);
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(DE), 0xFFFF_FFFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopN"));

    // XOR C1,C0
    c.cpu.flags = starting_flags.clone();
    c.cpu.set_vreg(C1, 0b1010_1010);
    c.cpu.set_vreg(C0, 0b1010_1010);
    c.ram.write_word(0x00_0004, 0x2845);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(C1), 0b1010_1010);
    assert_eq!(c.cpu.flags, starting_flags);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(C1), 0b0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // XOR B,[HL]
    c.cpu.flags = Flags::from_string("ZCOpn");
    c.cpu.set_reg(B, 0b0011_0000_0000_1111);
    c.cpu.set_breg(HL, 0xFE_DCBA);
    c.ram.write_word(0xFE_DCBA, 0b1000_0100_0000_0011);
    c.ram.write_word(0x00_0006, 0x2912);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b0011_0000_0000_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOpn"));

    // Get [HL]
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b0011_0000_0000_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOpn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.reg(B), 0b1011_0100_0000_1100);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));

    // XOR D,0b0000_0000_1111_1111
    c.cpu.set_reg(D, 0x0000);
    c.ram.write_word(0x00_0008, 0x2A63);
    c.ram.write_word(0x00_000A, 0b0000_0000_1111_1111);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.reg(D), 0x0000);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(D), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(D), 0x00FF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // XOR BC,0x0F0F_F0F0
    c.cpu.set_breg(BC, 0x1010_0101);
    c.ram.write_word(0x00_000C, 0x2A70);
    c.ram.write_word(0x00_000E, 0xF0F0);
    c.ram.write_word(0x00_0010, 0x0F0F);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.breg(BC), 0x1010_0101);

    // Read word 0 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.breg(BC), 0x1010_0101);

    // Read word 1 of imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0x1010_0101);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(BC), 0x1F1F_F1F1);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // XOR L0,0x81
    c.cpu.set_vreg(L0, 0b0000_1111);
    c.ram.write_word(0x00_0012, 0x2A8D);
    c.ram.write_byte(0x00_0014, 0x81);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.vreg(L0), 0b0000_1111);

    // Read imm8
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.vreg(L0), 0b0000_1111);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0015));
    assert_eq!(c.cpu.vreg(L0), 0b1000_1110);
    assert_eq!(c.cpu.flags, Flags::from_string("zcoPN"));
}

#[test]
fn test_not() {
    let mut c = Computer::default();

    // NOT A
    c.cpu.set_reg(A, 0b1010_1010_1010_1010);
    c.ram.write_word(0x00_0000, 0x2A90);
    c.cpu.flags = Flags::from_string("ZCOPN");

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(A), 0b1010_1010_1010_1010);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.reg(A), 0b0101_0101_0101_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // NOT BC
    c.cpu.set_breg(BC, 0xFFFF_FFFF);
    c.ram.write_word(0x00_0002, 0x2AA0);
    c.cpu.flags = Flags::from_string("zcopn");

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(BC), 0xFFFF_FFFF);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));

    // NOT D1
    c.cpu.set_vreg(D1, 0x00);
    c.ram.write_word(0x00_0004, 0x2AB6);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(D1), 0x00);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(D1), 0xFF);
}
