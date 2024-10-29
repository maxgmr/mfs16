use mfs16core::{
    gen_ram, Computer, Flag::*, Flags, Instruction::*, Pc, Ram, RamWritable, Reg, Reg16::*,
    Reg32::*, Reg8::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::instr_test;

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

#[test]
fn test_as() {
    let mut c = Computer::default();

    // ASR A1,0
    c.ram.write_word(0x00_0000, 0x2D00);
    c.cpu.set_vreg(A1, 0b1010_1010);
    c.cpu.flags = Flags::from_string("ZCOPN");

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.vreg(A1), 0b1010_1010);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // ASR A1,1
    c.ram.write_word(0x00_0002, 0x2D01);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.vreg(A1), 0b1010_1010);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.vreg(A1), 0b1101_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPN"));

    // ASR A1,8
    c.ram.write_word(0x00_0004, 0x2D08);

    // Read instr + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.vreg(A1), 0b1111_1111);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCoPN"));

    // ASR C1,9
    c.cpu.set_vreg(C1, 0b0101_0101);
    c.ram.write_word(0x00_0006, 0x2D49);
    c.cpu.flags = Flags::from_string("ZCOPN");

    // Read instr + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.vreg(C1), 0b0000_0000);
    assert_eq!(c.cpu.flags, Flags::from_string("ZPN"));

    // ASR B,0
    c.ram.write_word(0x00_0008, 0x2B10);
    c.cpu.set_reg(B, 0x2F05);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.reg(B), 0x2F05);

    // ASR B,1
    c.cpu.flags = Flags::from_string("");
    c.ram.write_word(0x00_000A, 0x2B11);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(B), 0x2F05);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.reg(B), 0x1782);
    assert_eq!(c.cpu.flags, Flags::from_string("C"));

    // ASR B,4
    c.ram.write_word(0x00_000C, 0x2B14);

    // Read instr + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.reg(B), 0x0178);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // ASR HL,0
    c.cpu.set_breg(HL, 0x80F1_9696);
    c.ram.write_word(0x00_000E, 0x2C20);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0010));
    assert_eq!(c.cpu.breg(HL), 0x80F1_9696);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // ASR HL,1
    c.ram.write_word(0x00_0010, 0x2C21);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(HL), 0x80F1_9696);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.breg(HL), 0xC078_CB4B);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // ASR DE,15
    c.cpu.set_breg(DE, 0x8000_0000);
    c.ram.write_word(0x00_0012, 0x2C1F);

    // Read instr + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));
    assert_eq!(c.cpu.breg(DE), 0xFFFF_0000);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // ASL ra,b
    instr_test!(
        REGS: [(A, 0b1010_1111_0000_0101)],
        RAM: gen_ram![
            AslRaB(A, 0).into_opcode(),
            AslRaB(A, 1).into_opcode(),
            AslRaB(A, 2).into_opcode()
        ],
        FLAGS: "",
        [
            // ASL A,0
            (0x00_0002, [(A, 0xAF05)], ""),
            (0x00_0002, [(A, 0xAF05)], ""),
            // ASL A,1
            (0x00_0004, [(A, 0xAF05)], ""),
            (0x00_0004, [(A, 0x5E0A)], "CO"),
            // ASL A,2
            (0x00_0006, [(A, 0b0101_1110_0000_1010)], "CO"),
            (0x00_0006, [(A, 0x7828)], "C")
        ]
    );

    // ASL vra,b
    instr_test!(
        REGS: [(B1, 0b0100_1011)],
        RAM: gen_ram![
            AslVraB(B1, 0).into_opcode(),
            AslVraB(B1, 1).into_opcode(),
            AslVraB(B1, 15).into_opcode()
        ],
        FLAGS: "",
        [
            // ASL B1,0
            (0x00_0002, [(B1, 0x4B)], ""),
            (0x00_0002, [(B1, 0x4B)], ""),
            // ASL B1,1
            (0x00_0004, [(B1, 0x4B)], ""),
            (0x00_0004, [(B1, 0x96)], "O"),
            // ASL B1,15
            (0x00_0006, [(B1, 0x96)], "O"),
            (0x00_0006, [(B1, 0x00)], "O")
        ]
    );

    // ASL bra,b
    instr_test!(
        REGS: [(DE, 0x6969_0F0F)],
        RAM: gen_ram![
            AslBraB(DE, 0).into_opcode(),
            AslBraB(DE, 1).into_opcode(),
            AslBraB(DE, 12).into_opcode()
        ],
        FLAGS: "ZCOPN",
        [
            // ASL DE,0
            (0x00_0002, [(DE, 0x6969_0F0F)], "ZCOPN"),
            (0x00_0002, [(DE, 0x6969_0F0F)], "ZCOPN"),
            // ASL DE,1
            (0x00_0004, [(DE, 0x6969_0F0F)], "ZNCPO"),
            (0x00_0004, [(DE, 0xD2D2_1E1E_u32)], "ZOPN"),
            // ASL DE,12
            (0x00_0006, [(DE, 0xD2D2_1E1E_u32)], "ZOPN"),
            (0x00_0006, [(DE, 0x21E1_E000)], "ZCOPN")
        ]
    );
}

