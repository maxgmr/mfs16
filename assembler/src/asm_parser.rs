//! Assembly parser.
use std::{fs::File, io::Read, sync::OnceLock};

use camino::Utf8PathBuf;
use color_eyre::{
    eyre::{self, eyre, OptionExt},
    owo_colors::OwoColorize,
    Section, SectionExt,
};
use regex::Regex;

/// Assembly parser.
pub struct AsmParser {
    /// The file path.
    path: Utf8PathBuf,
    /// The complete contents of the file.
    contents: String,
    /// The current text being parsed.
    input: String,
    /// If true, print debug messages.
    debug: bool,
}
impl AsmParser {
    /// Create a new [AsmParser] from the given file path.
    pub fn from_path(path: Utf8PathBuf, debug: bool) -> eyre::Result<Self> {
        let mut file = File::open(&path)?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        let contents = input.clone();

        Ok(Self {
            path,
            contents,
            input,
            debug,
        })
    }

    /// Parse the input.
    pub fn parse_input(&mut self) -> eyre::Result<()> {
        self.skip_ws_com();

        while !self.input.is_empty() {
            self.parse_instruction()?;
        }
        Ok(())
    }

    /// Return a parsing error.
    fn parse_error(&self, message: String) -> eyre::Result<()> {
        let num_consumed_chars = self.contents.len() - self.input.len();
        let consumed_lines: Vec<&str> = self.contents[0..num_consumed_chars].split("\n").collect();
        let line_num = consumed_lines.len();
        let consumed_line = consumed_lines.last().ok_or_eyre("No consumed lines.")?;
        let col_num = consumed_line.len();
        let remaining_line: &str = self
            .input
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
                .header("Offending Line:")
            })
    }

    /// Skip whitespace and comments.
    fn skip_ws_com(&mut self) {
        'ws_com_loop: loop {
            // Match + remove whitespace.
            if self.consume_regex(ws_reg()).is_some() {
                continue 'ws_com_loop;
            }

            // Match + remove comments.
            if self.consume_regex(com_reg()).is_some() {
                continue 'ws_com_loop;
            }

            // Match + remove multi-line comments.
            if self.consume_regex(mcom_reg()).is_some() {
                continue 'ws_com_loop;
            }

            break 'ws_com_loop;
        }
    }

    /// Parse an instruction.
    fn parse_instruction(&mut self) -> eyre::Result<()> {
        if self.consume_regex(ld_reg()).is_some() {
        } else {
            self.parse_error(String::from("Unknown instruction."))?;
        }
        Ok(())
    }

    /// If regex matches start of input, consume the match from input and return said match.
    fn consume_regex(&mut self, re: &'static Regex) -> Option<String> {
        if let Some(mtch) = re.find(&self.input) {
            let match_str = mtch.as_str().to_owned();

            let mut chars = self.input.chars();
            for _ in 0..(mtch.end()) {
                chars.next();
            }
            self.input = chars.as_str().to_owned();

            if self.debug {
                println!("{match_str}");
            }
            Some(match_str)
        } else {
            None
        }
    }
}

// --- STATIC REGEXES ---
macro_rules! static_reg {
    ($fn_name:ident, $const_name:ident, $reg_str:literal) => {
        fn $fn_name() -> &'static Regex {
            static $const_name: OnceLock<Regex> = OnceLock::new();
            $const_name.get_or_init(|| Regex::new($reg_str).unwrap())
        }
    };
}
static_reg!(ws_reg, WS_REG, r"^\s+");
static_reg!(com_reg, COM_REG, r"^//.*[\n$]");
static_reg!(mcom_reg, MCOM_REG, r"(?s)^/\*.*?\*/");
static_reg!(ld_reg, LD_REG, r"(?i)^ld");
