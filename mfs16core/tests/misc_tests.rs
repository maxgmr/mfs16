#![cfg(test)]

use mfs16core::{
    gen_mem, Addr, Computer, Flags, Instruction::*, MemWritable, Memory, Reg, Reg16::*, Reg32::*,
    Reg8::*,
};
use pretty_assertions::assert_eq;

mod helpers;

use helpers::{instr_test, test_computer};

#[test]
fn test_ei_di() {
    let mut c = test_computer();
    c.cpu.interrupts_enabled = false;

    c.mmu.write_word(0x00_0000, 0xFFFD);
    c.mmu.write_word(0x00_0002, 0xFFFE);

    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));
    assert!(!c.cpu.interrupts_enabled);

    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0002));
    assert!(c.cpu.interrupts_enabled);

    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));
    assert!(c.cpu.interrupts_enabled);

    c.cycle();
    assert_eq!(c.cpu.pc, Addr::new_default_range(0x00_0004));
    assert!(!c.cpu.interrupts_enabled);
}

#[test]
fn test_rand() {
    instr_test!(
        REGS: [(A, 0), (BC, 0), (D1, 0)],
        MEM: gen_mem![
            Nop,
            Nop,
            RandRa(A),
            RandBra(BC),
            RandVra(D1)
        ],
        FLAGS: "",
        [
            (0x00_0002, [], ""),
            (0x00_0002, [], ""),
            (0x00_0004, [], ""),
            (0x00_0004, [], ""),
            (0x00_0006, [(A, 0)], ""),
            (0x00_0006, [(A, 0x0135)], ""),
            (0x00_0008, [(BC, 0)], ""),
            (0x00_0008, [(BC, 0x3800_0613)], ""),
            (0x00_000A, [(D1, 0)], ""),
            (0x00_000A, [(D1, 0xA9)], "")
        ]
    );
}

#[test]
fn test_set_reset_toggle_flags() {
    instr_test!(
        REGS: [],
        MEM: gen_mem![
            Szf.into_opcode(),
            Rzf.into_opcode(),
            Tzf.into_opcode(),
            Scf.into_opcode(),
            Rcf.into_opcode(),
            Tcf.into_opcode(),
            Sof.into_opcode(),
            Rof.into_opcode(),
            Tof.into_opcode(),
            Spf.into_opcode(),
            Rpf.into_opcode(),
            Tpf.into_opcode(),
            Snf.into_opcode(),
            Rnf.into_opcode(),
            Tnf.into_opcode(),
            Saf.into_opcode(),
            Raf.into_opcode()
        ],
        FLAGS: "CP",
        [
            // SZF
            (0x00_0002, [], "CP"),
            (0x00_0002, [], "ZCP"),
            // RZF
            (0x00_0004, [], "ZCP"),
            (0x00_0004, [], "CP"),
            // TZF
            (0x00_0006, [], "CP"),
            (0x00_0006, [], "ZCP"),
            // SCF
            (0x00_0008, [], "ZCP"),
            (0x00_0008, [], "ZCP"),
            // RCF
            (0x00_000A, [], "ZCP"),
            (0x00_000A, [], "ZP"),
            // TCF
            (0x00_000C, [], "ZP"),
            (0x00_000C, [], "ZCP"),
            // SOF
            (0x00_000E, [], "ZCP"),
            (0x00_000E, [], "ZCOP"),
            // ROF
            (0x00_0010, [], "ZCOP"),
            (0x00_0010, [], "ZCP"),
            // TOF
            (0x00_0012, [], "ZCP"),
            (0x00_0012, [], "ZCOP"),
            // SPF
            (0x00_0014, [], "ZCOP"),
            (0x00_0014, [], "ZCOP"),
            // RPF
            (0x00_0016, [], "ZCOP"),
            (0x00_0016, [], "ZCO"),
            // TPF
            (0x00_0018, [], "ZCO"),
            (0x00_0018, [], "ZCOP"),
            // SNF
            (0x00_001A, [], "ZCOP"),
            (0x00_001A, [], "ZCOPN"),
            // RNF
            (0x00_001C, [], "ZCOPN"),
            (0x00_001C, [], "ZCOP"),
            // TNF
            (0x00_001E, [], "ZCOP"),
            (0x00_001E, [], "ZCOPN"),
            // SAF
            (0x00_0020, [], "ZCOPN"),
            (0x00_0020, [], "ZCOPN"),
            // RAF
            (0x00_0022, [], "ZCOPN"),
            (0x00_0022, [], "")
        ]
    );

    instr_test!(
        REGS: [],
        MEM: gen_mem![
           Halt.into_opcode(),
           Szf.into_opcode()
        ],
        FLAGS: "",
        [
            // HALT
            (0x00_0002, [], ""),
            (0x00_0002, [], ""),
            // Flags should not be set, PC should not be advanced
            (0x00_0002, [], ""),
            (0x00_0002, [], ""),
            (0x00_0002, [], ""),
            (0x00_0002, [], "")
        ]
    );
}
