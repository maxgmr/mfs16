//! Performs lexical analysis on MFS-16 assembly language.
use color_eyre::eyre::{self, eyre};

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

    // TODO check for keyword

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
