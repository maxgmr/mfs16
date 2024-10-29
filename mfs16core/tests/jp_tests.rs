#![cfg(test)]

use mfs16core::{Addr, Computer, Flags, Instruction::*, Ram, Reg, Reg32::*};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::instr_test;

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
