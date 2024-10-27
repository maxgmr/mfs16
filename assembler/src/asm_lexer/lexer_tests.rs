use camino::Utf8PathBuf;
use mfs16core::{Reg16::*, Reg32::*, Reg8::*};
use pretty_assertions::assert_eq;

use super::*;

macro_rules! lexer_test_expect {
    ($test_name:ident, $fun:ident, $val:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let val: &str = $val;
            let expected_token = TokenKind::from($expected);
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
lexer_test_expect!(test_tokenise_sp, tokenise_identifier, "SP", StackPointer);
lexer_test_expect!(test_tokenise_pc, tokenise_identifier, "PC", ProgramCounter);
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

lexer_test_expect!(test_tokenise_byte, lex_token, "0x1B:b", Byte(0x1B));
lexer_test_expect!(test_tokenise_word, lex_token, "1234:w", Word(1234));
lexer_test_expect!(test_tokenise_dword, lex_token, "0o1:d", DWord(1));
lexer_test_expect!(test_tokenise_qword, lex_token, "0b_0101__:q", QWord(5));
lexer_test_expect!(
    test_tokenise_identifier,
    lex_token,
    "_my_var_=1234:w",
    Identifier(String::from("_my_var_"))
);
lexer_test_expect!(test_tokenise_equals, lex_token, "=blah blah blah", Equals);
lexer_test_expect!(test_tokenise_pound, lex_token, "#", Pound);
lexer_test_expect!(test_tokenise_ampersand, lex_token, "&", Ampersand);
lexer_test_expect!(test_tokenise_open_bracket, lex_token, "[", OpenBracket);
lexer_test_expect!(test_tokenise_close_bracket, lex_token, "]", CloseBracket);
lexer_test_expect!(test_tokenise_open_paren, lex_token, "(", OpenParen);
lexer_test_expect!(test_tokenise_close_paren, lex_token, ")", CloseParen);
lexer_test_expect!(test_tokenise_plus, lex_token, "+", Plus);
lexer_test_expect!(test_tokenise_minus, lex_token, "-", Minus);
lexer_test_expect!(test_tokenise_asterisk, lex_token, "*", Asterisk);
lexer_test_expect!(test_tokenise_slash, lex_token, "/", Slash);
lexer_test_expect!(test_tokenise_backslash, lex_token, r"\", Backslash);
lexer_test_expect!(test_tokenise_comma, lex_token, ",", Comma);
lexer_test_expect!(test_tokenise_semicolon, lex_token, ";", Semicolon);
lexer_test_expect!(test_tokenise_colon, lex_token, ":", Colon);
lexer_test_expect!(
    test_tokenise_vreg_start,
    lex_token,
    "A0B",
    Identifier(String::from("A0B"))
);
lexer_test_expect!(test_tokenise_vreg_mixed, lex_token, "A0,B", Vreg(A0));
lexer_test_expect!(test_tokenise_reg, lex_token, "A", Reg(A));
lexer_test_expect!(test_tokenise_breg, lex_token, "HL", Breg(HL));
lexer_test_expect!(test_tokenise_vreg, lex_token, "E0", Vreg(E0));

#[test]
fn test_skip_whitespace() {
    assert_eq!(skip_whitespace(" \t\n \rhullo! "), 5);
}

#[test]
fn test_skip_no_whitespace() {
    assert_eq!(skip_whitespace("wass6p everyone"), 0);
}

#[test]
fn test_skip_comment() {
    assert_eq!(skip_comment("// hello /**/ \n // there!"), 15);
}

#[test]
fn test_skip_multiline_comment() {
    assert_eq!(
        skip_comment("/*skip\nme!\n\n\t\tpleeeease*/ but not me \n or me"),
        25
    );
}

#[test]
fn test_skip_not_comment() {
    assert_eq!(skip_comment("hello // don't take this away!"), 0);
    assert_eq!(skip_comment("  // ignore whitespace too!"), 0);
}

#[test]
fn test_lex_full_expr() {
    let expr = "LD(A0, A1);";
    let expect = vec![
        Token::new(0, 2, TokenKind::from("LD")),
        Token::new(2, 3, OpenParen),
        Token::new(3, 5, Vreg(A0)),
        Token::new(5, 6, Comma),
        Token::new(7, 9, Vreg(A1)),
        Token::new(9, 10, CloseParen),
        Token::new(10, 11, Semicolon),
    ];
    assert_eq!(lex(expr, &Utf8PathBuf::from("")).unwrap(), expect);
}

#[test]
fn test_lex_multiline() {
    let data =
        "// Set A to 0x1234.\n\tld\n\t\t(A,\n\t\t0x1200:w + 0x34:w)\n;\n/*\n\tAdd BC to HL.\n*/\nadd(HL,BC);\t\t\t// Preddy cool!";
    let expect = vec![
        Token::new(21, 23, TokenKind::from("ld")),
        Token::new(26, 27, OpenParen),
        Token::new(27, 28, Reg(A)),
        Token::new(28, 29, Comma),
        Token::new(32, 40, Word(0x1200)),
        Token::new(41, 42, Plus),
        Token::new(43, 49, Word(0x0034)),
        Token::new(49, 50, CloseParen),
        Token::new(51, 52, Semicolon),
        Token::new(74, 77, TokenKind::from("add")),
        Token::new(77, 78, OpenParen),
        Token::new(78, 80, Breg(HL)),
        Token::new(80, 81, Comma),
        Token::new(81, 83, Breg(BC)),
        Token::new(83, 84, CloseParen),
        Token::new(84, 85, Semicolon),
    ];
    assert_eq!(lex(data, &Utf8PathBuf::from("")).unwrap(), expect);
}

#[test]
fn detect_invalid() {
    let _ = lex("blah blah blah~", &Utf8PathBuf::from("")).unwrap_err();
}
