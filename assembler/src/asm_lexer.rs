//! Performs lexical analysis on MFS-16 assembly language.
use color_eyre::eyre::{self, eyre};
use mfs16core::{Reg16, Reg32, Reg8};

mod token_type;

use token_type::TokenType::{self, *};

const PREFIX_SIZE: usize = 2;
const SUFFIX_SIZE: usize = 2;

const HEX_PREFIX_CHAR: char = 'x';
const OCT_PREFIX_CHAR: char = 'o';
const BIN_PREFIX_CHAR: char = 'b';

const BYTE_SUFFIX_CHAR: char = 'b';
const WORD_SUFFIX_CHAR: char = 'w';
const DWORD_SUFFIX_CHAR: char = 'd';
const QWORD_SUFFIX_CHAR: char = 'q';

/// Attempt to lex a single token from the input stream.
pub fn lex_token(data: &str) -> eyre::Result<(TokenType, usize)> {
    let next = match data.chars().next() {
        Some(c) => c,
        None => {
            return Err(eyre!("Unexpected end of file"));
        }
    };

    Ok(match next {
        '=' => (Equals, 1),
        '#' => (Pound, 1),
        '[' => (OpenBracket, 1),
        ']' => (CloseBracket, 1),
        '(' => (OpenParen, 1),
        ')' => (CloseParen, 1),
        '+' => (Plus, 1),
        '-' => (Minus, 1),
        '*' => (Asterisk, 1),
        '/' => (Slash, 1),
        '\\' => (Backslash, 1),
        ',' => (Comma, 1),
        ';' => (Semicolon, 1),
        ':' => (Colon, 1),
        '\n' => (Newline, 1),
        c if c.is_ascii_digit() => tokenise_number(data)?,
        c if is_identifier_char(c) => tokenise_identifier(data)?,
        _ => {
            return Err(eyre!("Unknown character."));
        }
    })
}

fn tokenise_identifier(data: &str) -> eyre::Result<(TokenType, usize)> {
    // Identifiers can't start with a number
    match data.chars().next() {
        None => {
            return Err(eyre!("Unexpected end of file."));
        }
        Some(c) if c.is_ascii_digit() => {
            return Err(eyre!("Identifiers can't start with a number."));
        }
        _ => {}
    }

    let (contents, num_bytes) = consume_chars_while(data, |c| c == '_' || c.is_alphanumeric())?;

    // Check if register instead of normal identifier
    if let Ok(reg) = <Reg16>::try_from(contents) {
        return Ok((Reg(reg), num_bytes));
    }
    if let Ok(breg) = <Reg32>::try_from(contents) {
        return Ok((Breg(breg), num_bytes));
    }
    if let Ok(vreg) = <Reg8>::try_from(contents) {
        return Ok((Vreg(vreg), num_bytes));
    }

    Ok((Identifier(contents.to_owned()), num_bytes))
}

fn tokenise_number(data: &str) -> eyre::Result<(TokenType, usize)> {
    let mut is_first_char = true;
    let mut can_be_prefix = false;
    // assume base 10
    let mut radix: u32 = 10;
    let mut has_prefix = false;

    // Get digits + prefix (if any)
    let (contents, contents_bytes) = consume_chars_while(data, |c| {
        // Check for format prefix
        if can_be_prefix && is_prefix_char(c) {
            is_first_char = false;
            can_be_prefix = false;
            radix = get_prefix_radix(c);
            has_prefix = true;
            return true;
        }
        can_be_prefix = false;
        if is_first_char && c == '0' {
            is_first_char = false;
            can_be_prefix = true;
            return true;
        }
        is_first_char = false;

        is_number_char(c, radix)
    })?;

    // Get type suffix
    let (type_suffix, suffix_bytes) = consume_n_chars(&data[contents_bytes..], SUFFIX_SIZE)?;

    // Parse string as a numerical token
    let token_type = match type_suffix {
        ":b" => Byte(<u8>::from_str_radix(&clean(contents, has_prefix), radix)?),
        ":w" => Word(<u16>::from_str_radix(&clean(contents, has_prefix), radix)?),
        ":d" => DWord(<u32>::from_str_radix(&clean(contents, has_prefix), radix)?),
        ":q" => QWord(<u64>::from_str_radix(&clean(contents, has_prefix), radix)?),
        _ => {
            return Err(eyre!("Expected a type."));
        }
    };

    Ok((token_type, contents_bytes + suffix_bytes))
}

