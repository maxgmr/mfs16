use std::{collections::HashMap, fmt::Display};

use camino::Utf8Path;
use color_eyre::{
    eyre::{self, eyre, OptionExt},
    owo_colors::OwoColorize,
    Section, SectionExt,
};

mod instruction_parser;

use crate::asm_lexer::{
    Token,
    TokenKind::{self, *},
};
use instruction_parser::{instr_to_bytes, Operand, Operation};

/// Get the next value when one of a list of [TokenKind] is expected.
macro_rules! get_next_expected {
    ($parser: ident, $expected_token_str:literal, $($pattern:pat),+) => {{
        match $parser.peek() {
            $(
                Some(&$pattern)
            )|+ => {
            }
            _ => return Err(eyre!("Expected {}.", $expected_token_str)),
        };
        $parser.next_expected()?
    }};
}

/// Parse a valid list of MFS-16 assembly [Token]s into machine code for the MFS-16 architecture.
pub fn parse(
    tokens: Vec<Token>,
    path: &Utf8Path,
    data: &str,
    bytes_offset: usize,
    debug: bool,
) -> eyre::Result<Vec<u8>> {
    let mut parser = Parser::new(tokens, path, data, bytes_offset, debug);
    let mut output_bytes: Vec<u8> = Vec::new();

    while let Some(bytes) = parser.parse_next()? {
        output_bytes.extend(bytes);
    }

    Ok(output_bytes)
}

/// MFS-16 parser. Parses a list of [Token]s into machine code for the MFS-16 architecture.
pub struct Parser<'a> {
    tokens: Vec<Token>,
    token_index: usize,
    variables: HashMap<String, Variable>,
    path: &'a Utf8Path,
    original: &'a str,
    bytes_parsed: usize,
    debug: bool,
}
impl<'a> Parser<'a> {
    /// Create a new [Parser] with the given [Token]s, filepath, and file data.
    pub fn new(
        tokens: Vec<Token>,
        path: &'a Utf8Path,
        data: &'a str,
        bytes_offset: usize,
        debug: bool,
    ) -> Self {
        Self {
            tokens,
            token_index: 0,
            variables: HashMap::new(),
            path,
            original: data,
            debug,
            bytes_parsed: bytes_offset,
        }
    }

    /// Parse the next statement. Return the bytes parsed from the statement, or [Option::None] if
    /// the end of the list of [Token]s has been reached.
    fn parse_next(&mut self) -> eyre::Result<Option<Vec<u8>>> {
        if self.token_index == self.tokens.len() {
            return Ok(None);
        }

        // Prioritise instructions over variable assignments.
        match self.parse_instr() {
            Ok(Some(bytes)) => {
                self.bytes_parsed += bytes.len();
                return Ok(Some(bytes));
            }
            // Wasn't an instruction but was an identifier. Check to see if something else.
            Ok(None) => {}
            Err(e) => return self.parsing_error(e.to_string()),
        };

        // Attempt to parse a label.
        match self.parse_label() {
            Ok(Some(_)) => return Ok(Some(Vec::new())),
            // Wasn't a label but was an identifier. Check to see if something else.
            Ok(None) => {}
            Err(e) => return self.parsing_error(e.to_string()),
        };

        // Attempt to parse a variable assignment.
        match self.parse_assignment() {
            Ok(_) => Ok(Some(Vec::new())),
            Err(e) => self.parsing_error(e.to_string()),
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

    /// Peek two [Token]s ahead.
    fn dbl_peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.token_index + 1).map(|t| &t.kind)
    }

