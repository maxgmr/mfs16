use std::time::Instant;

use mfs16core::{Reg16::*, Reg32::*, Reg8::*};
use pretty_assertions::assert_eq;

use super::{
    instr_to_bytes,
    Operand::{StackPointer as SP, *},
    Operation::*,
    Parser, Variable,
};
use crate::{lex, parse};

// ------- PARSING TESTS -------
macro_rules! parser_test {
    (FULL: $test_name:ident, $data:literal => $expected:expr) => {
        #[test]
        fn $test_name() {
            let tokens = lex($data).unwrap();
            let machine_code = parse(tokens, $data, 0, true).unwrap();
            assert_eq!(machine_code, $expected);
        }
    };
    (FULL FAIL: $test_name:ident, $data:literal) => {
        #[test]
        fn $test_name() {
            let tokens = lex($data).unwrap();
            let _ = parse(tokens, $data, 0, true).unwrap_err();
        }
    };
    ($test_name:ident, $data:expr => $expected:expr) => {
        #[test]
        fn $test_name() {
            let start = Instant::now();

            let tokens = lex($data).unwrap();
            let mut parser = Parser::new(tokens, $data, 0, false, true);
            let result = parser.parse_instr();

            println!("[{}] - {:.2?} elapsed", $data, start.elapsed());

            assert_eq!(result.unwrap(), $expected);
        }
    };
    ($test_name:ident, $data:expr => $var_name:expr, $var_val:expr) => {
        #[test]
        fn $test_name() {
            let start = Instant::now();

            let tokens = lex($data).unwrap();
            let mut parser = Parser::new(tokens, $data, 0, false, true);
            parser.parse_assignment().unwrap();

            println!("[{}] - {:.2?} elapsed", $data, start.elapsed());

            assert_eq!(parser.variables.get($var_name).unwrap(), $var_val);
        }
    };
    (FAIL: $test_name:ident, $method:ident, $data:expr) => {
        #[test]
        fn $test_name() {
            let start = Instant::now();

            let tokens = lex($data).unwrap();
            let mut parser = Parser::new(tokens, $data, 0, false, true);
            let result = parser.$method();

            println!("[{}] - {:.2?} elapsed", $data, start.elapsed());

            assert!(result.is_err(), "Expected error, got {:?}.", result);
        }
    };
}

parser_test!(
    FULL: label,
    "// looping label test.\nld A1,0x00:b;\n\nloop:\n\tinc A1;\n\tjnz loop;\n\nhalt;"
    =>
    vec![0x20, 0x03, 0x00, 0x50, 0x1D, 0x03, 0x80, 0x03, 0x00, 0x00, 0x00, 0xFF, 0xFF]
);
parser_test!(
    FULL: assignthenadd,
    "// 1 + 2\nb1 = 0x01:b;\nb2 = 0x02:b;\nld A1,b1;\nld B1,b2;\nadd A1,B1;\n// done!"
    =>
    vec![0x20, 0x03, 0x01, 0x22, 0x03, 0x02, 0x02, 0x11]
);
parser_test!(
    FULL: dblassign,
    "my_num=0:w;my_num_2=2:w;PSS my_num;my_num=my_num_2;PSS my_num;"
    =>
    vec![0xC0, 0x1D, 0x00, 0x00, 0xC0, 0x1D, 0x02, 0x00]
);
parser_test!(
    FULL: assignlabelafter,
    "loop:\npss L0;\njpz is_zero;\n// is not zero\ndec L0;\njp loop;\n\nis_zero:\nhalt;"
    =>
    vec![0xBD, 0x1D, 0x02, 0x80, 0x10, 0x00, 0x00, 0x00, 0x8D, 0x1D, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF]
);
parser_test!(
    FULL FAIL: unassignedlabel,
    "jnz loop;"
);

parser_test!(varbyteassign, "my_byte = 0xFE:b;" => "my_byte", &Variable::Byte(0xFE));
parser_test!(varwordassign, "my_word = 0xFE:w;" => "my_word", &Variable::Word(0x00FE));
parser_test!(vardwordassign, "my_dword = 0o0123_4567:d;" => "my_dword", &Variable::DWord(0o0123_4567));
parser_test!(varqwordassign, "my_qword = 0b0:q;" => "my_qword", &Variable::QWord(0b0));

