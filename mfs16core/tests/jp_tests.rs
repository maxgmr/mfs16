#![cfg(test)]

use mfs16core::{
    Addr, Computer,
    Flag::{self, *},
    Flags,
    Instruction::{self, *},
    Ram, RamWritable, Reg,
    Reg16::*,
    Reg32::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::instr_test;

const INTENDED_ADDR: u32 = 0x12_3456;
const TRAP_ADDR: u32 = 0xFF_FFFF;

#[test]
fn test_call_ret() {
    let mut r = Ram::default();
    CallImm32.ram_write(&mut r, 0x00_0000);
    0x12_3456_u32.ram_write(&mut r, 0x00_0002);
    CallBra(BC).ram_write(&mut r, 0x00_0006);
    LdRaImm16(A).ram_write(&mut r, 0x12_3456);
    0xBABE_u16.ram_write(&mut r, 0x12_3458);
    Ret.ram_write(&mut r, 0x12_345A);
    LdRaImm16(A).ram_write(&mut r, 0x65_4321);
    0xCAFE_u16.ram_write(&mut r, 0x65_4323);
    CallBra(DE).ram_write(&mut r, 0x65_4325);
    Ret.ram_write(&mut r, 0x65_4327);
    Ret.ram_write(&mut r, 0xFE_DCBA);

    instr_test!(
        REGS: [
            (BC, 0x65_4321),
            (DE, 0xFE_DCBA),
            (A, 0x0000)
        ],
        RAM: r,
        FLAGS: "",
        [
            // Call 0x12_3456
            (0x00_0002, [(A, 0x0000)], ""),
            (0x00_0004, [(A, 0x0000)], ""),
            (0x00_0006, [(A, 0x0000)], ""),
            (0x00_0006, [(A, 0x0000)], ""),
            (0x12_3456, [(A, 0x0000)], ""),
            // Ld A,0xBABE
            (0x12_3458, [(A, 0x0000)], ""),
            (0x12_345A, [(A, 0x0000)], ""),
            (0x12_345A, [(A, 0xBABE)], ""),
            // Ret
            (0x12_345C, [(A, 0xBABE)], ""),
            (0x00_0006, [(A, 0xBABE)], ""),
            // Call BC
            (0x00_0008, [(A, 0xBABE)], ""),
            (0x00_0008, [(A, 0xBABE)], ""),
            (0x65_4321, [(A, 0xBABE)], ""),
            // Ld A,0xCAFE
            (0x65_4323, [(A, 0xBABE)], ""),
            (0x65_4325, [(A, 0xBABE)], ""),
            (0x65_4325, [(A, 0xCAFE)], ""),
            // Call DE
            (0x65_4327, [(A, 0xCAFE)], ""),
            (0x65_4327, [(A, 0xCAFE)], ""),
            (0xFE_DCBA, [(A, 0xCAFE)], ""),
            // Ret
            (0xFE_DCBC, [(A, 0xCAFE)], ""),
            (0x65_4327, [(A, 0xCAFE)], ""),
            // Ret
            (0x65_4329, [(A, 0xCAFE)], ""),
            (0x00_0008, [(A, 0xCAFE)], "")

        ]
    );
}

#[test]
fn test_cond_call_ret() {
    cond_call_ret_imm_helper(Zero, true, ClzImm32, Rtz);
    cond_call_ret_imm_helper(Zero, false, CnzImm32, Rnz);
    cond_call_ret_imm_helper(Carry, true, ClcImm32, Rtc);
    cond_call_ret_imm_helper(Carry, false, CncImm32, Rnc);
    cond_call_ret_imm_helper(Overflow, true, CloImm32, Rto);
    cond_call_ret_imm_helper(Overflow, false, CnoImm32, Rno);
    cond_call_ret_imm_helper(Parity, true, ClpImm32, Rtp);
    cond_call_ret_imm_helper(Parity, false, CnpImm32, Rnp);
    cond_call_ret_imm_helper(Negative, true, ClnImm32, Rtn);
    cond_call_ret_imm_helper(Negative, false, CnnImm32, Rnn);

    cond_call_ret_bra_helper(Zero, true, ClzBra(BC), ClzBra(HL));
    cond_call_ret_bra_helper(Zero, false, CnzBra(BC), CnzBra(HL));
    cond_call_ret_bra_helper(Carry, true, ClcBra(BC), ClcBra(HL));
    cond_call_ret_bra_helper(Carry, false, CncBra(BC), CncBra(HL));
    cond_call_ret_bra_helper(Overflow, true, CloBra(BC), CloBra(HL));
    cond_call_ret_bra_helper(Overflow, false, CnoBra(BC), CnoBra(HL));
    cond_call_ret_bra_helper(Parity, true, ClpBra(BC), ClpBra(HL));
    cond_call_ret_bra_helper(Parity, false, CnpBra(BC), CnpBra(HL));
    cond_call_ret_bra_helper(Negative, true, ClnBra(BC), ClnBra(HL));
    cond_call_ret_bra_helper(Negative, false, CnnBra(BC), CnnBra(HL));
}

fn cond_call_ret_bra_helper(
    flag: Flag,
    expected: bool,
    call_instr_bc: Instruction,
    call_instr_hl: Instruction,
) {
    let mut c = new_jp_test_computer(expected);
    c.cpu.set_breg(BC, INTENDED_ADDR);
    c.cpu.set_breg(HL, TRAP_ADDR);
    c.ram.write_word(0x00_0000, call_instr_hl.into_opcode());
    c.ram.write_word(0x00_0002, call_instr_bc.into_opcode());
    c.cpu.sp = Addr::new(0x00_0000);

    // Read unsatisfied call instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));
    // Don't push to stack!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));
    assert_eq!(c.cpu.sp, Addr::new(0x00_0000));
    // Don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));

    // Conditional met; try calling again
    c.cpu.change_flag(flag, expected);

    // Read satisfied call instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));
    // Push to stack!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
}

fn cond_call_ret_imm_helper(
    flag: Flag,
    expected: bool,
    call_instr: Instruction,
    ret_instr: Instruction,
) {
    let mut c = new_jp_test_computer(expected);
    c.ram.write_word(0x00_0000, call_instr.into_opcode());
    c.ram.write_dword(0x00_0002, TRAP_ADDR);
    c.ram.write_word(0x00_0006, call_instr.into_opcode());
    c.ram.write_dword(0x00_0008, INTENDED_ADDR);
    c.ram.write_word(INTENDED_ADDR, ret_instr.into_opcode());
    c.ram.write_word(INTENDED_ADDR + 2, ret_instr.into_opcode());
    c.cpu.sp = Addr::new(0x00_0000);

    // Read unsatisfied call instr + dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0002));
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0004));
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));
    // Don't push to stack!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));
    assert_eq!(c.cpu.sp, Addr::new(0x00_0000));
    // Don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0006));

    // Conditional met; try calling again
    c.cpu.change_flag(flag, expected);

    // Read satisfied call instr + dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_0008));
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000A));
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000C));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000C));
    // Push to stack!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000C));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));

    // Conditional no longer met; ret shouldn't work
    c.cpu.change_flag(flag, !expected);

    // Read unsatisfied ret instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR + 2));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR + 2));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
    // Don't return!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR + 2));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));

    // Conditional now met; should ret
    c.cpu.change_flag(flag, expected);

    // Read satisfied ret instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR + 4));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(INTENDED_ADDR + 4));
    assert_eq!(c.cpu.sp, Addr::new(0xFF_FFFC));
    // Return!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new(0x00_000C));
    assert_eq!(c.cpu.sp, Addr::new(0x00_0000));
}

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
