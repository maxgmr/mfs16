#![cfg(test)]

use mfs16core::{
    Addr, Computer,
    Flag::{self, *},
    Flags,
    Instruction::{self, *},
    Ram, Reg,
    Reg32::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::instr_test;

const INTENDED_ADDR: u32 = 0x12_3456;
const TRAP_ADDR: u32 = 0xFF_FFFF;

#[test]
fn test_jp() {
    let mut r = Ram::default();
    // JP 0x11_1111
    r.write_word(0x00_0000, JpImm32.into_opcode());
    r.write_dword(0x00_0002, 0x11_1111);
    // JP BC = 0x33_3333
    r.write_word(0x11_1111, JpBra(BC).into_opcode());
    // JR DE = 0x11_110F
    r.write_word(0x33_3333, JrBra(DE).into_opcode());
    // JR 0xFFBB_BBB5 (-0x44_444B)
    r.write_word(0x44_4444, JrImm32.into_opcode());
    r.write_dword(0x44_4446, 0xFFBB_BBB5);

    instr_test!(
        REGS: [
            (BC, 0x33_3333),
            (DE, 0x11_110F)
        ],
        RAM: r,
        FLAGS: "",
        [
            // JP 0x11_1111
            (0x00_0002, [], ""),
            (0x00_0004, [], ""),
            (0x00_0006, [], ""),
            (0x11_1111, [], ""),
            // JP BC
            (0x11_1113, [], ""),
            (0x33_3333, [], ""),
            // JR DE
            (0x33_3335, [], ""),
            (0x44_4444, [], ""),
            // JR 0xFFBB_BBBB
            (0x44_4446, [], ""),
            (0x44_4448, [], ""),
            (0x44_444A, [], ""),
            (0xFF_FFFF, [], "")
        ]
    );
}

#[test]
fn test_cond_jp() {
    imm_test_helper(Zero, true, JpzImm32);
    imm_test_helper(Zero, false, JnzImm32);
    imm_test_helper(Carry, true, JpcImm32);
    imm_test_helper(Carry, false, JncImm32);
    imm_test_helper(Overflow, true, JpoImm32);
    imm_test_helper(Overflow, false, JnoImm32);
    imm_test_helper(Parity, true, JppImm32);
    imm_test_helper(Parity, false, JnpImm32);
    imm_test_helper(Negative, true, JpnImm32);
    imm_test_helper(Negative, false, JnnImm32);
    bra_test_helper(Zero, true, JpzBra(HL), JpzBra(BC));
    bra_test_helper(Zero, false, JnzBra(HL), JnzBra(BC));
    bra_test_helper(Carry, true, JpcBra(HL), JpcBra(BC));
    bra_test_helper(Carry, false, JncBra(HL), JncBra(BC));
    bra_test_helper(Overflow, true, JpoBra(HL), JpoBra(BC));
    bra_test_helper(Overflow, false, JnoBra(HL), JnoBra(BC));
    bra_test_helper(Parity, true, JppBra(HL), JppBra(BC));
    bra_test_helper(Parity, false, JnpBra(HL), JnpBra(BC));
    bra_test_helper(Negative, true, JpnBra(HL), JpnBra(BC));
    bra_test_helper(Negative, false, JnnBra(HL), JnnBra(BC));
}

fn imm_test_helper(flag: Flag, expected: bool, instr: Instruction) {
    let mut c = new_jp_test_computer(expected);
    c.ram.write_word(0x00_0000, instr.into_opcode());
    c.ram.write_dword(0x00_0002, TRAP_ADDR);
    c.ram.write_word(0x00_0006, instr.into_opcode());
    c.ram.write_dword(0x00_0008, INTENDED_ADDR);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));

    // Read lsw of dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));

    // Read msw of dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));

    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));

    // Don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));

    // Conditional met; try jumping again
    c.cpu.change_flag(flag, expected);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0008));

    // Read lsw of dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000A));

    // Read msw of dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000C));

    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000C));

    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR), "Fail for {instr}");
}

fn bra_test_helper(flag: Flag, expected: bool, instr_hl: Instruction, instr_bc: Instruction) {
    let mut c = new_jp_test_computer(expected);
    c.ram.write_word(0x00_0000, instr_hl.into_opcode());
    c.ram.write_word(0x00_0002, instr_bc.into_opcode());

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));

    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));

    // Don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));

    // Conditional met; try jumping again
    c.cpu.change_flag(flag, expected);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));

    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));

    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR));
}

fn new_jp_test_computer(expected: bool) -> Computer {
    let mut c = Computer::default();
    c.cpu.set_breg(BC, INTENDED_ADDR);
    c.cpu.set_breg(HL, TRAP_ADDR);
    c.cpu.flags = if expected {
        Flags::from_string("")
    } else {
        Flags::from_string("ZCOPN")
    };
    c
}
