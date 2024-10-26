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
    /// The binary output.
    binary: Vec<u8>,
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
            binary: vec![],
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
            if self.consume_regex(vec![ws_re()], false).is_some() {
                continue 'ws_com_loop;
            }

            // Match + remove comments.
            if self.consume_regex(vec![com_re()], false).is_some() {
                continue 'ws_com_loop;
            }

            // Match + remove multi-line comments.
            if self.consume_regex(vec![mcom_re()], false).is_some() {
                continue 'ws_com_loop;
            }

            break 'ws_com_loop;
        }
    }

    /// Parse an instruction.
    fn parse_instruction(&mut self) -> eyre::Result<()> {
        // TODO match all instructions instead of just ld
        let instr_string = match self.consume_regex(vec![ld_re()], true) {
            Some(result) => result,
            None => {
                return self.parse_error(String::from("Unknown instruction."));
            }
        };

        for (arg_string, reg_num) in self.parse_instr_args()? {
            match reg_num {
                // reg
                0 => {}
                // breg
                1 => {}
                // vreg
                2 => {}
                // breg deref
                3 => {}
                // stack pointer
                4 => {}
                // program counter
                5 => {}
                // binary
                6 => {}
                // hex
                7 => {}
                // octal
                8 => {}
                // decimal
                9 => {}
                _ => unreachable!("Regex number {} out of range.", reg_num),
            }
        }

        Ok(())
    }

    /// Parse instruction arguments.
    fn parse_instr_args(&mut self) -> eyre::Result<Vec<(String, usize)>> {
        let mut arg_strings: Vec<(String, usize)> = Vec::new();

        'parse_args_loop: loop {
            match self.consume_regex(instr_arg_res(), true) {
                Some(result) => arg_strings.push(result),
                None => self.parse_error(String::from("Expected argument."))?,
            }

            match self.consume_regex(vec![comma_re(), semicolon_re()], true) {
                // Semicolon was found, instruction is done
                Some((_, 1)) => break 'parse_args_loop,
                // Comma was found, continue
                Some((_, _)) => continue 'parse_args_loop,
                // Match failure
                None => self.parse_error(String::from("Expected `,` or `;`."))?,
            }
        }

        Ok(arg_strings)
    }

    /// If any regex matches the start of the input, consume the match and return the value of said
    /// match along with the index of the regex that matched it.
    fn consume_regex(
        &mut self,
        res: Vec<&'static Regex>,
        skip_ws_com_after: bool,
    ) -> Option<(String, usize)> {
        for (i, re) in res.iter().enumerate() {
            if let Some(mtch) = re.find(&self.input) {
                let match_str = mtch.as_str().to_owned();

                let mut chars = self.input.chars();
                for _ in match_str.chars() {
                    chars.next();
                }
                self.input = chars.as_str().to_owned();

                if self.debug {
                    println!("{match_str}");
                }

                if skip_ws_com_after {
                    self.skip_ws_com();
                }

                return Some((match_str, i));
            }
        }
        None
    }
}

/// Return a vector of all the regexes which can match instruction arguments.
fn instr_arg_res() -> Vec<&'static Regex> {
    vec![
        reg_re(),
        breg_re(),
        vreg_re(),
        breg_deref_re(),
        sp_re(),
        pc_re(),
        binary_re(),
        hex_re(),
        octal_re(),
        decimal_re(),
    ]
}

// --- STATIC REGEXES ---
macro_rules! static_re {
    ($fn_name:ident, $const_name:ident, $reg_str:literal) => {
        fn $fn_name() -> &'static Regex {
            static $const_name: OnceLock<Regex> = OnceLock::new();
            $const_name.get_or_init(|| Regex::new($reg_str).unwrap())
        }
    };
}
static_re!(ws_re, WS_RE, r"^\s+");
static_re!(com_re, COM_RE, r"^//.*[\n$]");
static_re!(mcom_re, MCOM_RE, r"(?s)^/\*.*?\*/");
static_re!(ld_re, LD_RE, r"(?i)^ld\b");
static_re!(reg_re, REG_RE, r"^[A-EHL]\b");
static_re!(breg_re, BREG_RE, r"^(?:BC|DE|HL)\b");
static_re!(breg_deref_re, BREG_DEREF_RE, r"^\[(?:BC|DE|HL)\]\b");
static_re!(vreg_re, REG_RE, r"^[A-EHL][01]\b");
static_re!(decimal_re, DECIMAL_RE, r"^-?[\d_]+\b");
static_re!(hex_re, HEX_RE, r"^0x[\dA-Fa-f_]+\b");
static_re!(octal_re, OCTAL_RE, r"^0o[0-7_]+\b");
static_re!(binary_re, BINARY_RE, r"^0b[01_]+\b");
static_re!(sp_re, SP_RE, r"^SP\b");
static_re!(pc_re, PC_RE, r"^PC\b");
static_re!(comma_re, COMMA_RE, r"^,");
static_re!(semicolon_re, SEMICOLON_RE, r"^;");
