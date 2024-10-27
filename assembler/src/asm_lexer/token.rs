use mfs16core::{Reg16, Reg32, Reg8};

/// An MFS-16 assembly code token.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The start and end of this token within its file.
    pub location: (usize, usize),
    /// The variant of this particular token.
    pub kind: TokenKind,
}
impl Token {
    /// Create a token from a given range and something which can be turned into a [TokenKind].
    pub fn new<K: Into<TokenKind>>(start: usize, end: usize, kind: K) -> Self {
        let kind = kind.into();
        let location = (start, end);
        Self { location, kind }
    }
}
impl<T> From<T> for Token
where
    T: Into<TokenKind>,
{
    fn from(value: T) -> Self {
        Self::new(0, 0, value)
    }
}

/// All the valid tokens of MFS-16 ASM.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
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
    /// &
    Ampersand,
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
    /// A 16-bit register.
    Reg(Reg16),
    /// A 32-bit big register.
    Breg(Reg32),
    /// An 8-bit virtual register.
    Vreg(Reg8),
    /// The stack pointer.
    StackPointer,
    /// The program counter.
    ProgramCounter,
}

macro_rules! from_impl {
    ($(($type:ty, $variant:path)),+) => {
        $(impl From<$type> for TokenKind {
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
impl<'a> From<&'a str> for TokenKind {
    fn from(value: &'a str) -> Self {
        Self::Identifier(value.to_owned())
    }
}
