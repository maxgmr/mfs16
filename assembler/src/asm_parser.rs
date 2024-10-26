use std::fmt::Display;

use camino::Utf8Path;
use color_eyre::{
    eyre::{self, eyre, OptionExt},
    owo_colors::OwoColorize,
    Section, SectionExt,
};

use crate::asm_lexer::{Token, TokenKind};

/// MFS-16 parser. Parses a list of [Token]s into machine code for the MFS-16 architecture.
pub struct Parser<'a> {
    tokens: Vec<Token>,
    token_index: usize,
    path: &'a Utf8Path,
    original: &'a str,
}
impl<'a> Parser<'a> {
    /// Create a new [Parser] with the given [Token]s, filepath, and file data.
    pub fn new(tokens: Vec<Token>, path: &'a Utf8Path, data: &'a str) -> Self {
        Self {
            tokens,
            token_index: 0,
            path,
            original: data,
        }
    }

    /// Get the current index within the original file.
    fn current_index(&self) -> Option<usize> {
        self.tokens.get(self.token_index).map(|t| t.location.0)
    }

    /// Peek at the current [Token].
    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.token_index).map(|t| &t.kind)
    }

    /// Get the current [Token], advancing the token index.
    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.token_index);

        if token.is_some() {
            self.token_index += 1;
        }

        token
    }

    /// Return an error showing where in the file the error occurred.
    fn parsing_error<S: AsRef<str> + Display>(&self, message: S) -> eyre::Result<()> {
        let num_consumed_chars =
            self.original.len() - self.current_index().ok_or_eyre("Invalid token index.")?;
        let consumed_lines: Vec<&str> = self.original[..num_consumed_chars].split("\n").collect();
        let line_num = consumed_lines.len();
        let consumed_line = consumed_lines.last().ok_or_eyre("No consumed lines.")?;
        let col_num = consumed_line.len();
        let remaining_line: &str = self.original[num_consumed_chars..]
            .split("\n")
            .next()
            .ok_or_eyre("No remaining line.")?;

        Err(eyre!("{}", message))
            .with_section(|| {
                format!(
                    "{}:{}:{}",
                    self.path.blue(),
                    line_num.blue(),
                    (col_num + 1).blue()
                )
                .header("Line Info:")
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

    // ------- PARSER FUNCTIONS -------
}
