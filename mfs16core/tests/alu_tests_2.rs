use mfs16core::{
    gen_mem, Addr, Computer, Flags, Instruction::*, MemWritable, Memory, Reg, Reg16::*, Reg32::*,
    Reg8::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::instr_test;

#[test]
fn test_mulu() {
    instr_test!(
        REGS: [
            (H, 0x0101),
            (L, 0x00FF),
            (BC, 0x0000_0002),
            (DE, 0x0000_0004),
            (A1, 0x06),
            (A0, 0x06)
        ],
        MEM: gen_mem![
            MuluRaRb(H, L),
            MuluBraBrb(BC, DE),
            MuluVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 0x0101), (L, 0x00FF)], ""),
            (0x00_0002, [(H, 0xFFFF), (L, 0x00FF)], "N"),
            (0x00_0004, [(BC, 2), (DE, 4)], "N"),
            (0x00_0004, [(BC, 8), (DE, 4)], "P"),
            (0x00_0006, [(A1, 6), (A0, 6)], "P"),
            (0x00_0006, [(A1, 36), (A0, 6)], "P")
        ]
    );

    instr_test!(
        REGS: [
            (H, 0xFFFF),
            (L, 0),
            (BC, 1),
            (DE, 0),
            (A1, 0),
            (A0, 0)
        ],
        MEM: gen_mem![
            MuluRaRb(H, L),
            Raf,
            MuluBraBrb(BC, DE),
            Raf,
            MuluVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 0xFFFF), (L, 0)], ""),
            (0x00_0002, [(H, 0), (L, 0)], "ZP"),
            (0x00_0004, [], "ZP"),
            (0x00_0004, [], ""),
            (0x00_0006, [(BC, 1), (DE, 0)], ""),
            (0x00_0006, [(BC, 0), (DE, 0)], "ZP"),
            (0x00_0008, [], "ZP"),
            (0x00_0008, [], ""),
            (0x00_000A, [(A1, 0), (A0, 0)], ""),
            (0x00_000A, [(A1, 0), (A0, 0)], "ZP")
        ]
    );

    instr_test!(
        REGS: [
            (H, 0x0102),
            (L, 0x00FF),
            (BC, 0xFFFF_FFFF),
            (DE, 0x0000_0002),
            (A1, 0xFF),
            (A0, 0xFF)
        ],
        MEM: gen_mem![
            MuluRaRb(H, L),
            Raf,
            MuluBraBrb(BC, DE),
            Raf,
            MuluVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 0x0102), (L, 0x00FF)], ""),
            (0x00_0002, [(H, 0x00FE), (L, 0x00FF)], "CP"),
            (0x00_0004, [], "CP"),
            (0x00_0004, [], ""),
            (0x00_0006, [(BC, 0xFFFF_FFFF_u32), (DE, 2)], ""),
            (0x00_0006, [(BC, 0xFFFF_FFFE_u32), (DE, 2)], "CPN"),
            (0x00_0008, [], "CPN"),
            (0x00_0008, [], ""),
            (0x00_000A, [(A1, 255), (A0, 255)], ""),
            (0x00_000A, [(A1, 1), (A0, 255)], "C")
        ]
    );

    let mut m = gen_mem![
        MuluRaBrb(A, BC),
        MuluRaImm16(D),
        2_u16,
        MuluBraImm32(HL),
        3_u32,
        MuluVraImm8(E1),
        0_u8
    ];
    m.write_word(0x0012_3456, 4);
    instr_test!(
        REGS: [
            (A, 4),
            (BC, 0x0012_3456),
            (D, 3),
            (HL, 9),
            (E1, 0x12)
        ],
        MEM: m,
        FLAGS: "",
        [
            (0x00_0002, [(A, 4), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 4), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 16), (BC, 0x12_3456)], "P"),
            (0x00_0004, [(D, 3)], "P"),
            (0x00_0006, [(D, 3)], "P"),
            (0x00_0006, [(D, 6)], "P"),
            (0x00_0008, [(HL, 9)], "P"),
            (0x00_000A, [(HL, 9)], "P"),
            (0x00_000C, [(HL, 9)], "P"),
            (0x00_000C, [(HL, 27)], ""),
            (0x00_000E, [(E1, 0x12)], ""),
            (0x00_000F, [(E1, 0x12)], ""),
            (0x00_000F, [(E1, 0x0)], "ZP")
        ]
    );
}

