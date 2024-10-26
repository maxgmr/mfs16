use color_eyre::eyre::{self, eyre};
use mfs16core::{Reg16, Reg32, Reg8};

/// All the valid tokens of MFS-16 ASM.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// A single byte.
    Byte(u8),
    /// A single 16-bit word.
    Word(u16),
    /// A single 32-bit dword.
    DWord(u32),
    /// A single 64-bit qword.
    QWord(u64),
    /// A single identifier; e.g., variable name
    Identifier(String),
    /// =
    Equals,
    /// #
    Pound,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// \
    Backslash,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// :
    Colon,
    /// \n
    Newline,
    /// A 16-bit register.
    Reg(Reg16),
    /// A 32-bit big register.
    Breg(Reg32),
    /// An 8-bit virtual register.
    Vreg(Reg8),
}

macro_rules! from_impl {
    ($(($type:ty, $variant:path)),+) => {
        $(impl From<$type> for TokenType {
            fn from(value: $type) -> Self {
                $variant(value)
            }
        })*
    };
}
from_impl!(
    (String, Self::Identifier),
    (u8, Self::Byte),
    (u16, Self::Word),
    (u32, Self::DWord),
    (u64, Self::QWord),
    (Reg16, Self::Reg),
    (Reg32, Self::Breg),
    (Reg8, Self::Vreg)
);
impl<'a> From<&'a str> for TokenType {
    fn from(value: &'a str) -> Self {
        Self::Identifier(value.to_owned())
    }
}