#[test]
fn test_lsr() {
    instr_test!(
        REGS: [(L, 0x9696), (BC, 0xA4A4_A4A4), (E0, 0x81)],
        RAM: gen_ram![
            LsrRaB(L, 0).into_opcode(),
            LsrRaB(L, 1).into_opcode(),
            LsrRaB(L, 1).into_opcode(),
            LsrRaB(L, 15).into_opcode(),
            LsrBraB(BC, 0).into_opcode(),
            LsrBraB(BC, 1).into_opcode(),
            LsrBraB(BC, 4).into_opcode(),
            LsrVraB(E0, 0).into_opcode(),
            LsrVraB(E0, 1).into_opcode(),
            LsrVraB(E0, 15).into_opcode()
        ],
        FLAGS: "ZPN",
        [
            // LSR L,0
            (0x00_0002, [(L, 0x9696)], "ZPN"),
            (0x00_0002, [(L, 0x9696)], "ZPN"),
            // LSR L,1
            (0x00_0004, [(L, 0x9696)], "ZPN"),
            (0x00_0004, [(L, 0x4B4B)], "ZOPN"),
            // LSR L,1
            (0x00_0006, [(L, 0x4B4B)], "ZOPN"),
            (0x00_0006, [(L, 0x25A5)], "ZCPN"),
            // LSR L,15
            (0x00_0008, [(L, 0x25A5)], "ZCPN"),
            (0x00_0008, [(L, 0x0000)], "ZPN"),
            // LSR BC,0
            (0x00_000A, [(BC, 0xA4A4_A4A4_u32)], "ZPN"),
            (0x00_000A, [(BC, 0xA4A4_A4A4_u32)], "ZPN"),
            // LSR BC,1
            (0x00_000C, [(BC, 0xA4A4_A4A4_u32)], "ZPN"),
            (0x00_000C, [(BC, 0x5252_5252)], "ZOPN"),
            // LSR BC,4
            (0x00_000E, [(BC, 0x5252_5252)], "ZOPN"),
            (0x00_000E, [(BC, 0x0525_2525)], "ZPN")
        ]
    );
}