#[test]
fn test_muli() {
    instr_test!(
        REGS: [
            (H, 0xFFFB),
            (L, 0x0000),
            (BC, 0x0000_0002),
            (DE, 0x0000_0004),
            (A1, 0x01),
            (A0, 0xFF)
        ],
        MEM: gen_mem![
            MuliRaRb(H, L),
            MuliBraBrb(BC, DE),
            MuliVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 0xFFFB), (L, 0)], ""),
            (0x00_0002, [(H, 0), (L, 0)], "ZP"),
            (0x00_0004, [(BC, 2), (DE, 4)], "ZP"),
            (0x00_0004, [(BC, 8), (DE, 4)], "P"),
            (0x00_0006, [(A1, 1), (A0, 0xFF)], "P"),
            (0x00_0006, [(A1, 0xFF), (A0, 0xFF)], "N")
        ]
    );

    instr_test!(
        REGS: [
            (H, 0x8001),
            (L, 0x0002),
            (BC, 0xFFFF_FFFE),
            (DE, 0xFFFF_FFFE),
            (A1, 0x70),
            (A0, 0x02)
        ],
        MEM: gen_mem![
            MuliRaRb(H, L),
            Raf,
            MuliBraBrb(BC, DE),
            Raf,
            MuliVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 0x8001), (L, 0x0002)], ""),
            (0x00_0002, [(H, 0x0002), (L, 0x0002)], "OP"),
            (0x00_0004, [], "OP"),
            (0x00_0004, [], ""),
            (0x00_0006, [(BC, 0xFFFF_FFFE_u32), (DE, 0xFFFF_FFFE_u32)], ""),
            (0x00_0006, [(BC, 4), (DE, 0xFFFF_FFFE_u32)], "P"),
            (0x00_0008, [], "P"),
            (0x00_0008, [], ""),
            (0x00_000A, [(A1, 112), (A0, 2)], ""),
            (0x00_000A, [(A1, 224), (A0, 2)], "OPN")
        ]
    );

    let mut m = gen_mem![
        MuliRaBrb(A, BC),
        MuliRaImm16(D),
        2_u16,
        MuliBraImm32(HL),
        3_u32,
        MuliVraImm8(E1),
        0_u8
    ];
    m.write_word(0x0012_3456, 4);
    instr_test!(
        REGS: [
            (A, 0xFFFE),
            (BC, 0x0012_3456),
            (D, 3),
            (HL, 9),
            (E1, 0x12)
        ],
        MEM: m,
        FLAGS: "",
        [
            (0x00_0002, [(A, 0xFFFE), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 0xFFFE), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 0xFFF8), (BC, 0x12_3456)], "NP"),
            (0x00_0004, [(D, 3)], "NP"),
            (0x00_0006, [(D, 3)], "NP"),
            (0x00_0006, [(D, 6)], "P"),
            (0x00_0008, [(HL, 9)], "P"),
            (0x00_000A, [(HL, 9)], "P"),
            (0x00_000C, [(HL, 9)], "P"),
            (0x00_000C, [(HL, 27)], ""),
            (0x00_000E, [(E1, 0x12)], ""),
            (0x00_000F, [(E1, 0x12)], ""),
            (0x00_000F, [(E1, 0x0)], "ZP")
        ]
    );
}