    /// Get the current [Token], advancing the token index.
    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.token_index);

        if let Some(t) = &token {
            if self.debug {
                println!("Consumed `{:?}`.", t);
            }
            self.token_index += 1;
        }

        token
    }

    /// Get the next when no next should be unreachable.
    fn next_expected(&mut self) -> eyre::Result<&Token> {
        Ok(self.next().expect("Unreachable: no next."))
    }

    /// Return an error, showing where in the file the error occurred.
    fn parsing_error<S: AsRef<str> + Display>(&self, message: S) -> eyre::Result<Option<Vec<u8>>> {
        let line_num = self.line_num()?;
        let col_num = self.col_num()?;
        let consumed_line = self.consumed_line()?;
        let remaining_line = self.remaining_line()?;

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

    /// Get the number of consumed characters so far.
    fn num_consumed_chars(&self) -> eyre::Result<usize> {
        self.current_index().ok_or_eyre("Invalid token index.")
    }

    /// Get the lines consumed so far.
    fn consumed_lines(&self) -> eyre::Result<Vec<&str>> {
        dbg!(self.original[..self.num_consumed_chars()?]
            .split("\n")
            .collect::<Vec<&str>>());
        Ok(self.original[..self.num_consumed_chars()?]
            .split("\n")
            .collect())
    }

    /// Get the current line number.
    fn line_num(&self) -> eyre::Result<usize> {
        Ok(self.consumed_lines()?.len())
    }

    /// Get the already-consumed text of the current line.
    fn consumed_line(&self) -> eyre::Result<&str> {
        Ok(self
            .consumed_lines()?
            .last()
            .ok_or_eyre("No consumed lines.")?)
    }

    /// Get the current column number.
    fn col_num(&self) -> eyre::Result<usize> {
        Ok(self.consumed_line()?.len())
    }

    /// Get the unconsumed text of the current line.
    fn remaining_line(&self) -> eyre::Result<&str> {
        self.original[self.num_consumed_chars()?..]
            .split("\n")
            .next()
            .ok_or_eyre("No remaining line.")
    }

    // ------- PARSER FUNCTIONS -------

    /// Parse a variable assignment.
    fn parse_assignment(&mut self) -> eyre::Result<()> {
        let assignee_name = match self.peek() {
            Some(Identifier(string)) => match string.parse::<Operation>() {
                Ok(op) => return Err(eyre!("Variable name cannot be instruction name `{}`.", op)),
                Err(_) => string.clone(),
            },
            _ => return Err(eyre!("Expected an identifier.")),
        };
        self.next_expected()?;

        get_next_expected!(self, "`=`.\nIf this line is intended to be an instruction, then the instruction name is invalid", Equals);

        let value = self.parse_variable_value()?;

        get_next_expected!(self, "`;`", Semicolon);

        self.variables.insert(assignee_name, value);

        Ok(())
    }

    /// Parse a label, returning the memory address of said label.
    fn parse_label(&mut self) -> eyre::Result<Option<usize>> {
        let label_name = match self.peek() {
            Some(Identifier(string)) => match string.parse::<Operation>() {
                Ok(op) => return Err(eyre!("Variable name cannot be instruction name `{}`.", op)),
                Err(_) => string.clone(),
            },
            _ => return Err(eyre!("Expected an identifier.")),
        };
        match self.dbl_peek() {
            Some(Colon) => {}
            _ => return Ok(None),
        };
        get_next_expected!(self, "identifier", Identifier(_));
        get_next_expected!(self, "`;`", Colon);

        self.variables
            .insert(label_name, Variable::DWord(self.bytes_parsed.try_into()?));

        Ok(Some(self.bytes_parsed))
    }

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
        let id_string = match self.peek() {
            Some(Identifier(string)) => match string.parse::<Operation>() {
                Ok(op) => op,
                // Not an instruction!
                Err(_) => return Ok(None),
            },
            _ => return Err(eyre!("Expected identifier.")),
        };
        self.next_expected()?;
        Ok(Some(id_string))
    }

    /// Return `Ok(Some())` if operand, `Ok(None)` if end of instruction, and `Err` if error.
    fn parse_operand(&mut self) -> eyre::Result<Operand> {
        // Handle non-atomic operand cases separately.
        match self.peek() {
            Some(&OpenBracket) => return self.parse_deref(),
            Some(&Identifier(_)) => return Ok(self.parse_variable()?.into_operand()),
            _ => {}
        };

        let next = get_next_expected!(
            self,
            "an operand or `;`.",
            Byte(_),
            Word(_),
            DWord(_),
            QWord(_),
            Reg(_),
            Breg(_),
            Vreg(_),
            ProgramCounter,
            StackPointer,
            Semicolon
        );
        match &next.kind {
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

    /// Get the value of a variable.
    fn parse_variable(&mut self) -> eyre::Result<Variable> {
        let variable_name = match self.peek() {
            Some(Identifier(val)) => val.clone(),
            _ => {
                return Err(eyre!(
                    "Unreachable: was already determined to be identifier."
                ))
            }
        };
        let value = match self.variables.get(&variable_name) {
            Some(val) => *val,
            None => return Err(eyre!("Variable `{}` not found.", variable_name)),
        };
        self.next_expected()?;
        Ok(value)
    }

    /// Get the value being assigned to a variable.
    fn parse_variable_value(&mut self) -> eyre::Result<Variable> {
        if let Some(&Identifier(_)) = self.peek() {
            return self.parse_variable();
        }

        let value = match self.peek() {
            Some(&Byte(b)) => Variable::Byte(b),
            Some(&Word(w)) => Variable::Word(w),
            Some(&DWord(d)) => Variable::DWord(d),
            Some(&QWord(q)) => Variable::QWord(q),
            _ => return Err(eyre!("Expected value to assign to variable.")),
        };
        self.next_expected()?;
        Ok(value)
    }

    fn parse_deref(&mut self) -> eyre::Result<Operand> {
        get_next_expected!(self, "`[`", OpenBracket);

        let next = get_next_expected!(self, "big register or dword", Breg(_), DWord(_));
        let operand = match &next.kind {
            Breg(breg) => Operand::BregDeref(*breg),
            DWord(d) => Operand::DWordDeref(*d),
            _ => return Err(eyre!("Unreachable: already known to be big register.")),
        };

        get_next_expected!(self, "`]`", CloseBracket);

        Ok(operand)
    }
}

/// All the types which can be stored as a variable.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Variable {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),
}
impl Variable {
    fn into_operand(self) -> Operand {
        match self {
            Self::Byte(b) => Operand::Byte(b),
            Self::Word(w) => Operand::Word(w),
            Self::DWord(d) => Operand::DWord(d),
            Self::QWord(q) => Operand::QWord(q),
        }
    }
}
impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Byte(b) => format!("{:#04X}:b", b),
                Self::Word(w) => format!("{:#06X}:w", w),
                Self::DWord(d) => format!("{:#010X}:d", d),
                Self::QWord(q) => format!("{:#018X}:q", q),
            }
        )
    }
}

#[cfg(test)]
mod parser_tests;
