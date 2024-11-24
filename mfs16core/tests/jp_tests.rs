#![cfg(test)]

use mfs16core::{
    Addr, Computer,
    Flag::{self, *},
    Flags,
    Instruction::{self, *},
    MemWritable, Memory, Reg,
    Reg16::*,
    Reg32::*,
    RAM_OFFSET, RAM_SIZE,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::{instr_test, test_computer};

const STACK_ZERO: u32 = RAM_OFFSET as u32;
const STACK_END: u32 = (RAM_OFFSET + RAM_SIZE) as u32;

const INTENDED_ADDR: u32 = 0x12_3456;
const TRAP_ADDR: u32 = 0xFF_FFFF;

#[test]
fn test_reti() {
    let mut c = test_computer();
    c.cpu.interrupts_enabled = false;
    c.mmu.write_word(0x00_0000, CallImm32.into_opcode());
    c.mmu.write_dword(0x00_0002, 0x12_3456);
    c.mmu.write_word(0x12_3456, 0x811E);

    // CALL IMM32
    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));
    assert!(!c.cpu.interrupts_enabled);
    // Read imm32
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));
    assert!(!c.cpu.interrupts_enabled);
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));
    assert!(!c.cpu.interrupts_enabled);
    // Push addr to stack
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));
    assert!(!c.cpu.interrupts_enabled);
    // Jump
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x12_3456));
    assert!(!c.cpu.interrupts_enabled);

    // RETI
    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x12_3458));
    assert!(!c.cpu.interrupts_enabled);
    // Pop addr off stack + return
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));
    assert!(!c.cpu.interrupts_enabled);
    // Enable interrupts
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));
    assert!(c.cpu.interrupts_enabled);
}

#[test]
fn test_call_ret() {
    let mut m = Memory::default();
    CallImm32.mem_write(&mut m, 0x00_0000);
    0x12_3456_u32.mem_write(&mut m, 0x00_0002);
    CallBra(BC).mem_write(&mut m, 0x00_0006);
    LdRaImm16(A).mem_write(&mut m, 0x12_3456);
    0xBABE_u16.mem_write(&mut m, 0x12_3458);
    Ret.mem_write(&mut m, 0x12_345A);
    LdRaImm16(A).mem_write(&mut m, 0x65_4321);
    0xCAFE_u16.mem_write(&mut m, 0x65_4323);
    CallBra(DE).mem_write(&mut m, 0x65_4325);
    Ret.mem_write(&mut m, 0x65_4327);
    Ret.mem_write(&mut m, 0x0E_DCBA);

    instr_test!(
        REGS: [
            (BC, 0x65_4321),
            (DE, 0x0E_DCBA),
            (A, 0x0000)
        ],
        MEM: m,
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
            (0x0E_DCBA, [(A, 0xCAFE)], ""),
            // Ret
            (0x0E_DCBC, [(A, 0xCAFE)], ""),
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
    c.mmu.write_word(0x00_0000, call_instr_hl.into_opcode());
    c.mmu.write_word(0x00_0002, call_instr_bc.into_opcode());

    // Read unsatisfied call instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));
    // Evaluate conditional- don't push to stack or jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));
    assert_eq!(c.cpu.sp.address(), STACK_ZERO);

    // Conditional met; try calling again
    c.cpu.change_flag(flag, expected);

    // Read satisfied call instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));
    // Push to stack!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);
    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);
}

fn cond_call_ret_imm_helper(
    flag: Flag,
    expected: bool,
    call_instr: Instruction,
    ret_instr: Instruction,
) {
    let mut c = new_jp_test_computer(expected);
    c.mmu.write_word(0x00_0000, call_instr.into_opcode());
    c.mmu.write_dword(0x00_0002, TRAP_ADDR);
    c.mmu.write_word(0x00_0006, call_instr.into_opcode());
    c.mmu.write_dword(0x00_0008, INTENDED_ADDR);
    c.mmu.write_word(INTENDED_ADDR, ret_instr.into_opcode());
    c.mmu.write_word(INTENDED_ADDR + 2, ret_instr.into_opcode());

    // Read unsatisfied call instr + dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));
    // Don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));
    assert_eq!(c.cpu.sp.address(), STACK_ZERO);

    // Conditional met; try calling again
    c.cpu.change_flag(flag, expected);

    // Read satisfied call instr + dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0008));
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0008));
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000A));
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000C));
    // Push to stack!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000C));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);
    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);

    // Conditional no longer met; ret shouldn't work
    c.cpu.change_flag(flag, !expected);

    // Read unsatisfied ret instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR + 2));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);
    // Evaluate conditional- don't return!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR + 2));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);

    // Conditional now met; should ret
    c.cpu.change_flag(flag, expected);

    // Read satisfied ret instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR + 4));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);
    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR + 4));
    assert_eq!(c.cpu.sp.address(), STACK_END - 4);
    // Return!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000C));
    assert_eq!(c.cpu.sp.address(), STACK_ZERO);
}

#[test]
fn test_jp() {
    let mut m = Memory::default();
    // JP 0x11_1111
    m.write_word(0x00_0000, JpImm32.into_opcode());
    m.write_dword(0x00_0002, 0x11_1111);
    // JP BC = 0x33_3333
    m.write_word(0x11_1111, JpBra(BC).into_opcode());
    // JR DE = 0x11_110F
    m.write_word(0x33_3333, JrBra(DE).into_opcode());
    // JR 0xFFBB_BBB5 (-0x44_444B)
    m.write_word(0x44_4444, JrImm32.into_opcode());
    m.write_dword(0x44_4446, 0xFFBB_BBB5);

    instr_test!(
        REGS: [
            (BC, 0x33_3333),
            (DE, 0x11_110F)
        ],
        MEM: m,
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
            (0xFFFF_FFFF_u32, [], "")
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
    c.mmu.write_word(0x00_0000, instr.into_opcode());
    c.mmu.write_dword(0x00_0002, TRAP_ADDR);
    c.mmu.write_word(0x00_0006, instr.into_opcode());
    c.mmu.write_dword(0x00_0008, INTENDED_ADDR);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));

    // Evaluate conditional- don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0006));

    // Conditional met; try jumping again
    c.cpu.change_flag(flag, expected);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0008));

    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0008));

    // Read lsw of dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000A));

    // Read msw of dword
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_000C));

    // Jump!
    c.cycle();
    assert_eq!(
        c.cpu.pc,
        Addr::new_default_range(INTENDED_ADDR),
        "Fail for {instr}"
    );
}

fn bra_test_helper(flag: Flag, expected: bool, instr_hl: Instruction, instr_bc: Instruction) {
    let mut c = new_jp_test_computer(expected);
    c.mmu.write_word(0x00_0000, instr_hl.into_opcode());
    c.mmu.write_word(0x00_0002, instr_bc.into_opcode());

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));

    // Evaluate conditional- don't jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));

    // Conditional met; try jumping again
    c.cpu.change_flag(flag, expected);

    // Read instr
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));

    // Evaluate conditional
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));

    // Jump!
    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(INTENDED_ADDR));
}

fn new_jp_test_computer(expected: bool) -> Computer {
    let mut c = Computer::default();
    c.mmu.rom.set_writable(true);
    c.cpu.set_breg(BC, INTENDED_ADDR);
    c.cpu.set_breg(HL, TRAP_ADDR);
    c.cpu.flags = if expected {
        Flags::from_string("")
    } else {
        Flags::from_string("ZCOPN")
    };
    c
}
