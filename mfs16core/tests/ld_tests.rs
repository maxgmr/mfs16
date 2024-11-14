#![cfg(test)]

use mfs16core::{
    gen_mem, Addr, Computer, Flags, Instruction::*, MemWritable, Memory, Reg, Reg16::*, Reg32::*,
    Reg8::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::{instr_test, test_computer};

#[test]
fn test_ld() {
    let mut c = test_computer();
    let flags = c.cpu.flags.clone();
    // LD A,B
    c.mmu.write_word(0x00_0000, 0x0101);
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
    c.mmu.write_word(0x00_0002, 0x02A9);
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
    c.mmu.write_word(0x00_0004, 0x0304);
    c.mmu.write_word(0x00_0006, 0x3865);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));
    assert_eq!(c.cpu.reg(E), 0x0034);

    // Read immediate word
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0008));
    assert_eq!(c.cpu.reg(E), 0x0034);

    // Write immediate word into register
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0008));
    assert_eq!(c.cpu.reg(E), 0x3865);

    // LD E1,0x9E
    c.mmu.write_word(0x00_0008, 0x0328);
    c.mmu.write_byte(0x00_000A, 0x9E);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000A));
    assert_eq!(c.cpu.vreg(E1), 0x38);

    // Read immediate byte
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000B));
    assert_eq!(c.cpu.vreg(E1), 0x38);

    // Write immediate byte into register
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000B));
    assert_eq!(c.cpu.vreg(E1), 0x9E);

    // LD [DE],L
    c.cpu.set_breg(DE, 0x00FE_DCBA);
    c.cpu.set_reg(L, 0x1234);
    c.mmu.write_word(0x00_000B, 0x0416);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000D));
    assert_eq!(c.mmu.read_word(0x00FE_DCBA), 0x0000);

    // Get memory location
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000D));
    assert_eq!(c.mmu.read_word(0x00FE_DCBA), 0x0000);

    // Write L to memory location
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000D));
    assert_eq!(c.mmu.read_word(0x00FE_DCBA), 0x1234);

    // LD HL,DE
    c.cpu.set_breg(HL, 0x0000_0000);
    c.cpu.set_breg(DE, 0xDEAD_BEEF);
    c.mmu.write_word(0x00_000D, 0x0198);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000F));
    assert_eq!(c.cpu.breg(HL), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000F));
    assert_eq!(c.cpu.breg(HL), 0xDEAD_BEEF);
    assert_eq!(c.cpu.breg(DE), 0xDEAD_BEEF);

    // LD BC,0x0DEF_ACED
    c.cpu.set_breg(BC, 0x0000_0000);
    c.mmu.write_word(0x00_000F, 0x0310);
    c.mmu.write_word(0x00_0011, 0xACED);
    c.mmu.write_word(0x00_0013, 0x0DEF);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0011));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);

    // Read first word
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0013));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);

    // Read second word
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0015));
    assert_eq!(c.cpu.breg(BC), 0x0000_0000);

    // Write to BC
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0015));
    assert_eq!(c.cpu.breg(BC), 0x0DEF_ACED);

    // LD [DE],0xBABE
    c.cpu.set_breg(DE, 0x67_89AB);
    c.mmu.write_word(0x67_89AB, 0x0000);
    c.mmu.write_word(0x00_0015, 0x0331);
    c.mmu.write_word(0x00_0017, 0xBABE);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0017));
    assert_eq!(c.mmu.read_word(0x67_89AB), 0x0000);

    // Read immediate word
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0019));
    assert_eq!(c.mmu.read_word(0x67_89AB), 0x0000);

    // Write immediate word to pointed-to value
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0019));
    assert_eq!(c.mmu.read_word(0x67_89AB), 0xBABE);

    // LD D,[BC]
    c.cpu.set_breg(BC, 0x0000_CAFE);
    c.cpu.set_reg(D, 0x0000);
    c.mmu.write_word(0x0000_CAFE, 0xC001);
    c.mmu.write_word(0x00_0019, 0x0530);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001B));
    assert_eq!(c.cpu.reg(D), 0x0000);

    // Get [BC]
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001B));
    assert_eq!(c.cpu.reg(D), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001B));
    assert_eq!(c.cpu.reg(D), 0xC001);

    // LDI [BC],A
    c.cpu.set_breg(BC, 0x0F_FFFE);
    c.cpu.set_reg(A, 0xD00D);
    c.mmu.write_word(0x0F_FFFE, 0x0000);
    c.mmu.write_word(0x00_001B, 0x0600);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001D));
    assert_eq!(c.cpu.breg(BC), 0x0F_FFFE);
    assert_eq!(c.mmu.read_word(0x0F_FFFE), 0x0000);

    // Get A
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001D));
    assert_eq!(c.cpu.breg(BC), 0x0F_FFFE);
    assert_eq!(c.mmu.read_word(0x0F_FFFE), 0x0000);

    // Set [BC] and increment BC
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001D));
    assert_eq!(c.cpu.breg(BC), 0x10_0000);
    assert_eq!(c.mmu.read_word(0x0F_FFFE), 0xD00D);

    // LDD [BC],D
    c.cpu.set_reg(D, 0xF1F1);
    c.mmu.write_word(0x10_0000, 0x0000);
    c.mmu.write_word(0x00_001D, 0x0703);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001F));
    assert_eq!(c.cpu.breg(BC), 0x10_0000);
    assert_eq!(c.mmu.read_word(0x10_0000), 0x0000);

    // Get D
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001F));
    assert_eq!(c.cpu.breg(BC), 0x10_0000);
    assert_eq!(c.mmu.read_word(0x10_0000), 0x0000);

    // Set [BC] and decrement BC
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_001F));
    assert_eq!(c.cpu.breg(BC), 0x0F_FFFE);
    assert_eq!(c.mmu.read_word(0x10_0000), 0xF1F1);

    // LDI H,[DE]
    c.cpu.set_reg(H, 0xDEAD);
    c.cpu.set_breg(DE, 0xF1_F1F1);
    c.mmu.write_word(0xF1_F1F1, 0x1234);
    c.mmu.write_word(0x00_001F, 0x0851);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0021));
    assert_eq!(c.cpu.breg(DE), 0xF1_F1F1);
    assert_eq!(c.cpu.reg(H), 0xDEAD);

    // Get [DE]
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0021));
    assert_eq!(c.cpu.breg(DE), 0xF1_F1F1);
    assert_eq!(c.cpu.reg(H), 0xDEAD);

    // Set H and increment DE
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0021));
    assert_eq!(c.cpu.breg(DE), 0xF1_F1F3);
    assert_eq!(c.cpu.reg(H), 0x1234);

    // LDD L,[DE]
    c.cpu.set_reg(L, 0xDEAD);
    c.mmu.write_word(0xF1_F1F3, 0xDEDE);
    c.mmu.write_word(0x00_0021, 0x0961);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0023));
    assert_eq!(c.cpu.breg(DE), 0xF1_F1F3);
    assert_eq!(c.cpu.reg(L), 0xDEAD);

    // Get [DE]
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0023));
    assert_eq!(c.cpu.breg(DE), 0xF1_F1F3);
    assert_eq!(c.cpu.reg(L), 0xDEAD);

    // Set H and increment DE
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0023));
    assert_eq!(c.cpu.breg(DE), 0xF1_F1F1);
    assert_eq!(c.cpu.reg(L), 0xDEDE);

    // LD SP,imm32
    c.cpu.sp = Addr::new_default_range(0xFF_FFFF);
    c.mmu.write_word(0x00_0023, 0x01A0);
    c.mmu.write_word(0x00_0025, 0x5678);
    c.mmu.write_word(0x00_0027, 0x0034);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0025));
    assert_eq!(c.cpu.sp, Addr::new_default_range(0xFF_FFFF));

    // Read msw
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0027));
    assert_eq!(c.cpu.sp, Addr::new_default_range(0xFF_FFFF));

    // Read lsw
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0029));
    assert_eq!(c.cpu.sp, Addr::new_default_range(0xFF_FFFF));

    // Set SP
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0029));
    assert_eq!(c.cpu.sp, Addr::new_default_range(0x0034_5678));

    // LD imm32,SP
    c.cpu.sp = Addr::new_default_range(0xDC_BA98);
    // Ensure that things are actually being stored little-endian
    c.mmu.rom.contents[0x00_0029] = 0xA1;
    c.mmu.rom.contents[0x00_002A] = 0x01;
    c.mmu.rom.contents[0x00_002B] = 0x45;
    c.mmu.rom.contents[0x00_002C] = 0x23;
    c.mmu.rom.contents[0x00_002D] = 0x01;
    c.mmu.rom.contents[0x00_002E] = 0x00;
    c.mmu.write_word(0x01_2345, 0x0000);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_002B));
    assert_eq!(c.mmu.read_word(0x01_2345), 0x0000);

    // Read msw
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_002D));
    assert_eq!(c.mmu.read_word(0x01_2345), 0x0000);

    // Read lsw
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_002F));
    assert_eq!(c.mmu.read_word(0x01_2345), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_002F));
    // Ensure that things are actually being stored little-endian
    assert_eq!(c.mmu.rom.contents[0x01_2345], 0x98);
    assert_eq!(c.mmu.rom.contents[0x01_2346], 0xBA);
    assert_eq!(c.mmu.rom.contents[0x01_2347], 0xDC);
    assert_eq!(c.mmu.rom.contents[0x01_2348], 0x00);

    // LD SP,DE
    c.cpu.set_breg(DE, 0x00E1_D2C3);
    c.cpu.sp = Addr::new_default_range(0xFF_FFFF);
    c.mmu.write_word(0x00_002F, 0x01B1);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0031));
    assert_eq!(c.cpu.sp, Addr::new_default_range(0xFF_FFFF));

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0031));
    assert_eq!(c.cpu.sp, Addr::new_default_range(0x00E1_D2C3));

    // LD HL,SP
    instr_test!(
        REGS: [(HL, 0x1234_5678)],
        MEM: gen_mem![
            LdBraSp(HL).into_opcode()
        ],
        FLAGS: "",
        [
            (0x00_0002, [(HL, 0x1234_5678)], ""),
            (0x00_0002, [(HL, 0x80_0000)], "")
        ]
    );

    // LDR A,0x12_3455+HL
    // LDR B,0x12_3457+HL
    instr_test!(
        REGS: [
            (HL,0x0000_0001),
            (A,0x0000),
            (B,0x0000)
        ],
        MEM: gen_mem![
            LdBraImm32(DE),
            0x12_3456_u32,
            LdBraImm16(DE),
            0xBABE_u16,
            LdrRaImm32(A),
            0x12_3455_u32,
            LdBraImm32(HL),
            0xFFFF_FFFF_u32,
            LdrRaImm32(B),
            0x12_3457_u32
        ],
        FLAGS: "",
        [
            // load 0xBABE into mmu at address 0x12_3456
            (0x00_0002, [], ""),
            (0x00_0004, [], ""),
            (0x00_0006, [], ""),
            (0x00_0006, [], ""),

            (0x00_0008, [], ""),
            (0x00_000A, [], ""),
            (0x00_000A, [], ""),

            // LDR A,[0x12_3455 + 1]
            (0x00_000C, [(A, 0)], ""),
            (0x00_000E, [(A, 0)], ""),
            (0x00_0010, [(A, 0)], ""),
            (0x00_0010, [(A, 0)], ""),
            (0x00_0010, [(A, 0xBABE)], ""),

            // set HL to -1
            (0x00_0012, [(HL, 1)], ""),
            (0x00_0014, [(HL, 1)], ""),
            (0x00_0016, [(HL, 1)], ""),
            (0x00_0016, [(HL, 0xFFFF_FFFF_u32)], ""),

            // LDR B,[0x12_3457 - 1]
            (0x00_0018, [(B, 0)], ""),
            (0x00_001A, [(B, 0)], ""),
            (0x00_001C, [(B, 0)], ""),
            (0x00_001C, [(B, 0)], ""),
            (0x00_001C, [(B, 0xBABE)], "")
        ]
    );

    // LDI [BC],0xCAFE
    c.cpu.set_breg(BC, 0xAB_ABAB);
    c.mmu.write_word(0xAB_ABAB, 0x0000);
    c.mmu.write_word(0x00_0031, 0x0970);
    c.mmu.write_word(0x00_0033, 0xCAFE);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0033));
    assert_eq!(c.cpu.breg(BC), 0xAB_ABAB);
    assert_eq!(c.mmu.read_word(0xAB_ABAB), 0x0000);

    // Read immediate word
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0035));
    assert_eq!(c.cpu.breg(BC), 0xAB_ABAB);
    assert_eq!(c.mmu.read_word(0xAB_ABAB), 0x0000);

    // Write immediate word to pointed-to value, dbl inc BC
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0035));
    assert_eq!(c.cpu.breg(BC), 0xAB_ABAD);
    assert_eq!(c.mmu.read_word(0xAB_ABAB), 0xCAFE);

    // LDD [DE],0xF00D
    c.cpu.set_breg(DE, 0xFE_FEFE);
    c.mmu.write_word(0xFE_FEFE, 0x0000);
    c.mmu.write_word(0x00_0035, 0x0981);
    c.mmu.write_word(0x00_0037, 0xF00D);

    // Read instruction
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0037));
    assert_eq!(c.cpu.breg(DE), 0xFE_FEFE);
    assert_eq!(c.mmu.read_word(0xFE_FEFE), 0x0000);

    // Read immediate word
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0039));
    assert_eq!(c.cpu.breg(DE), 0xFE_FEFE);
    assert_eq!(c.mmu.read_word(0xFE_FEFE), 0x0000);

    // Write immediate word to pointed-to value, dbl dec DE
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0039));
    assert_eq!(c.cpu.breg(DE), 0xFE_FEFC);
    assert_eq!(c.mmu.read_word(0xFE_FEFE), 0xF00D);

    // VLD [bra],brb
    c.cpu.set_breg(BC, 0x1234_5678);
    c.cpu.set_breg(HL, 0x0100_0000);
    c.mmu.write_dword(0x0100_0000, 0x0000_0000);
    c.mmu.write_word(0x00_0039, 0x0A20);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_003B));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0100_0000);
    assert_eq!(c.mmu.read_dword(0x0100_0000), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_003B));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0100_0000);
    assert_eq!(c.mmu.read_dword(0x0100_0000), 0x1234_5678);

    // VLD [bra],brb outside of VRAM
    c.cpu.set_breg(BC, 0x1234_5678);
    c.cpu.set_breg(HL, 0x0080_0000);
    c.mmu.write_dword(0x0080_0000, 0x0000_0000);
    c.mmu.write_word(0x00_003B, 0x0A20);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_003D));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0080_0000);
    assert_eq!(c.mmu.read_dword(0x0080_0000), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_003D));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0080_0000);
    assert_eq!(c.mmu.read_dword(0x0080_0000), 0x0000_0000);

    // VLDI [bra],brb
    c.cpu.set_breg(BC, 0x1234_5678);
    c.cpu.set_breg(HL, 0x0100_BEEF);
    c.mmu.write_dword(0x0100_BEEF, 0x0000_0000);
    c.mmu.write_word(0x00_003D, 0x0B20);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_003F));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0100_BEEF);
    assert_eq!(c.mmu.read_dword(0x0100_BEEF), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_003F));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0100_BEF3);
    assert_eq!(c.mmu.read_dword(0x0100_BEEF), 0x1234_5678);

    // VLDD [bra],brb
    c.mmu.write_dword(0x0100_BEF3, 0x0000_0000);
    c.mmu.write_word(0x00_003F, 0x0C20);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0041));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0100_BEF3);
    assert_eq!(c.mmu.read_dword(0x0100_BEF3), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0041));
    assert_eq!(c.cpu.breg(BC), 0x1234_5678);
    assert_eq!(c.cpu.breg(HL), 0x0100_BEEF);
    assert_eq!(c.mmu.read_dword(0x0100_BEF3), 0x1234_5678);

    // VLD [bra],imm32
    c.cpu.set_breg(HL, 0x0100_BABE);
    c.mmu.write_dword(0x0100_BABE, 0x0000_0000);
    c.mmu.write_word(0x00_0041, 0x0C32);
    c.mmu.write_dword(0x00_0043, 0x1234_5678);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0043));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x0000_0000);

    // Read word 1
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0045));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x0000_0000);

    // Read word 0
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0047));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0047));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x1234_5678);

    // VLDI [bra],imm32
    c.mmu.write_dword(0x0100_BABE, 0x0000_0000);
    c.mmu.write_word(0x00_0047, 0x0C42);
    c.mmu.write_dword(0x00_0049, 0x1234_5678);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0049));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x0000_0000);

    // Read word 1
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_004B));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x0000_0000);

    // Read word 0
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_004D));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_004D));
    assert_eq!(c.cpu.breg(HL), 0x0100_BAC2);
    assert_eq!(c.mmu.read_dword(0x0100_BABE), 0x1234_5678);

    // VLDD [bra],imm32
    c.mmu.write_dword(0x0100_BAC2, 0x0000_0000);
    c.mmu.write_word(0x00_004D, 0x0C52);
    c.mmu.write_dword(0x00_004F, 0x1234_5678);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_004F));
    assert_eq!(c.cpu.breg(HL), 0x0100_BAC2);
    assert_eq!(c.mmu.read_dword(0x0100_BAC2), 0x0000_0000);

    // Read word 1
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0051));
    assert_eq!(c.cpu.breg(HL), 0x0100_BAC2);
    assert_eq!(c.mmu.read_dword(0x0100_BAC2), 0x0000_0000);

    // Read word 0
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0053));
    assert_eq!(c.cpu.breg(HL), 0x0100_BAC2);
    assert_eq!(c.mmu.read_dword(0x0100_BAC2), 0x0000_0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0053));
    assert_eq!(c.cpu.breg(HL), 0x0100_BABE);
    assert_eq!(c.mmu.read_dword(0x0100_BAC2), 0x1234_5678);

    // LD [imm32],A
    c.mmu.write_word(0x34_5678, 0x0000);
    c.cpu.set_reg(A, 0xFEED);
    c.mmu.write_word(0x00_0053, 0x0990);
    c.mmu.write_dword(0x00_0055, 0x34_5678);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0055));
    assert_eq!(c.cpu.reg(A), 0xFEED);
    assert_eq!(c.mmu.read_word(0x34_5678), 0x0000);

    // Read word 1
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0057));
    assert_eq!(c.cpu.reg(A), 0xFEED);
    assert_eq!(c.mmu.read_word(0x34_5678), 0x0000);

    // Read word 0
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0059));
    assert_eq!(c.cpu.reg(A), 0xFEED);
    assert_eq!(c.mmu.read_word(0x34_5678), 0x0000);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0059));
    assert_eq!(c.cpu.reg(A), 0xFEED);
    assert_eq!(c.mmu.read_word(0x34_5678), 0xFEED);

    // LD [imm32],A
    c.cpu.set_reg(B, 0x0000);
    c.mmu.write_word(0x00_0059, 0x09A1);
    c.mmu.write_dword(0x00_005B, 0x34_5678);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_005B));
    assert_eq!(c.cpu.reg(B), 0x0000);
    assert_eq!(c.mmu.read_word(0x34_5678), 0xFEED);

    // Read word 1
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_005D));
    assert_eq!(c.cpu.reg(B), 0x0000);
    assert_eq!(c.mmu.read_word(0x34_5678), 0xFEED);

    // Read word 0
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_005F));
    assert_eq!(c.cpu.reg(B), 0x0000);
    assert_eq!(c.mmu.read_word(0x34_5678), 0xFEED);

    // Do operation
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_005F));
    assert_eq!(c.cpu.reg(B), 0xFEED);
    assert_eq!(c.mmu.read_word(0x34_5678), 0xFEED);
}
