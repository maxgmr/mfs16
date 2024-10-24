//! Assembly parser.
use std::{fs::File, io::Read, path::Path, sync::OnceLock};

use camino::Utf8Path;
use color_eyre::eyre;
use regex::{bytes::Match, Regex};

/// Assembly parser.
pub struct AsmParser {
    /// The current text being parsed.
    input: String,
}
impl AsmParser {
    /// Create a new [AsmParser] from the given file path.
    pub fn from_path<T: AsRef<Utf8Path> + AsRef<Path>>(file_path: T) -> eyre::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        Ok(Self { input })
    }

    /// Parse the input.
    pub fn parse(&mut self) {
        self.skip_ws_com();
    }

    /// Skip whitespace and comments.
    fn skip_ws_com(&mut self) {
        // Match + remove whitespace.
        if let Some(mtch) = ws_reg().find(&self.input) {
            print_match("whitespace", mtch.as_str());
            self.rm_first_n(mtch.end());
        };

        // Match + remove comments.
        if let Some(mtch) = comment_reg().find(&self.input) {
            print_match("comment", mtch.as_str());
            self.rm_first_n(mtch.end());
        }

        // Match + remove multi-line comments.
        if let Some(mtch) = multiline_comment_reg().find(&self.input) {
            print_match("comment", mtch.as_str());
            self.rm_first_n(mtch.end());
        }

        println!("{}", self.input);
    }

    fn rm_first_n(&mut self, n: usize) {
        let mut chars = self.input.chars();
        for _ in 0..(n) {
            chars.next();
        }
        self.input = chars.as_str().to_owned()
    }
}

fn print_match(match_name: &'static str, mtch: &str) {
    println!("Found {}: <{}>", match_name, mtch);
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
static_reg!(comment_reg, COMMENT_REG, r"^//.*[\n$]");
static_reg!(
    multiline_comment_reg,
    MULTILINE_COMMENT_REG,
    r"(?s)^/\*.*?\*/"
);