/// Skip whitespace and/or comments.
fn skip_ws_com(data: &str) -> usize {
    let mut remaining = data;

    loop {
        let ws = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        let com = skip_comment(remaining);
        remaining = &remaining[com..];

        if (ws + com) == 0 {
            return data.len() - remaining.len();
        }
    }
}

fn skip_whitespace(data: &str) -> usize {
    match consume_chars_while(data, |c| c.is_whitespace()) {
        Ok((_, skipped_bytes)) => skipped_bytes,
        _ => 0,
    }
}

fn skip_comment(data: &str) -> usize {
    let pairs = [("//", "\n"), ("/*", "*/")];

    for &(pat, matcher) in &pairs {
        if data.starts_with(pat) {
            let remaining = skip_until(data, matcher);
            return data.len() - remaining.len();
        }
    }

    0
}

// ------- HELPERS -------

/// Consume n chars.
fn consume_n_chars(data: &str, n: usize) -> eyre::Result<(&str, usize)> {
    let mut count = 0;
    consume_chars_while(data, |_| {
        count += 1;
        count <= n
    })
}

/// Consume chars until a given predicate is no longer true, returning the consumed chars and the
/// number of bytes consumed.
fn consume_chars_while<F: FnMut(char) -> bool>(
    data: &str,
    mut predicate: F,
) -> eyre::Result<(&str, usize)> {
    let mut index = 0;

    for c in data.chars() {
        if !predicate(c) {
            break;
        }
        index += c.len_utf8();
    }

    if index == 0 {
        return Err(eyre!("No matches."));
    }

    Ok((&data[..index], index))
}

/// Skip chars until a pattern is found, returning everything after the pattern.
fn skip_until<'a>(mut data: &'a str, pattern: &str) -> &'a str {
    while !data.is_empty() && !data.starts_with(pattern) {
        let next_char_size = data
            .chars()
            .next()
            .expect("The string isn't empty.")
            .len_utf8();
        data = &data[next_char_size..];
    }
    &data[pattern.len()..]
}

/// Return true iff the given char is a valid identifier char.
fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || (c == '_')
}

/// Return true iff the given char is a valid numerical format prefix char.
fn is_prefix_char(c: char) -> bool {
    (c == HEX_PREFIX_CHAR) || (c == OCT_PREFIX_CHAR) || (c == BIN_PREFIX_CHAR)
}

/// Match the given prefix char to its respective radix.
fn get_prefix_radix(c: char) -> u32 {
    match c {
        HEX_PREFIX_CHAR => 16,
        OCT_PREFIX_CHAR => 8,
        BIN_PREFIX_CHAR => 2,
        _ => 10,
    }
}

/// Return true iff the given char is a valid numerical char given the radix.
fn is_number_char(c: char, radix: u32) -> bool {
    c.is_digit(radix) || (c == '_')
}

/// Return true iff the given char is a valid type suffix char.
fn is_type_suffix_char(c: char) -> bool {
    (c == BYTE_SUFFIX_CHAR)
        || (c == WORD_SUFFIX_CHAR)
        || (c == DWORD_SUFFIX_CHAR)
        || (c == QWORD_SUFFIX_CHAR)
}

/// Clean up a number string slice.
fn clean(val: &str, has_prefix: bool) -> String {
    trim_prefix(val, has_prefix).replace('_', "")
}

/// Trim the prefix if the value contains one.
fn trim_prefix(val: &str, has_prefix: bool) -> &str {
    if has_prefix {
        &val[PREFIX_SIZE..]
    } else {
        val
    }
}

#[cfg(test)]
mod lexer_tests;