#[test]
fn test_rt() {
    instr_test!(
        REGS: [
            (A, 0x9696),
            (BC, 0xA4A4_A4A4),
            (D1, 0x81),
            (E, 0x1234),
            (HL, 0xE1E1_1E1E),
            (D0, 0x75)
        ],
        RAM: gen_ram![
            RtrRaB(A, 0).into_opcode(),
            RtrRaB(A, 1).into_opcode(),
            RtrRaB(A, 2).into_opcode(),
            RtrBraB(BC, 0).into_opcode(),
            RtrBraB(BC, 1).into_opcode(),
            RtrBraB(BC, 2).into_opcode(),
            RtrVraB(D1, 0).into_opcode(),
            RtrVraB(D1, 1).into_opcode(),
            RtrVraB(D1, 2).into_opcode(),
            RtrVraB(D1, 8).into_opcode(),
            RtrVraB(D1, 9).into_opcode(),
            RtlRaB(E, 0).into_opcode(),
            RtlRaB(E, 1).into_opcode(),
            RtlRaB(E, 2).into_opcode(),
            RtlBraB(HL, 0).into_opcode(),
            RtlBraB(HL, 1).into_opcode(),
            RtlBraB(HL, 2).into_opcode(),
            RtlVraB(D0, 0).into_opcode(),
            RtlVraB(D0, 1).into_opcode(),
            RtlVraB(D0, 2).into_opcode(),
            RtlVraB(D0, 8).into_opcode(),
            RtlVraB(D0, 9).into_opcode()
        ],
        FLAGS: "",
        [
            // RTR A,0
            (0x00_0002, [(A,0x9696)], ""),
            (0x00_0002, [(A,0x9696)], ""),
            // RTR A,1
            (0x00_0004, [(A,0x9696)], ""),
            (0x00_0004, [(A,0x4B4B)], "O"),
            // RTR A,2
            (0x00_0006, [(A,0x4B4B)], "O"),
            (0x00_0006, [(A,0xD2D2)], "CO"),
            // RTR BC,0
            (0x00_0008, [(BC,0xA4A4_A4A4_u32)], "CO"),
            (0x00_0008, [(BC,0xA4A4_A4A4_u32)], "CO"),
            // RTR BC,1
            (0x00_000A, [(BC,0xA4A4_A4A4_u32)], "CO"),
            (0x00_000A, [(BC,0x5252_5252)], "O"),
            // RTR BC,2
            (0x00_000C, [(BC,0x5252_5252)], "O"),
            (0x00_000C, [(BC,0x9494_9494_u32)], "OC"),
            // RTR D1,0
            (0x00_000E, [(D1,0x81)], "CO"),
            (0x00_000E, [(D1,0x81)], "CO"),
            // RTR D1,1
            (0x00_0010, [(D1,0x81)], "CO"),
            (0x00_0010, [(D1,0xC0)], "C"),
            // RTR D1,2
            (0x00_0012, [(D1,0xC0)], "C"),
            (0x00_0012, [(D1,0x30)], "O"),
            // RTR D1,8
            (0x00_0014, [(D1,0x30)], "O"),
            (0x00_0014, [(D1,0x30)], "O"),
            // RTR D1,9
            (0x00_0016, [(D1,0x30)], "O"),
            (0x00_0016, [(D1,0x18)], ""),
            // RTL E,0
            (0x00_0018, [(E,0x1234)], ""),
            (0x00_0018, [(E,0x1234)], ""),
            // RTL E,1
            (0x00_001A, [(E,0x1234)], ""),
            (0x00_001A, [(E,0x2468)], ""),
            // RTL E,2
            (0x00_001C, [(E,0x2468)], ""),
            (0x00_001C, [(E,0x91A0)], "O"),
            // RTL HL,0
            (0x00_001E, [(HL,0xE1E1_1E1E_u32)], "O"),
            (0x00_001E, [(HL,0xE1E1_1E1E_u32)], "O"),
            // RTL HL,1
            (0x00_0020, [(HL,0xE1E1_1E1E_u32)], "O"),
            (0x00_0020, [(HL,0xC3C2_3C3D_u32)], "C"),
            // RTL HL,2
            (0x00_0022, [(HL,0xC3C2_3C3D_u32)], "C"),
            (0x00_0022, [(HL,0x0F08_F0F7)], "OC"),
            // RTL D0,0
            (0x00_0024, [(D0,0x75)], "OC"),
            (0x00_0024, [(D0,0x75)], "OC"),
            // RTL D0,1
            (0x00_0026, [(D0,0x75)], "CO"),
            (0x00_0026, [(D0,0xEA)], "O"),
            // RTL D0,2
            (0x00_0028, [(D0,0xEA)], "O"),
            (0x00_0028, [(D0,0xAB)], "C"),
            // RTL D0,8
            (0x00_002A, [(D0,0xAB)], "C"),
            (0x00_002A, [(D0,0xAB)], "C"),
            // RTL D0,9
            (0x00_002C, [(D0,0xAB)], "C"),
            (0x00_002C, [(D0,0x57)], "CO")
        ]
    );
}

