//! Performs lexical analysis on MFS-16 assembly language.
use std::fmt::Display;

use color_eyre::{
    eyre::{self, eyre, OptionExt},
    owo_colors::OwoColorize,
    Section, SectionExt,
};
use mfs16core::{Reg16, Reg32, Reg8};

mod token;

// Re-exports
pub use token::{
    Token,
    TokenKind::{self, *},
};

const PREFIX_SIZE: usize = 2;

const HEX_PREFIX_CHAR: char = 'x';
const OCT_PREFIX_CHAR: char = 'o';
const BIN_PREFIX_CHAR: char = 'b';

const BYTE_SUFFIX: &str = ":b";
const WORD_SUFFIX: &str = ":w";
const DWORD_SUFFIX: &str = ":d";
const QWORD_SUFFIX: &str = ":q";

const SP_STRING: &str = "SP";
const PC_STRING: &str = "PC";

struct Lexer<'a> {
    index: usize,
    original: &'a str,
    remaining: &'a str,
}
impl<'a> Lexer<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            index: 0,
            original: data,
            remaining: data,
        }
    }

    /// Return an error showing where in the file the error occurred.
    fn lexing_error<S: AsRef<str> + Display>(&self, message: S) -> eyre::Result<Option<Token>> {
        let num_consumed_chars = self.original.len() - self.remaining.len();
        let consumed_lines: Vec<&str> = self.original[..num_consumed_chars].split("\n").collect();
        let line_num = consumed_lines.len();
        let consumed_line = consumed_lines.last().ok_or_eyre("No consumed lines.")?;
        let col_num = consumed_line.len();
        let remaining_line: &str = self
            .remaining
            .split("\n")
            .next()
            .ok_or_eyre("No remaining line.")?;

        Err(eyre!("{}", message))
            .with_section(|| {
                format!("{}:{}", line_num.blue(), (col_num + 1).blue()).header("Line Info:")
            })
            .with_section(|| {
                format!(
                    "{}{}\n{}{}",
                    consumed_line,
                    remaining_line,
                    (0..col_num).map(|_| " ").collect::<String>(),
                    "^ Here".bright_red().bold(),
                )
            })
    }

    fn next_token(&mut self) -> eyre::Result<Option<Token>> {
        self.skip_whitespace_comments();

        if self.remaining.is_empty() {
            Ok(None)
        } else {
            let start_index = self.index;
            let token = match self._next_token() {
                Ok(val) => val,
                Err(e) => return self.lexing_error(e.to_string()),
            };
            Ok(Some(Token::new(start_index, self.index, token)))
        }
    }

    fn _next_token(&mut self) -> eyre::Result<TokenKind> {
        let (token, num_bytes) = lex_token(self.remaining)?;
        self.consume(num_bytes);
        Ok(token)
    }

    fn skip_whitespace_comments(&mut self) {
        let num_bytes = skip_ws_com(self.remaining);
        self.consume(num_bytes);
    }

    fn consume(&mut self, num_bytes: usize) {
        self.remaining = &self.remaining[num_bytes..];
        self.index += num_bytes;
    }
}

/// Lex a string of valid MFS-16 assembly code into a list of tokens alongside each token's start
/// and end indicies in the original assembly code.
pub fn lex(data: &str) -> eyre::Result<Vec<Token>> {
    let mut lexer = Lexer::new(data);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token()? {
        tokens.push(token);
    }

    Ok(tokens)
}

/// Attempt to lex a single token from the input stream.
pub fn lex_token(data: &str) -> eyre::Result<(TokenKind, usize)> {
    let next = match data.chars().next() {
        Some(c) => c,
        None => {
            return Err(eyre!("Unexpected end of file"));
        }
    };

    Ok(match next {
        '=' => (Equals, 1),
        '#' => (Pound, 1),
        '&' => (Ampersand, 1),
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
        c if c.is_ascii_digit() => tokenise_number(data)?,
        c if is_identifier_char(c) => tokenise_identifier(data)?,
        _ => {
            return Err(eyre!("Unknown character."));
        }
    })
}

fn tokenise_identifier(data: &str) -> eyre::Result<(TokenKind, usize)> {
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

    // Check if stack pointer instead of normal identifier
    if contents == SP_STRING {
        return Ok((StackPointer, num_bytes));
    }

    // Check if program counter instead of normal identifier
    if contents == PC_STRING {
        return Ok((ProgramCounter, num_bytes));
    }

    Ok((Identifier(contents.to_owned()), num_bytes))
}

fn tokenise_number(data: &str) -> eyre::Result<(TokenKind, usize)> {
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

    // Get type suffix (if any)
    // let (type_suffix, suffix_bytes) = consume_n_chars(&data[contents_bytes..], SUFFIX_SIZE)?;
    let mut counter = 0;
    let (type_suffix, suffix_bytes) = consume_chars_while(&data[contents_bytes..], |c| {
        if counter >= 2 {
            return false;
        }

        if counter == 0 && c == ':' {
            counter += 1;
            return true;
        }

        if counter == 1 && is_type_suffix_letter(c) {
            counter += 1;
            return true;
        }

        counter += 1;
        false
    })
    .unwrap_or(("", 0));

    // Parse string as a numerical token
    let token_type = match type_suffix {
        BYTE_SUFFIX | "" => Byte(<u8>::from_str_radix(&clean(contents, has_prefix), radix)?),
        WORD_SUFFIX => Word(<u16>::from_str_radix(&clean(contents, has_prefix), radix)?),
        DWORD_SUFFIX => DWord(<u32>::from_str_radix(&clean(contents, has_prefix), radix)?),
        QWORD_SUFFIX => QWord(<u64>::from_str_radix(&clean(contents, has_prefix), radix)?),
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

    if data.is_empty() {
        data
    } else {
        &data[pattern.len()..]
    }
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

/// Return true iff the given char matches the letter portion of any type suffix.
fn is_type_suffix_letter(c: char) -> bool {
    _is_type_suffix_letter(c, BYTE_SUFFIX)
        || _is_type_suffix_letter(c, WORD_SUFFIX)
        || _is_type_suffix_letter(c, DWORD_SUFFIX)
        || _is_type_suffix_letter(c, QWORD_SUFFIX)
}
fn _is_type_suffix_letter(c: char, type_suffix: &'static str) -> bool {
    c == type_suffix.chars().last().unwrap()
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
