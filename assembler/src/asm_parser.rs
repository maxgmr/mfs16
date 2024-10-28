use std::fmt::Display;

use camino::Utf8Path;
use color_eyre::{
    eyre::{self, eyre, OptionExt},
    owo_colors::OwoColorize,
    Section, SectionExt,
};
use mfs16core::Instruction;

mod instruction_parser;

use crate::asm_lexer::{
    Token,
    TokenKind::{self, *},
};
use instruction_parser::{instr_to_bytes, Operand, Operation};

/// MFS-16 parser. Parses a list of [Token]s into machine code for the MFS-16 architecture.
pub struct Parser<'a> {
    tokens: Vec<Token>,
    token_index: usize,
    path: &'a Utf8Path,
    original: &'a str,
    output_bytes: Vec<u8>,
}
impl<'a> Parser<'a> {
    /// Create a new [Parser] with the given [Token]s, filepath, and file data.
    pub fn new(tokens: Vec<Token>, path: &'a Utf8Path, data: &'a str) -> Self {
        Self {
            tokens,
            token_index: 0,
            path,
            original: data,
            output_bytes: Vec::new(),
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

    /// Get the next when no next should be unreachable.
    fn next_expected(&mut self) -> eyre::Result<&Token> {
        Ok(self.next().expect("Unreachable: no next."))
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

    /// Parse an instruction into a vector of bytes.
    fn parse_instr(&mut self) -> eyre::Result<Option<Vec<u8>>> {
        let mut comma_found = false;

        let operation = match self.parse_operation()? {
            Some(op) => op,
            None => return Ok(None),
        };

        let first_operand = self.parse_operand()?;

        if first_operand.is_some() {
            if let Some(&Comma) = self.peek() {
                self.next_expected()?;
                comma_found = true;
            }
        }

        let second_operand = if first_operand.is_some() {
            self.parse_operand()?
        } else {
            Operand::None
        };

        if second_operand.is_some() && !comma_found {
            return Err(eyre!("Expected `,`."));
        }

        if first_operand.is_some() && second_operand.is_some() {
            if let Some(&Semicolon) = self.peek() {
                self.next_expected()?;
            } else {
                return Err(eyre!("Expected `;`."));
            }
        }

        match instr_to_bytes(&operation, &first_operand, &second_operand) {
            Ok(val) => Ok(Some(val)),
            Err(e) => Err(e),
        }
    }

    fn parse_operation(&mut self) -> eyre::Result<Option<Operation>> {
        match self.peek() {
            Some(&Identifier(_)) => {}
            _ => return Err(eyre!("Expected instruction keyword.")),
        };

        let next = self.next_expected()?;
        match &next.kind {
            Identifier(string) => match string.parse::<Operation>() {
                Ok(op) => Ok(Some(op)),
                Err(_) => Ok(None),
            },
            _ => Err(eyre!("Unreachable: bad Token variant.")),
        }
    }

    /// Return `Ok(Some())` if operand, `Ok(None)` if end of instruction, and `Err` if error.
    fn parse_operand(&mut self) -> eyre::Result<Operand> {
        match self.peek() {
            Some(&OpenBracket) => return self.parse_deref(),
            Some(&Identifier(_))
            | Some(&Byte(_))
            | Some(&Word(_))
            | Some(&DWord(_))
            | Some(&QWord(_))
            | Some(&Reg(_))
            | Some(&Breg(_))
            | Some(&Vreg(_))
            | Some(&ProgramCounter)
            | Some(&StackPointer)
            | Some(&Semicolon) => {}
            _ => return Err(eyre!("Expected an operand or `;`.")),
        };

        let next = self.next_expected()?;
        match &next.kind {
            Identifier(id) => Ok(Operand::Variable(id.clone())),
            Byte(b) => Ok(Operand::Byte(*b)),
            Word(w) => Ok(Operand::Word(*w)),
            DWord(d) => Ok(Operand::DWord(*d)),
            QWord(q) => Ok(Operand::QWord(*q)),
            Reg(reg) => Ok(Operand::Reg(*reg)),
            Breg(breg) => Ok(Operand::Breg(*breg)),
            Vreg(vreg) => Ok(Operand::Vreg(*vreg)),
            ProgramCounter => Ok(Operand::ProgramCounter),
            StackPointer => Ok(Operand::StackPointer),
            Semicolon => Ok(Operand::None),
            _ => Err(eyre!("Unreachable: bad Token variant.")),
        }
    }

    fn parse_deref(&mut self) -> eyre::Result<Operand> {
        match self.peek() {
            Some(&OpenBracket) => {}
            _ => return Err(eyre!("Unreachable: should be open bracket.")),
        }
        let next = self.next_expected()?;
        match &next.kind {
            OpenBracket => {}
            _ => return Err(eyre!("Unreachable: bad Token variant.")),
        }

        match self.peek() {
            Some(&Breg(_)) => {}
            _ => return Err(eyre!("Expected big register.")),
        }
        let next = self.next_expected()?;
        let breg_value = match &next.kind {
            Breg(breg) => *breg,
            _ => return Err(eyre!("Unreachable: bad Token variant.")),
        };

        match self.peek() {
            Some(&CloseBracket) => {}
            _ => return Err(eyre!("Expected `]`.")),
        }
        self.next_expected()?;
        Ok(Operand::BregDeref(breg_value))
    }
}

#[cfg(test)]
mod parser_tests;