#[test]
fn test_rc() {
    let mut c = Computer::default();

    // RCR A1,0; Carry reset
    c.ram.write_word(0x00_0000, 0x3C00);
    c.cpu.set_vreg(A1, 0b1010_1010);
    c.cpu.flags = Flags::from_string("ZcOPN");

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0002));
    assert_eq!(c.cpu.flags, Flags::from_string("ZcOPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.flags, Flags::from_string("ZcOPN"));
    assert_eq!(c.cpu.vreg(A1), 0b1010_1010);

    // RCR A1,4; Carry reset
    c.ram.write_word(0x00_0002, 0x3C04);
    c.cpu.set_vreg(A1, 0b1010_1010);
    c.cpu.flags = Flags::from_string("zcopn");

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0004));
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.flags, Flags::from_string("zCOpn"));
    assert_eq!(c.cpu.vreg(A1), 0b0100_1010);

    // RCR A0,1; Carry set
    c.ram.write_word(0x00_0004, 0x3C11);
    c.cpu.set_vreg(A0, 0b0101_0101);
    c.cpu.flags = Flags::from_string("ZCoPn");

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.flags, Flags::from_string("ZCoPn"));
    assert_eq!(c.cpu.vreg(A0), 0b0101_0101);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0006));
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPn"));
    assert_eq!(c.cpu.vreg(A0), 0b1010_1010);

    // RCR B1,2; Carry set
    c.ram.write_word(0x00_0006, 0x3C22);
    c.cpu.set_vreg(B1, 0b0001_0111);

    // Read + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0008));
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPn"));
    assert_eq!(c.cpu.vreg(B1), 0b1100_0101);

    // RCR B1,11; Carry set
    c.ram.write_word(0x00_0008, 0x3C2B);

    // Read + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000A));
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));
    assert_eq!(c.cpu.vreg(B1), 0b1111_0001);

    // RCR B0,8; Carry reset
    c.cpu.set_vreg(B0, 0b0001_0111);
    c.ram.write_word(0x00_000A, 0x3C38);

    // Read + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000C));
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPn"));
    assert_eq!(c.cpu.vreg(B0), 0b0010_1110);

    // RCR A,0; Carry set
    c.cpu.flags = Flags::from_string("zCopn");
    c.cpu.set_reg(A, 0b1010_1010_1010_1010);
    c.ram.write_word(0x00_000C, 0x3A00);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.reg(A), 0b1010_1010_1010_1010);
    assert_eq!(c.cpu.pc, Pc::new(0x00_000E));
    assert_eq!(c.cpu.flags, Flags::from_string("zCopn"));

    // RCR A,1; Carry set
    c.ram.write_word(0x00_000E, 0x3A01);

    // Read + do operation
    c.cycle();
    assert_eq!(c.cpu.reg(A), 0b1010_1010_1010_1010);
    assert_eq!(c.cpu.flags, Flags::from_string("zCopn"));
    c.cycle();
    assert_eq!(c.cpu.reg(A), 0b1101_0101_0101_0101);
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));

    // RCR B,15; Carry reset
    c.cpu.set_reg(B, 0b0100_0000_0000_0000);
    c.ram.write_word(0x00_0010, 0x3A1F);

    // Read + do operation
    c.cycle();
    assert_eq!(c.cpu.reg(B), 0x4000);
    c.cycle();
    assert_eq!(c.cpu.reg(B), 0);
    assert_eq!(c.cpu.pc, Pc::new(0x00_0012));
    assert_eq!(c.cpu.flags, Flags::from_string("zCopn"));

    // RCR DE,0; Carry set
    c.cpu.flags = Flags::from_string("zCOpn");
    c.cpu.set_breg(DE, 0xAAAA_AAAA);
    c.ram.write_word(0x00_0012, 0x3B10);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0014));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.breg(DE), 0xAAAA_AAAA);
    assert_eq!(c.cpu.flags, Flags::from_string("zCOpn"));

    // RCR DE,1; Carry set
    c.ram.write_word(0x00_0014, 0x3B11);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0016));
    assert_eq!(c.cpu.flags, Flags::from_string("zCOpn"));
    assert_eq!(c.cpu.breg(DE), 0xAAAA_AAAA);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0016));
    assert_eq!(c.cpu.flags, Flags::from_string("zcopn"));
    assert_eq!(c.cpu.breg(DE), 0xD555_5555);

    // RCR HL,15; Carry reset
    c.cpu.set_breg(HL, 0x8000_0000);
    c.ram.write_word(0x00_0016, 0x3B2F);

    // Read + do op
    c.cycle();
    assert_eq!(c.cpu.breg(HL), 0x8000_0000);
    c.cycle();
    assert_eq!(c.cpu.flags, Flags::from_string("zcOpn"));
    assert_eq!(c.cpu.breg(HL), 0x0001_0000);

    // RCL A1,0; Carry set
    c.cpu.set_vreg(A1, 0x88);
    c.cpu.flags = Flags::from_string("ZCOPN");
    c.ram.write_word(0x00_0018, 0x3F00);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001A));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001A));
    assert_eq!(c.cpu.vreg(A1), 0x88);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // RCL A1,1; Carry set
    c.cpu.flags = Flags::from_string("ZCoPN");
    c.ram.write_word(0x00_001A, 0x3F01);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001C));
    assert_eq!(c.cpu.vreg(A1), 0x88);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCoPN"));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001C));
    assert_eq!(c.cpu.vreg(A1), 0x11);
    assert_eq!(c.cpu.flags, Flags::from_string("ZCOPN"));

    // RCL A1,10; Carry set
    c.ram.write_word(0x00_001C, 0x3F0A);

    // Read + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_001E));
    assert_eq!(c.cpu.vreg(A1), 0x23);
    assert_eq!(c.cpu.flags, Flags::from_string("ZcoPN"));

    // RCL D,0; Carry reset
    c.cpu.set_reg(D, 0x9600);
    c.cpu.flags = Flags::from_string("");
    c.ram.write_word(0x00_001E, 0x3D30);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0020));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0020));
    assert_eq!(c.cpu.reg(D), 0x9600);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // RCL D,1; Carry reset
    c.ram.write_word(0x00_0020, 0x3D31);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0022));
    assert_eq!(c.cpu.flags, Flags::from_string(""));
    assert_eq!(c.cpu.reg(D), 0x9600);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0022));
    assert_eq!(c.cpu.flags, Flags::from_string("CO"));
    assert_eq!(c.cpu.reg(D), 0x2C00);

    // RCL E,15; Carry set
    c.cpu.set_reg(E, 0x0000);
    c.ram.write_word(0x00_0022, 0x3D4F);

    // Read + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0024));
    assert_eq!(c.cpu.reg(E), 0x4000);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // RCL HL,0; Carry reset
    c.cpu.set_breg(HL, 0xAAAA_00FF);
    c.ram.write_word(0x00_0024, 0x3E20);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0026));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0026));
    assert_eq!(c.cpu.breg(HL), 0xAAAA_00FF);
    assert_eq!(c.cpu.flags, Flags::from_string(""));

    // RCL HL,1; Carry reset
    c.ram.write_word(0x00_0026, 0x3E21);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0028));
    assert_eq!(c.cpu.flags, Flags::from_string(""));
    assert_eq!(c.cpu.breg(HL), 0xAAAA_00FF);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_0028));
    assert_eq!(c.cpu.flags, Flags::from_string("CO"));
    assert_eq!(c.cpu.breg(HL), 0x5554_01FE);

    // RCL DE,15; Carry set
    c.cpu.set_breg(DE, 0x0000_0001);
    c.ram.write_word(0x00_0028, 0x3E1F);

    // Read instr + do operation
    c.cycle();
    c.cycle();
    assert_eq!(c.cpu.pc, Pc::new(0x00_002A));
    assert_eq!(c.cpu.breg(DE), 0x0000_C000);
    assert_eq!(c.cpu.flags, Flags::from_string(""));
}

