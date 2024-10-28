// TODO
// add A B; -> fail
// add A, B -> fail
// add A, B, C; -> fail

// add A,[HL]; -> pass
// add A,[HL; -> fail
// add A,HL]; -> fail
// add [HL],A; -> fail

use super::{
    instr_to_bytes,
    Operand::{ProgramCounter as PC, StackPointer as SP, *},
    Operation::*,
};
use mfs16core::{Reg16::*, Reg32::*, Reg8::*};
use pretty_assertions::assert_eq;

macro_rules! to_bytes_test {
    ($test_name:ident, $operation:path, $operand_1:expr, $operand_2:expr, $expect:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!(
                instr_to_bytes(&$operation, &$operand_1, &$operand_2).unwrap(),
                $expect
            );
        }
    };
    ($test_name:ident, $operation:path, $operand_1:path, $operand_2:path, $expect:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!(
                instr_to_bytes(&$operation, &$operand_1, &$operand_2).unwrap(),
                $expect
            );
        }
    };
}

to_bytes_test!(nop, Nop, None, None, vec![0x00, 0x00]);
to_bytes_test!(ldrarb, Ld, Reg(A), Reg(B), vec![0x01, 0x01]);
to_bytes_test!(ldbrabrb, Ld, Breg(HL), Breg(DE), vec![0x98, 0x01]);
to_bytes_test!(
    ldspimm32,
    Ld,
    SP,
    DWord(0x0123_4567),
    vec![0xA0, 0x01, 0x67, 0x45, 0x23, 0x01]
);
to_bytes_test!(
    ldimm32sp,
    Ld,
    DWord(0x0123_4567),
    SP,
    vec![0xA1, 0x01, 0x67, 0x45, 0x23, 0x01]
);
to_bytes_test!(ldspbra, Ld, SP, Breg(HL), vec![0xB2, 0x01]);
to_bytes_test!(ldvravrb, Ld, Vreg(A0), Vreg(A1), vec![0x10, 0x02]);
to_bytes_test!(
    ldraimm16,
    Ld,
    Reg(A),
    Word(0x0123),
    vec![0x00, 0x03, 0x23, 0x01]
);
to_bytes_test!(
    ldbraimm32,
    Ld,
    Breg(BC),
    DWord(0x0123_4567),
    vec![0x10, 0x03, 0x67, 0x45, 0x23, 0x01]
);
to_bytes_test!(ldvraimm8, Ld, Vreg(A0), Byte(0x01), vec![0x21, 0x03, 0x01]);
to_bytes_test!(
    ldbraimm16,
    Ld,
    BregDeref(HL),
    Word(0xFEDC),
    vec![0x32, 0x03, 0xDC, 0xFE]
);
to_bytes_test!(ldbrarb, Ld, BregDeref(BC), Reg(L), vec![0x06, 0x04]);
to_bytes_test!(ldrabrb, Ld, Reg(H), BregDeref(DE), vec![0x51, 0x05]);
to_bytes_test!(ldibrarb, Ldi, BregDeref(BC), Reg(A), vec![0x00, 0x06]);
to_bytes_test!(lddbrarb, Ldd, BregDeref(BC), Reg(A), vec![0x00, 0x07]);
to_bytes_test!(ldirabrb, Ldi, Reg(A), BregDeref(BC), vec![0x00, 0x08]);
to_bytes_test!(lddrabrb, Ldd, Reg(A), BregDeref(BC), vec![0x00, 0x09]);
to_bytes_test!(addrarb, Add, Reg(A), Reg(B), vec![0x01, 0x10]);
to_bytes_test!(addbrabrb, Add, Breg(BC), Breg(DE), vec![0x78, 0x10]);
to_bytes_test!(addvravrb, Add, Vreg(A0), Vreg(A1), vec![0x10, 0x11]);
to_bytes_test!(adcrarb, Adc, Reg(A), Reg(B), vec![0x01, 0x12]);
to_bytes_test!(adcbrabrb, Adc, Breg(BC), Breg(DE), vec![0x78, 0x12]);
to_bytes_test!(adcvravrb, Adc, Vreg(A0), Vreg(A1), vec![0x10, 0x13]);
to_bytes_test!(subrarb, Sub, Reg(A), Reg(B), vec![0x01, 0x14]);
to_bytes_test!(subbrabrb, Sub, Breg(BC), Breg(DE), vec![0x78, 0x14]);
to_bytes_test!(subvravrb, Sub, Vreg(A0), Vreg(A1), vec![0x10, 0x15]);
to_bytes_test!(sbbrarb, Sbb, Reg(A), Reg(B), vec![0x01, 0x16]);
to_bytes_test!(sbbbrabrb, Sbb, Breg(BC), Breg(DE), vec![0x78, 0x16]);
to_bytes_test!(sbbvravrb, Sbb, Vreg(A0), Vreg(A1), vec![0x10, 0x17]);
to_bytes_test!(
    addraimm16,
    Add,
    Reg(A),
    Word(0xFEDC),
    vec![0x00, 0x18, 0xDC, 0xFE]
);
to_bytes_test!(
    adcraimm16,
    Adc,
    Reg(A),
    Word(0xFEDC),
    vec![0x10, 0x18, 0xDC, 0xFE]
);
to_bytes_test!(
    addbraimm32,
    Add,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x20, 0x18, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    adcbraimm32,
    Adc,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x30, 0x18, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    addvraimm8,
    Add,
    Vreg(A1),
    Byte(0xFE),
    vec![0x40, 0x18, 0xFE]
);
to_bytes_test!(
    adcvraimm8,
    Adc,
    Vreg(A1),
    Byte(0xFE),
    vec![0x50, 0x18, 0xFE]
);
to_bytes_test!(
    subraimm16,
    Sub,
    Reg(A),
    Word(0xFEDC),
    vec![0x60, 0x18, 0xDC, 0xFE]
);
to_bytes_test!(
    sbbraimm16,
    Sbb,
    Reg(A),
    Word(0xFEDC),
    vec![0x70, 0x18, 0xDC, 0xFE]
);
to_bytes_test!(
    subbraimm32,
    Sub,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x80, 0x18, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    sbbbraimm32,
    Sbb,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x90, 0x18, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    subvraimm8,
    Sub,
    Vreg(A1),
    Byte(0xFE),
    vec![0xA0, 0x18, 0xFE]
);
to_bytes_test!(
    sbbvraimm8,
    Sbb,
    Vreg(A1),
    Byte(0xFE),
    vec![0xB0, 0x18, 0xFE]
);
to_bytes_test!(addrabrb, Add, Reg(A), BregDeref(BC), vec![0x00, 0x19]);
to_bytes_test!(adcrabrb, Adc, Reg(A), BregDeref(BC), vec![0x00, 0x1A]);
to_bytes_test!(subrabrb, Sub, Reg(A), BregDeref(BC), vec![0x00, 0x1B]);
to_bytes_test!(sbbrabrb, Sbb, Reg(A), BregDeref(BC), vec![0x00, 0x1C]);
to_bytes_test!(tcpra, Tcp, Reg(A), None, vec![0x00, 0x1D]);
to_bytes_test!(tcpbra, Tcp, Breg(BC), None, vec![0x10, 0x1D]);
to_bytes_test!(tcpvra, Tcp, Vreg(A1), None, vec![0x20, 0x1D]);
to_bytes_test!(incra, Inc, Reg(A), None, vec![0x30, 0x1D]);
to_bytes_test!(incbra, Inc, Breg(BC), None, vec![0x40, 0x1D]);
to_bytes_test!(incvra, Inc, Vreg(A1), None, vec![0x50, 0x1D]);
to_bytes_test!(decra, Dec, Reg(A), None, vec![0x60, 0x1D]);
to_bytes_test!(decbra, Dec, Breg(BC), None, vec![0x70, 0x1D]);
to_bytes_test!(decvra, Dec, Vreg(A1), None, vec![0x80, 0x1D]);
to_bytes_test!(pssra, Pss, Reg(A), None, vec![0x90, 0x1D]);
to_bytes_test!(pssbra, Pss, Breg(HL), None, vec![0xA2, 0x1D]);
to_bytes_test!(pssvra, Pss, Vreg(L0), None, vec![0xBD, 0x1D]);
to_bytes_test!(
    pssimm16,
    Pss,
    Word(0x0123),
    None,
    vec![0xC0, 0x1D, 0x23, 0x01]
);
to_bytes_test!(
    pssimm32,
    Pss,
    DWord(0x0123_4567),
    None,
    vec![0xC1, 0x1D, 0x67, 0x45, 0x23, 0x01]
);
to_bytes_test!(pssimm8, Pss, Byte(0xFE), None, vec![0xC2, 0x1D, 0xFE]);
to_bytes_test!(andrarb, And, Reg(L), Reg(H), vec![0x65, 0x1E]);
to_bytes_test!(andbrabrb, And, Breg(HL), Breg(DE), vec![0x21, 0x1F]);
to_bytes_test!(andvravrb, And, Vreg(L0), Vreg(L1), vec![0xDC, 0x20]);
to_bytes_test!(andrabrb, And, Reg(A), BregDeref(HL), vec![0x02, 0x21]);
to_bytes_test!(orrarb, Or, Reg(L), Reg(H), vec![0x65, 0x22]);
to_bytes_test!(orbrabrb, Or, Breg(HL), Breg(DE), vec![0x21, 0x23]);
to_bytes_test!(orvravrb, Or, Vreg(L0), Vreg(L1), vec![0xDC, 0x24]);
to_bytes_test!(orrabrb, Or, Reg(A), BregDeref(HL), vec![0x02, 0x25]);
to_bytes_test!(xorrarb, Xor, Reg(L), Reg(H), vec![0x65, 0x26]);
to_bytes_test!(xorbrabrb, Xor, Breg(HL), Breg(DE), vec![0x21, 0x27]);
to_bytes_test!(xorvravrb, Xor, Vreg(L0), Vreg(L1), vec![0xDC, 0x28]);
to_bytes_test!(xorrabrb, Xor, Reg(A), BregDeref(HL), vec![0x02, 0x29]);
to_bytes_test!(
    andraimm16,
    And,
    Reg(A),
    Word(0xFEDC),
    vec![0x00, 0x2A, 0xDC, 0xFE]
);
to_bytes_test!(
    andbraimm32,
    And,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x10, 0x2A, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    andvraimm8,
    And,
    Vreg(L0),
    Byte(0x98),
    vec![0x2D, 0x2A, 0x98]
);
to_bytes_test!(
    orraimm16,
    Or,
    Reg(A),
    Word(0xFEDC),
    vec![0x30, 0x2A, 0xDC, 0xFE]
);
to_bytes_test!(
    orbraimm32,
    Or,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x40, 0x2A, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(orvraimm8, Or, Vreg(L0), Byte(0x98), vec![0x5D, 0x2A, 0x98]);
to_bytes_test!(
    xorraimm16,
    Xor,
    Reg(A),
    Word(0xFEDC),
    vec![0x60, 0x2A, 0xDC, 0xFE]
);
to_bytes_test!(
    xorbraimm32,
    Xor,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x70, 0x2A, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    xorvraimm8,
    Xor,
    Vreg(L0),
    Byte(0x98),
    vec![0x8D, 0x2A, 0x98]
);
to_bytes_test!(notra, Not, Reg(H), None, vec![0x95, 0x2A]);
to_bytes_test!(notbra, Not, Breg(HL), None, vec![0xA2, 0x2A]);
to_bytes_test!(notvra, Not, Vreg(L0), None, vec![0xBD, 0x2A]);
to_bytes_test!(asrrab, Asr, Reg(B), Byte(0x0F), vec![0x1F, 0x2B]);
to_bytes_test!(asrbrab, Asr, Breg(HL), Byte(0x0E), vec![0x2E, 0x2C]);
to_bytes_test!(asrvrab, Asr, Vreg(D1), Byte(0x07), vec![0x67, 0x2D]);
to_bytes_test!(aslrab, Asl, Reg(B), Byte(0x0F), vec![0x1F, 0x2E]);
to_bytes_test!(aslbrab, Asl, Breg(HL), Byte(0x0E), vec![0x2E, 0x2F]);
to_bytes_test!(aslvrab, Asl, Vreg(D1), Byte(0x07), vec![0x67, 0x30]);
to_bytes_test!(lsrrab, Lsr, Reg(B), Byte(0x0F), vec![0x1F, 0x31]);
to_bytes_test!(lsrbrab, Lsr, Breg(HL), Byte(0x0E), vec![0x2E, 0x32]);
to_bytes_test!(lsrvrab, Lsr, Vreg(D1), Byte(0x07), vec![0x67, 0x33]);
to_bytes_test!(rtrrab, Rtr, Reg(B), Byte(0x0F), vec![0x1F, 0x34]);
to_bytes_test!(rtrbrab, Rtr, Breg(HL), Byte(0x0E), vec![0x2E, 0x35]);
to_bytes_test!(rtrvrab, Rtr, Vreg(D1), Byte(0x07), vec![0x67, 0x36]);
to_bytes_test!(rtlrab, Rtl, Reg(B), Byte(0x0F), vec![0x1F, 0x37]);
to_bytes_test!(rtlbrab, Rtl, Breg(HL), Byte(0x0E), vec![0x2E, 0x38]);
to_bytes_test!(rtlvrab, Rtl, Vreg(D1), Byte(0x07), vec![0x67, 0x39]);
to_bytes_test!(rcrrab, Rcr, Reg(B), Byte(0x0F), vec![0x1F, 0x3A]);
to_bytes_test!(rcrbrab, Rcr, Breg(HL), Byte(0x0E), vec![0x2E, 0x3B]);
to_bytes_test!(rcrvrab, Rcr, Vreg(D1), Byte(0x07), vec![0x67, 0x3C]);
to_bytes_test!(rclrab, Rcl, Reg(B), Byte(0x0F), vec![0x1F, 0x3D]);
to_bytes_test!(rclbrab, Rcl, Breg(HL), Byte(0x0E), vec![0x2E, 0x3E]);
to_bytes_test!(rclvrab, Rcl, Vreg(D1), Byte(0x07), vec![0x67, 0x3F]);