parser_test!(parseadd, "add A,B;" => Some(vec![0x01_u8, 0x10_u8]));
parser_test!(parseld, "ld HL,SP;" => Some(vec![0xC2_u8, 0x01_u8]));
parser_test!(FAIL: parsenocomma, parse_instr, "add A B;");
parser_test!(FAIL: parsenosemi, parse_instr, "add A, B");
parser_test!(FAIL: parse3arg, parse_instr, "add A, B, C;");
parser_test!(FAIL: parseargsmismatch, parse_instr, "add HL,B;");
parser_test!(FAIL: parsenotreg, parse_instr, "add, a,b;");

parser_test!(parsenop, "NOP;" => Some(vec![0x00_u8, 0x00_u8]));
parser_test!(FAIL: parsenosemi2, parse_instr, "nop");

parser_test!(parsederef, "ADD A,\t\t[HL];" => Some(vec![0x02_u8, 0x19_u8]));
parser_test!(parsederefld, "ld [DE],B;" => Some(vec![0x11_u8, 0x04_u8]));
parser_test!(parseimmderef, "LD [0x0012_3456:d],SP;" => Some(vec![0xA1, 0x01, 0x56, 0x34, 0x12, 0x00]));
parser_test!(FAIL: parsenoclosebr, parse_instr, "add A,[HL;");
parser_test!(FAIL: parsenoopenbr, parse_instr, "add A,HL];");
parser_test!(FAIL: parsebadargsderef, parse_instr, "add [HL],A;");

parser_test!(parseimm, "ADC D,0xFE:w;" => Some(vec![0x13, 0x18, 0xFE, 0x00]));
parser_test!(FAIL: parseimmwrongtype, parse_instr, "ADC A,0xFE:b;");

parser_test!(parse1arg, "INC A;" => Some(vec![0x30, 0x1D]));

parser_test!(parsenotinstr, "my_var = 1234:w;" => Option::None);
parser_test!(parsenotinstr2, "notinc A;" => Option::None);