#[test]
fn test_cmp() {
    instr_test!(
        REGS: [
            (A, 0x0001),
            (BC, 0x0000_0001),
            (DE, 0x0000_0000),
            (H1, 0x01),
            (H0, 0x00)
        ],
        RAM: gen_ram![
            CmpRaRb(B, A).into_opcode(),
            CmpBraBrb(DE, BC).into_opcode(),
            CmpVraVrb(H0, H1).into_opcode(),
            CmpRaImm16(A).into_opcode(),
            0xFFFF_u16,
            CmpBraImm32(BC).into_opcode(),
            0xFFFF_FFFF_u32,
            CmpVraImm8(H1).into_opcode(),
            0xFF_u8,
            CmpImm16Ra(A).into_opcode(),
            0x8000_u16,
            CmpImm32Bra(BC).into_opcode(),
            0x8000_0000_u32,
            CmpImm8Vra(H1).into_opcode(),
            0x80_u8
        ],
        FLAGS: "ZOP",
        [
            // CMP B,A
            (0x00_0002, [(A, 0x0001), (B, 0x0000)], "ZOP"),
            (0x00_0002, [(A, 0x0001), (B, 0x0000)], "CN"),
            // CMP DE,BC
            (0x00_0004, [(DE, 0), (BC, 1)], "CN"),
            (0x00_0004, [(DE, 0), (BC, 1)], "CN"),
            // CMP H0,H1
            (0x00_0006, [(H0, 0), (H1, 1)], "CN"),
            (0x00_0006, [(H0, 0), (H1, 1)], "CN"),
            // CMP A,0xFFFF
            (0x00_0008, [], "CN"),
            (0x00_000A, [], "CN"),
            (0x00_000A, [], "CP"),
            // CMP BC,0xFFFF_FFFF
            (0x00_000C, [], "CP"),
            (0x00_000E, [], "CP"),
            (0x00_0010, [], "CP"),
            (0x00_0010, [], "CP"),
            // CMP H1,0xFF
            (0x00_0012, [], "CP"),
            (0x00_0013, [], "CP"),
            (0x00_0013, [], "CP"),
            // CMP 0x8000,A
            (0x00_0015, [], "CP"),
            (0x00_0017, [], "CP"),
            (0x00_0017, [], "O"),
            // CMP 0x8000_0000,BC
            (0x00_0019, [], "O"),
            (0x00_001B, [], "O"),
            (0x00_001D, [], "O"),
            (0x00_001D, [], "O"),
            // CMP 0x80,H1
            (0x00_001F, [], "O"),
            (0x00_0020, [], "O"),
            (0x00_0020, [], "O")
        ]
    );

    instr_test!(
        REGS: [
            (A, 0xFEDC),
            (B, 0xFEDC),
            (DE, 0x1234_5678),
            (HL, 0x1234_5678),
            (C1, 0x01),
            (C0, 0x01)
        ],
        RAM: gen_ram![
            CmpRaRb(B, A).into_opcode(),
            CmpBraBrb(DE, HL).into_opcode(),
            CmpVraVrb(C0, C1).into_opcode()
        ],
        FLAGS: "",
        [
            // CMP B,A
            (0x00_0002, [], ""),
            (0x00_0002, [], "ZP"),
            // CMP DE,HL
            (0x00_0004, [], "ZP"),
            (0x00_0004, [], "ZP"),
            // CMP C0,C1
            (0x00_0006, [], "ZP"),
            (0x00_0006, [], "ZP")
        ]
    );

    instr_test!(
        REGS: [
            (HL, 0x0012_3456),
            (A, 0x8000),
            (B, 0x0002)
        ],
        RAM: gen_ram![
            LdBraImm16(HL).into_opcode(),
            0x0001_u16,
            CmpRaBrb(A, HL).into_opcode(),
            CmpBraRb(HL, B).into_opcode()
        ],
        FLAGS: "",
        [
            // LD [HL],0x0001
            (0x00_0002, [], ""),
            (0x00_0004, [], ""),
            (0x00_0004, [], ""),
            // CMP A,[HL]
            (0x00_0006, [], ""),
            (0x00_0006, [], ""),
            (0x00_0006, [], "O"),
            // CMP [HL],B
            (0x00_0008, [], "O"),
            (0x00_0008, [], "O"),
            (0x00_0008, [], "CN")
        ]
    );
}

