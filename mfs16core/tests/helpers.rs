use mfs16core::{gen_mem, Addr, Computer, Flags, MemWritable, Memory, Reg, Reg8::*};
use pretty_assertions::assert_eq;

/// Create a [Computer] without a ROM write restriction.
pub fn test_computer() -> Computer {
    let mut c = Computer::default();
    c.mmu.rom.set_writable(true);
    c
}

macro_rules! instr_test {
    (
        REGS: [$(($start_reg:ident, $start_val:literal)),*],
        MEM: $mem:expr,
        FLAGS: $start_flags:literal,
        // Test values at each cycle
        [
            $(($pc:literal, [$(($reg:ident, $val:literal)),*], $flags:literal)),+
        ]
    ) => {{
        let mut _cycle_num: u32 = 0;
        let mut c = Computer::default();
        c.mmu.rom = $mem;
        $(
            $start_reg.set(&mut c.cpu, $start_val);
        )*
        c.cpu.flags = Flags::from_string($start_flags);

        $(
            c.cycle();
            _cycle_num += 1;
            assert_eq!(
                c.cpu.pc,
                Addr::new_default_range($pc),
                "[{}] PC FAIL: {}, expected {:#08X}",
                _cycle_num,
                c.cpu.pc,
                $pc
            );
            $(
                assert_eq!(
                    $reg.get(&c.cpu),
                    $val, "[{}] REG FAIL: {}={:#X}, expected {:#X}",
                    _cycle_num,
                    $reg,
                    $reg.get(&c.cpu),
                    $val
                );
            )*
            assert_eq!(
                c.cpu.flags,
                Flags::from_string($flags),
                "[{}] FLAG FAIL: {}, expected {}",
                _cycle_num,
                c.cpu.flags,
                Flags::from_string($flags)
            );
        )+
    }};
}
pub(crate) use instr_test;

#[test]
fn helper_test() {
    // ADD A1,A0
    // 0xF2 + 0x05 = 0xF7, zcopN
    instr_test!(
        REGS: [(A1, 0xF2), (A0, 0x05)],
        MEM: gen_mem![0x1101_u16],
        FLAGS: "",
        [
            (0x00_0002, [(A1, 0xF2), (A0, 0x05)], ""),
            (0x00_0002, [(A1, 0xF7), (A0, 0x05)], "N")
        ]
    );
}
