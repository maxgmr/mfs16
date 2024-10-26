use pretty_assertions::assert_eq;

use super::*;

macro_rules! lexer_test_expect {
    ($test_name:ident, $fun:ident, $val:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let val: &str = $val;
            let expected_token = TokenType::from($expected);
            let fun = $fun;

            let (result, _) = fun(val).unwrap();
            assert_eq!(result, expected_token, "Input value was {:?}.", val);
        }
    };
}
macro_rules! lexer_test_expect_fail {
    ($test_name:ident, $fun:ident, $val:expr) => {
        #[test]
        fn $test_name() {
            let val: &str = $val;
            let fun = $fun;

            let result = fun(val);
            assert!(result.is_err(), "{:?} should be error.", result);
        }
    };
}

lexer_test_expect!(
    test_tokenise_single_letter,
    tokenise_identifier,
    "m",
    Identifier("m".to_owned())
);
lexer_test_expect!(
    tokenise_foo_bar,
    tokenise_identifier,
    "foo_bar",
    Identifier("foo_bar".to_owned())
);
lexer_test_expect!(test_tokenise_zh, tokenise_identifier, "很好", "很好");

lexer_test_expect_fail!(test_tokenise_number_start, tokenise_identifier, "7foo_bar");
lexer_test_expect_fail!(test_tokenise_poop, tokenise_identifier, "💩");

lexer_test_expect!(test_tokenise_zero_byte, tokenise_number, "0:b", Byte(0));
lexer_test_expect!(test_tokenise_zero_word, tokenise_number, "0:w", Word(0));
lexer_test_expect!(test_tokenise_zero_dword, tokenise_number, "0:d", DWord(0));
lexer_test_expect!(test_tokenise_zero_qword, tokenise_number, "0:q", QWord(0));
lexer_test_expect!(test_uscore_aft_prefix, tokenise_number, "0b_1010:b", 0xA_u8);
lexer_test_expect!(test_mult_uscore, tokenise_number, "0xF_____F:w", Word(0xFF));
lexer_test_expect!(test_uscore_end, tokenise_number, "0xfF_:w", 0xFF_u16);
lexer_test_expect!(test_oct, tokenise_number, "0o76543210:d", 0o7654_3210_u32);
lexer_test_expect!(test_trailing_chars, tokenise_number, "0x12:b; //", 0x12u8);
lexer_test_expect!(test_d_hex, tokenise_number, "0xdddddddd:d", 0xDDDDDDDDu32);
lexer_test_expect_fail!(test_wrong_radix, tokenise_number, "0bff:b");
lexer_test_expect_fail!(test_b_too_big, tokenise_number, "0x1_00_:b");
lexer_test_expect_fail!(test_w_too_big, tokenise_number, "0x1_0000_:w");
lexer_test_expect_fail!(test_d_too_big, tokenise_number, "0x1_0000_0000_:d");
lexer_test_expect_fail!(test_q_too_big, tokenise_number, "0x10000000000000000:q");
lexer_test_expect_fail!(test_no_type, tokenise_number, "1234");
lexer_test_expect_fail!(test_bad_type_1, tokenise_number, "1234:i");
lexer_test_expect_fail!(test_bad_type_2, tokenise_number, "1234:{");
lexer_test_expect_fail!(test_tokenise_empty_byte, tokenise_number, "0x:b");