#[test]
fn test_bit() {
    instr_test!(
        REGS: [
            (A, 0xA00A),
            (BC, 0x00FE_DCBA)
        ],
        RAM: gen_ram![
            LdBraImm16(BC).into_opcode(),
            0x5005_u16,
            BitRaB(A,0).into_opcode(),
            BitRaB(A,1).into_opcode(),
            BitRaB(A,15).into_opcode(),
            BitBraB(BC,0).into_opcode(),
            BitBraB(BC,1).into_opcode(),
            BitBraB(BC,15).into_opcode()
        ],
        FLAGS: "CN",
        [
            // LD [BC],0x5005
            (0x00_0002, [], "CN"),
            (0x00_0004, [], "CN"),
            (0x00_0004, [], "CN"),
            // BIT A,0
            (0x00_0006, [], "CN"),
            (0x00_0006, [], "ZCN"),
            // BIT A,1
            (0x00_0008, [], "ZCN"),
            (0x00_0008, [], "CN"),
            // BIT A,15
            (0x00_000A, [], "CN"),
            (0x00_000A, [], "CN"),
            // BIT [BC],0
            (0x00_000C, [], "CN"),
            (0x00_000C, [], "CN"),
            (0x00_000C, [], "CN"),
            // BIT [BC],1
            (0x00_000E, [], "CN"),
            (0x00_000E, [], "CN"),
            (0x00_000E, [], "ZCN"),
            // BIT [BC],15
            (0x00_0010, [], "ZCN"),
            (0x00_0010, [], "ZCN"),
            (0x00_0010, [], "ZCN")
        ]
    );
}