#[test]
fn test_divu() {
    instr_test!(
        REGS: [
            (H, 7),
            (L, 2),
            (BC, 0),
            (DE, 0xFFFF_FFFF),
            (A1, 0x80),
            (A0, 0xFF)
        ],
        MEM: gen_mem![
            DivuRaRb(H, L),
            DivuBraBrb(BC, DE),
            DivuVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 7), (L, 2)], ""),
            (0x00_0002, [(H, 3), (L, 1)], ""),
            (0x00_0004, [(BC, 0), (DE, 0xFFFF_FFFF_u32)], ""),
            (0x00_0004, [(BC, 0), (DE, 0)], "ZP"),
            (0x00_0006, [(A1, 0x80), (A0, 0xFF)], "ZP"),
            (0x00_0006, [(A1, 0), (A0, 0x80)], "ZP")
        ]
    );

    let mut m = gen_mem![
        DiviRaBrb(A, BC),
        DiviRaImm16(D),
        2_u16,
        DiviBraImm32(HL),
        3_u32,
        DiviVraImm8(E1),
        0_u8
    ];
    m.write_word(0x0012_3456, 4);
    instr_test!(
        REGS: [
            (A, 0xFFFB),
            (BC, 0x0012_3456),
            (D, 3),
            (HL, 9),
            (E1, 0x12)
        ],
        MEM: m,
        FLAGS: "",
        [
            (0x00_0002, [(A, 0xFFFB), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 0xFFFB), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 0xFFFF), (BC, 0x12_3456)], "N"),
            (0x00_0002, [(A, 0xFFFF), (BC, 0x12_3456)], "N"),
            (0x00_0004, [(D, 3), (A, 0xFFFF)], "N"),
            (0x00_0006, [(D, 3), (A, 0xFFFF)], "N"),
            (0x00_0006, [(D, 1), (A, 1)], ""),
            (0x00_0008, [(HL, 9), (BC, 0x12_3456)], ""),
            (0x00_000A, [(HL, 9), (BC, 0x12_3456)], ""),
            (0x00_000C, [(HL, 9), (BC, 0x12_3456)], ""),
            (0x00_000C, [(HL, 3), (BC, 0)], ""),
            (0x00_000E, [(E1, 0x12), (A1, 0)], ""),
            (0x00_000F, [(E1, 0x12), (A1, 0)], ""),
            (0x00_000F, [(E1, 0x12), (A1, 0)], "")
        ]
    );
}

#[test]
fn test_divi() {
    instr_test!(
        REGS: [
            (H, 7),
            (L, 2),
            (BC, 0),
            (DE, 0xFFFF_FFFF),
            (A1, 0x80),
            (A0, 0xFF)
        ],
        MEM: gen_mem![
            DiviRaRb(H, L),
            DiviBraBrb(BC, DE),
            DiviVraVrb(A1, A0)
        ],
        FLAGS: "",
        [
            (0x00_0002, [(H, 7), (L, 2)], ""),
            (0x00_0002, [(H, 3), (L, 1)], ""),
            (0x00_0004, [(BC, 0), (DE, 0xFFFF_FFFF_u32)], ""),
            (0x00_0004, [(BC, 0), (DE, 0)], "ZP"),
            (0x00_0006, [(A1, 0x80), (A0, 0xFF)], "ZP"),
            (0x00_0006, [(A1, 0x80), (A0, 0xFF)], "OPN")
        ]
    );

    let mut m = gen_mem![
        DiviRaBrb(A, BC),
        DiviRaImm16(D),
        0xFFFE_u16,
        DiviBraImm32(HL),
        3_u32,
        DiviVraImm8(E1),
        0_u8
    ];
    m.write_word(0x0012_3456, 4);
    instr_test!(
        REGS: [
            (A, 0xFFFB),
            (BC, 0x0012_3456),
            (D, 3),
            (HL, 9),
            (E1, 0x12)
        ],
        MEM: m,
        FLAGS: "",
        [
            (0x00_0002, [(A, 0xFFFB), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 0xFFFB), (BC, 0x12_3456)], ""),
            (0x00_0002, [(A, 0xFFFF), (BC, 0x12_3456)], "N"),
            (0x00_0002, [(A, 0xFFFF), (BC, 0x12_3456)], "N"),
            (0x00_0004, [(D, 3), (A, 0xFFFF)], "N"),
            (0x00_0006, [(D, 3), (A, 0xFFFF)], "N"),
            (0x00_0006, [(D, 0xFFFF), (A, 1)], "N"),
            (0x00_0008, [(HL, 9), (BC, 0x12_3456)], "N"),
            (0x00_000A, [(HL, 9), (BC, 0x12_3456)], "N"),
            (0x00_000C, [(HL, 9), (BC, 0x12_3456)], "N"),
            (0x00_000C, [(HL, 3), (BC, 0)], ""),
            (0x00_000E, [(E1, 0x12), (A1, 0)], ""),
            (0x00_000F, [(E1, 0x12), (A1, 0)], ""),
            (0x00_000F, [(E1, 0x12), (A1, 0)], "")
        ]
    );
}
