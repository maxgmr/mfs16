#![cfg(test)]

use mfs16core::{
    gen_mem, Addr, Computer, Flags, Instruction::*, MemWritable, Memory, Reg, Reg32::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::instr_test;

#[test]
fn test_push_pop() {
    instr_test!(
        REGS: [
            (BC, 0x12_3456),
            (DE, 0x23_4567),
            (HL, 0x34_5678)
        ],
        MEM: gen_mem![
            PushBra(BC),
            PushBra(DE),
            PushBra(HL),
            PopBra(BC),
            PopBra(DE),
            PopBra(HL)
        ],
        FLAGS: "",
        [
            // Push BC
            (0x00_0002, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0002, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Push DE
            (0x00_0004, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0004, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Push HL
            (0x00_0006, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0006, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Pop into BC
            (0x00_0008, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0008, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Pop into DE
            (0x00_000A, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_000A, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Pop into HL
            (0x00_000C, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_000C, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x12_3456)], "")
        ]
    );
}

#[test]
fn test_peek() {
    instr_test!(
        REGS: [
            (BC, 0x12_3456),
            (DE, 0x23_4567),
            (HL, 0x34_5678)
        ],
        MEM: gen_mem![
            PushBra(BC),
            PushBra(DE),
            PushBra(HL),
            PeekBra(BC),
            PopBra(BC),
            PeekBra(HL),
            PopBra(HL),
            PeekBra(DE),
            PopBra(DE)
        ],
        FLAGS: "",
        [
            // Push BC
            (0x00_0002, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0002, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Push DE
            (0x00_0004, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0004, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Push HL
            (0x00_0006, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0006, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Peek into BC
            (0x00_0008, [(BC, 0x12_3456), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_0008, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Pop into BC
            (0x00_000A, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_000A, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            // Peek into HL
            (0x00_000C, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x34_5678)], ""),
            (0x00_000C, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x23_4567)], ""),
            // Pop into HL
            (0x00_000E, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x23_4567)], ""),
            (0x00_000E, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x23_4567)], ""),
            // Peek into DE
            (0x00_0010, [(BC, 0x34_5678), (DE, 0x23_4567), (HL, 0x23_4567)], ""),
            (0x00_0010, [(BC, 0x34_5678), (DE, 0x12_3456), (HL, 0x23_4567)], ""),
            // Pop into DE
            (0x00_0012, [(BC, 0x34_5678), (DE, 0x12_3456), (HL, 0x23_4567)], ""),
            (0x00_0012, [(BC, 0x34_5678), (DE, 0x12_3456), (HL, 0x23_4567)], "")
        ]
    );
}