// ------- TO BYTES TESTS -------
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
    DWordDeref(0x0123_4567),
    SP,
    vec![0xA1, 0x01, 0x67, 0x45, 0x23, 0x01]
);
to_bytes_test!(ldspbra, Ld, SP, Breg(HL), vec![0xB2, 0x01]);
to_bytes_test!(ldbrasp, Ld, Breg(HL), SP, vec![0xC2, 0x01]);
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
to_bytes_test!(
    ldrraimm32,
    Ldr,
    Reg(A),
    DWordDeref(0x0012_3456),
    vec![0x70, 0x05, 0x56, 0x34, 0x12, 0x00]
);
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
to_bytes_test!(cmprarb, Cmp, Reg(A), Reg(A), vec![0x00, 0x40]);
to_bytes_test!(cmpbrabrb, Cmp, Breg(BC), Breg(DE), vec![0x78, 0x40]);
to_bytes_test!(cmpvravrb, Cmp, Vreg(H1), Vreg(H0), vec![0xAB, 0x41]);
to_bytes_test!(
    cmpraimm16,
    Cmp,
    Reg(A),
    Word(0xFEDC),
    vec![0x00, 0x42, 0xDC, 0xFE]
);
to_bytes_test!(
    cmpbraimm32,
    Cmp,
    Breg(BC),
    DWord(0xFEDC_BA98),
    vec![0x10, 0x42, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    cmpvraimm8,
    Cmp,
    Vreg(H0),
    Byte(0xFE),
    vec![0x2B, 0x42, 0xFE]
);
to_bytes_test!(
    cmpimm16ra,
    Cmp,
    Word(0xFEDC),
    Reg(A),
    vec![0x30, 0x42, 0xDC, 0xFE]
);
to_bytes_test!(
    cmpimm32bra,
    Cmp,
    DWord(0xFEDC_BA98),
    Breg(BC),
    vec![0x40, 0x42, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    cmpimm8vra,
    Cmp,
    Byte(0xFE),
    Vreg(H0),
    vec![0x5B, 0x42, 0xFE]
);
to_bytes_test!(cmprabrb, Cmp, Reg(A), BregDeref(BC), vec![0x00, 0x43]);
to_bytes_test!(cmpbrarb, Cmp, BregDeref(BC), Reg(A), vec![0x00, 0x44]);
to_bytes_test!(bitrab, Bit, Reg(A), Byte(0x00), vec![0x00, 0x45]);
to_bytes_test!(bitbrab, Bit, BregDeref(BC), Byte(0x00), vec![0x00, 0x46]);
to_bytes_test!(stbrab, Stb, Reg(A), Byte(0x00), vec![0x00, 0x47]);
to_bytes_test!(stbbrab, Stb, BregDeref(BC), Byte(0x00), vec![0x00, 0x48]);
to_bytes_test!(rsbrab, Rsb, Reg(A), Byte(0x00), vec![0x00, 0x49]);
to_bytes_test!(rsbbrab, Rsb, BregDeref(BC), Byte(0x00), vec![0x00, 0x4A]);
to_bytes_test!(tgbrab, Tgb, Reg(A), Byte(0x00), vec![0x00, 0x4B]);
to_bytes_test!(tgbbrab, Tgb, BregDeref(BC), Byte(0x00), vec![0x00, 0x4C]);
to_bytes_test!(swpra, Swp, Reg(A), None, vec![0x00, 0x4D]);
to_bytes_test!(swpbra, Swp, BregDeref(BC), None, vec![0x10, 0x4D]);
to_bytes_test!(szf, Szf, None, None, vec![0x20, 0x4D]);
to_bytes_test!(rzf, Rzf, None, None, vec![0x21, 0x4D]);
to_bytes_test!(tzf, Tzf, None, None, vec![0x22, 0x4D]);
to_bytes_test!(scf, Scf, None, None, vec![0x23, 0x4D]);
to_bytes_test!(rcf, Rcf, None, None, vec![0x24, 0x4D]);
to_bytes_test!(tcf, Tcf, None, None, vec![0x25, 0x4D]);
to_bytes_test!(sof, Sof, None, None, vec![0x26, 0x4D]);
to_bytes_test!(rof, Rof, None, None, vec![0x27, 0x4D]);
to_bytes_test!(tof, Tof, None, None, vec![0x28, 0x4D]);
to_bytes_test!(spf, Spf, None, None, vec![0x29, 0x4D]);
to_bytes_test!(rpf, Rpf, None, None, vec![0x2A, 0x4D]);
to_bytes_test!(tpf, Tpf, None, None, vec![0x2B, 0x4D]);
to_bytes_test!(snf, Snf, None, None, vec![0x2C, 0x4D]);
to_bytes_test!(rnf, Rnf, None, None, vec![0x2D, 0x4D]);
to_bytes_test!(tnf, Tnf, None, None, vec![0x2E, 0x4D]);
to_bytes_test!(saf, Saf, None, None, vec![0x2F, 0x4D]);
to_bytes_test!(raf, Raf, None, None, vec![0x30, 0x4D]);
to_bytes_test!(
    jpimm32,
    Jp,
    DWord(0x12_3456),
    None,
    vec![0x00, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jrimm32,
    Jr,
    DWord(0xFEDC_BA98),
    None,
    vec![0x01, 0x80, 0x98, 0xBA, 0xDC, 0xFE]
);
to_bytes_test!(
    jpzimm32,
    Jpz,
    DWord(0x12_3456),
    None,
    vec![0x02, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jnzimm32,
    Jnz,
    DWord(0x12_3456),
    None,
    vec![0x03, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jpcimm32,
    Jpc,
    DWord(0x12_3456),
    None,
    vec![0x04, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jncimm32,
    Jnc,
    DWord(0x12_3456),
    None,
    vec![0x05, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jpoimm32,
    Jpo,
    DWord(0x12_3456),
    None,
    vec![0x06, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jnoimm32,
    Jno,
    DWord(0x12_3456),
    None,
    vec![0x07, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jppimm32,
    Jpp,
    DWord(0x12_3456),
    None,
    vec![0x08, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jnpimm32,
    Jnp,
    DWord(0x12_3456),
    None,
    vec![0x09, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jpnimm32,
    Jpn,
    DWord(0x12_3456),
    None,
    vec![0x0A, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    jnnimm32,
    Jnn,
    DWord(0x12_3456),
    None,
    vec![0x0B, 0x80, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(jpbra, Jp, Breg(BC), None, vec![0x10, 0x80]);
to_bytes_test!(jrbra, Jr, Breg(BC), None, vec![0x20, 0x80]);
to_bytes_test!(jpzbra, Jpz, Breg(BC), None, vec![0x30, 0x80]);
to_bytes_test!(jnzbra, Jnz, Breg(BC), None, vec![0x40, 0x80]);
to_bytes_test!(jpcbra, Jpc, Breg(BC), None, vec![0x50, 0x80]);
to_bytes_test!(jncbra, Jnc, Breg(BC), None, vec![0x60, 0x80]);
to_bytes_test!(jpobra, Jpo, Breg(BC), None, vec![0x70, 0x80]);
to_bytes_test!(jnobra, Jno, Breg(BC), None, vec![0x80, 0x80]);
to_bytes_test!(jppbra, Jpp, Breg(BC), None, vec![0x90, 0x80]);
to_bytes_test!(jnpbra, Jnp, Breg(BC), None, vec![0xA0, 0x80]);
to_bytes_test!(jpnbra, Jpn, Breg(BC), None, vec![0xB0, 0x80]);
to_bytes_test!(jnnbra, Jnn, Breg(BC), None, vec![0xC0, 0x80]);
to_bytes_test!(halt, Halt, None, None, vec![0xFF, 0xFF]);
to_bytes_test!(
    callimm32,
    Call,
    DWord(0x0012_3456),
    None,
    vec![0x00, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    clzimm32,
    Clz,
    DWord(0x0012_3456),
    None,
    vec![0x01, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    cnzimm32,
    Cnz,
    DWord(0x0012_3456),
    None,
    vec![0x02, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    clcimm32,
    Clc,
    DWord(0x0012_3456),
    None,
    vec![0x03, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    cncimm32,
    Cnc,
    DWord(0x0012_3456),
    None,
    vec![0x04, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    cloimm32,
    Clo,
    DWord(0x0012_3456),
    None,
    vec![0x05, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    cnoimm32,
    Cno,
    DWord(0x0012_3456),
    None,
    vec![0x06, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    clpimm32,
    Clp,
    DWord(0x0012_3456),
    None,
    vec![0x07, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    cnpimm32,
    Cnp,
    DWord(0x0012_3456),
    None,
    vec![0x08, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    clnimm32,
    Cln,
    DWord(0x0012_3456),
    None,
    vec![0x09, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(
    cnnimm32,
    Cnn,
    DWord(0x0012_3456),
    None,
    vec![0x0A, 0x81, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(callbra, Call, Breg(BC), None, vec![0x10, 0x81]);
to_bytes_test!(ret, Ret, None, None, vec![0x13, 0x81]);
to_bytes_test!(rtz, Rtz, None, None, vec![0x14, 0x81]);
to_bytes_test!(rnz, Rnz, None, None, vec![0x15, 0x81]);
to_bytes_test!(rtc, Rtc, None, None, vec![0x16, 0x81]);
to_bytes_test!(rnc, Rnc, None, None, vec![0x17, 0x81]);
to_bytes_test!(rto, Rto, None, None, vec![0x18, 0x81]);
to_bytes_test!(rno, Rno, None, None, vec![0x19, 0x81]);
to_bytes_test!(rtp, Rtp, None, None, vec![0x1A, 0x81]);
to_bytes_test!(rnp, Rnp, None, None, vec![0x1B, 0x81]);
to_bytes_test!(rtn, Rtn, None, None, vec![0x1C, 0x81]);
to_bytes_test!(rnn, Rnn, None, None, vec![0x1D, 0x81]);
to_bytes_test!(clzbra, Clz, Breg(BC), None, vec![0x20, 0x81]);
to_bytes_test!(cnzbra, Cnz, Breg(BC), None, vec![0x30, 0x81]);
to_bytes_test!(clcbra, Clc, Breg(BC), None, vec![0x40, 0x81]);
to_bytes_test!(cncbra, Cnc, Breg(BC), None, vec![0x50, 0x81]);
to_bytes_test!(clobra, Clo, Breg(BC), None, vec![0x60, 0x81]);
to_bytes_test!(cnobra, Cno, Breg(BC), None, vec![0x70, 0x81]);
to_bytes_test!(clpbra, Clp, Breg(BC), None, vec![0x80, 0x81]);
to_bytes_test!(cnpbra, Cnp, Breg(BC), None, vec![0x90, 0x81]);
to_bytes_test!(clnbra, Cln, Breg(BC), None, vec![0xA0, 0x81]);
to_bytes_test!(cnnbra, Cnn, Breg(BC), None, vec![0xB0, 0x81]);
to_bytes_test!(pushbra, Push, Breg(BC), None, vec![0x00, 0x82]);
to_bytes_test!(popbra, Pop, Breg(BC), None, vec![0x03, 0x82]);
to_bytes_test!(peekbra, Peek, Breg(BC), None, vec![0x06, 0x82]);
to_bytes_test!(
    pushimm32,
    Push,
    DWord(0x12_3456),
    None,
    vec![0x09, 0x82, 0x56, 0x34, 0x12, 0x00]
);
to_bytes_test!(mulurarb, Mulu, Reg(A), Reg(B), vec![0x01, 0x50]);
to_bytes_test!(mulirarb, Muli, Reg(A), Reg(B), vec![0x01, 0x51]);
to_bytes_test!(divurarb, Divu, Reg(A), Reg(B), vec![0x01, 0x52]);
to_bytes_test!(divirarb, Divi, Reg(A), Reg(B), vec![0x01, 0x53]);
to_bytes_test!(mulubrabrb, Mulu, Breg(BC), Breg(DE), vec![0x78, 0x50]);
to_bytes_test!(mulibrabrb, Muli, Breg(BC), Breg(DE), vec![0x78, 0x51]);
to_bytes_test!(divubrabrb, Divu, Breg(BC), Breg(DE), vec![0x78, 0x52]);
to_bytes_test!(divibrabrb, Divi, Breg(BC), Breg(DE), vec![0x78, 0x53]);
to_bytes_test!(muluvravrb, Mulu, Vreg(A1), Vreg(A0), vec![0x01, 0x54]);
to_bytes_test!(mulivravrb, Muli, Vreg(A1), Vreg(A0), vec![0x01, 0x55]);
to_bytes_test!(divuvravrb, Divu, Vreg(A1), Vreg(A0), vec![0x01, 0x56]);
to_bytes_test!(divivravrb, Divi, Vreg(A1), Vreg(A0), vec![0x01, 0x57]);
to_bytes_test!(mulurabrb, Mulu, Reg(A), BregDeref(BC), vec![0x00, 0x58]);
to_bytes_test!(mulirabrb, Muli, Reg(A), BregDeref(BC), vec![0x00, 0x59]);
to_bytes_test!(divurabrb, Divu, Reg(A), BregDeref(BC), vec![0x00, 0x5A]);
to_bytes_test!(divirabrb, Divi, Reg(A), BregDeref(BC), vec![0x00, 0x5B]);
to_bytes_test!(
    muluraimm16,
    Mulu,
    Reg(A),
    Word(0x1234),
    vec![0x00, 0x5C, 0x34, 0x12]
);
to_bytes_test!(
    muliraimm16,
    Muli,
    Reg(A),
    Word(0x1234),
    vec![0x10, 0x5C, 0x34, 0x12]
);
to_bytes_test!(
    divuraimm16,
    Divu,
    Reg(A),
    Word(0x1234),
    vec![0x20, 0x5C, 0x34, 0x12]
);
to_bytes_test!(
    diviraimm16,
    Divi,
    Reg(A),
    Word(0x1234),
    vec![0x30, 0x5C, 0x34, 0x12]
);
to_bytes_test!(
    mulubraimm32,
    Mulu,
    Breg(BC),
    DWord(0x1234_5678),
    vec![0x40, 0x5C, 0x78, 0x56, 0x34, 0x12]
);
to_bytes_test!(
    mulibraimm32,
    Muli,
    Breg(BC),
    DWord(0x1234_5678),
    vec![0x50, 0x5C, 0x78, 0x56, 0x34, 0x12]
);
to_bytes_test!(
    divubraimm32,
    Divu,
    Breg(BC),
    DWord(0x1234_5678),
    vec![0x60, 0x5C, 0x78, 0x56, 0x34, 0x12]
);
to_bytes_test!(
    divibraimm32,
    Divi,
    Breg(BC),
    DWord(0x1234_5678),
    vec![0x70, 0x5C, 0x78, 0x56, 0x34, 0x12]
);
to_bytes_test!(
    muluvraimm8,
    Mulu,
    Vreg(A1),
    Byte(0xAB),
    vec![0x80, 0x5C, 0xAB]
);
to_bytes_test!(
    mulivraimm8,
    Muli,
    Vreg(A1),
    Byte(0xAB),
    vec![0x90, 0x5C, 0xAB]
);
to_bytes_test!(
    divuvraimm8,
    Divu,
    Vreg(A1),
    Byte(0xAB),
    vec![0xA0, 0x5C, 0xAB]
);
to_bytes_test!(
    divivraimm8,
    Divi,
    Vreg(A1),
    Byte(0xAB),
    vec![0xB0, 0x5C, 0xAB]
);
