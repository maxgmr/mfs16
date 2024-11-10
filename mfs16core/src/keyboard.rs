use std::default::Default;

/// Size of the keyboard register.
pub const KB_REG_SIZE: usize = 0x0000_0040;

/// The register storing the state of the keyboard keys. Each byte corresponds to a key.
/// 1 = pressed, 0 = not pressed.
#[derive(Debug, Clone, PartialEq)]
pub struct KbReg {
    /// The raw byte contents of the keyboard register.
    bytes: [u8; KB_REG_SIZE],
}
impl KbReg {
    /// Create a new [KbReg].
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the status of the bit corresponding to the given [KbCode] or index.
    pub fn key<C: Into<u16> + Copy>(&self, code: C) -> bool {
        let (byte_index, bit_index) = Self::byte_and_bit_indicies(code);
        (self.bytes[byte_index as usize] & (1 << bit_index)) != 0
    }

    /// Set the register bit corresponding to the given [KbCode] or index.
    pub fn key_down<C: Into<u16> + Copy>(&mut self, code: C) {
        self.change_bit(code, true);
    }

    /// Reset the register bit corresponding to the given [KbCode] or index.
    pub fn key_up<C: Into<u16> + Copy>(&mut self, code: C) {
        self.change_bit(code, false);
    }

    fn change_bit<C: Into<u16> + Copy>(&mut self, code: C, value: bool) {
        let (byte_index, bit_index) = Self::byte_and_bit_indicies(code);

        if value {
            self.bytes[byte_index as usize] ^= 1 << bit_index;
        } else {
            self.bytes[byte_index as usize] &= !(1 << bit_index);
        }
    }

    /// Get the indicies of the byte and bit corresponding to the given index.
    fn byte_and_bit_indicies<C: Into<u16> + Copy>(index: C) -> (u8, u8) {
        let quotient = index.into() / (<u8>::BITS as u16);
        let remainder = index.into() % (<u8>::BITS as u16);
        (quotient as u8, remainder as u8)
    }
}
impl Default for KbReg {
    fn default() -> Self {
        Self {
            bytes: [0; KB_REG_SIZE],
        }
    }
}

/// The different keys of the keyboard. Corresponds to SDL2 scancodes.
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KbCode {
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Num1 = 30,
    Num2 = 31,
    Num3 = 32,
    Num4 = 33,
    Num5 = 34,
    Num6 = 35,
    Num7 = 36,
    Num8 = 37,
    Num9 = 38,
    Num0 = 39,
    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    NonUsHash = 50,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    NumLockClear = 83,
    KpDivide = 84,
    KpMultiply = 85,
    KpMinus = 86,
    KpPlus = 87,
    KpEnter = 88,
    Kp1 = 89,
    Kp2 = 90,
    Kp3 = 91,
    Kp4 = 92,
    Kp5 = 93,
    Kp6 = 94,
    Kp7 = 95,
    Kp8 = 96,
    Kp9 = 97,
    Kp0 = 98,
    KpPeriod = 99,
    NonUsBackslash = 100,
    Application = 101,
    Power = 102,
    KpEquals = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    Execute = 116,
    Help = 117,
    Menu = 118,
    Select = 119,
    Stop = 120,
    Again = 121,
    Undo = 122,
    Cut = 123,
    Copy = 124,
    Paste = 125,
    Find = 126,
    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,
    KpComma = 133,
    KpEqualsAS400 = 134,
    International1 = 135,
    International2 = 136,
    International3 = 137,
    International4 = 138,
    International5 = 139,
    International6 = 140,
    International7 = 141,
    International8 = 142,
    International9 = 143,
    Lang1 = 144,
    Lang2 = 145,
    Lang3 = 146,
    Lang4 = 147,
    Lang5 = 148,
    Lang6 = 149,
    Lang7 = 150,
    Lang8 = 151,
    Lang9 = 152,
    AltErase = 153,
    SysReq = 154,
    Cancel = 155,
    Clear = 156,
    Prior = 157,
    Return2 = 158,
    Separator = 159,
    Out = 160,
    Oper = 161,
    ClearAgain = 162,
    CrSel = 163,
    ExSel = 164,
    Kp00 = 176,
    Kp000 = 177,
    ThousandsSeparator = 178,
    DecimalSeparator = 179,
    CurrencyUnit = 180,
    CurrencySubUnit = 181,
    KpLeftParen = 182,
    KpRightParen = 183,
    KpLeftBrace = 184,
    KpRightBrace = 185,
    KpTab = 186,
    KpBackspace = 187,
    KpA = 188,
    KpB = 189,
    KpC = 190,
    KpD = 191,
    KpE = 192,
    KpF = 193,
    KpXor = 194,
    KpPower = 195,
    KpPercent = 196,
    KpLess = 197,
    KpGreater = 198,
    KpAmpersand = 199,
    KpDblAmpersand = 200,
    KpVerticalBar = 201,
    KpDblVerticalBar = 202,
    KpColon = 203,
    KpHash = 204,
    KpSpace = 205,
    KpAt = 206,
    KpExclam = 207,
    KpMemStore = 208,
    KpMemRecall = 209,
    KpMemClear = 210,
    KpMemAdd = 211,
    KpMemSubtract = 212,
    KpMemMultiply = 213,
    KpMemDivide = 214,
    KpPlusMinus = 215,
    KpClear = 216,
    KpClearEntry = 217,
    KpBinary = 218,
    KpOctal = 219,
    KpDecimal = 220,
    KpHexadecimal = 221,
    LCtrl = 224,
    LShift = 225,
    LAlt = 226,
    LGui = 227,
    RCtrl = 228,
    RShift = 229,
    RAlt = 230,
    RGui = 231,
    Mode = 257,
    AudioNext = 258,
    AudioPrev = 259,
    AudioStop = 260,
    AudioPlay = 261,
    AudioMute = 262,
    MediaSelect = 263,
    Www = 264,
    Mail = 265,
    Calculator = 266,
    Computer = 267,
    AcSearch = 268,
    AcHome = 269,
    AcBack = 270,
    AcForward = 271,
    AcStop = 272,
    AcRefresh = 273,
    AcBookmarks = 274,
    BrightnessDown = 275,
    BrightnessUp = 276,
    DisplaySwitch = 277,
    KbdIllumToggle = 278,
    KbdIllumDown = 279,
    KbdIllumUp = 280,
    Eject = 281,
    Sleep = 282,
    App1 = 283,
    App2 = 284,
    Num = 512,
}
impl From<KbCode> for u16 {
    fn from(value: KbCode) -> Self {
        value as u16
    }
}
impl From<KbCode> for i32 {
    fn from(value: KbCode) -> Self {
        value as u16 as i32
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use KbCode::*;

    #[test]
    fn test_kb_code() {
        assert_eq!(A as u16, 4);
        assert_eq!(Num as u16, 512);
    }

    #[test]
    fn test_get_set() {
        let mut kbr = KbReg::new();

        let mut i: u16 = 0;
        for byte in 0..KB_REG_SIZE {
            for bit in 0..<u8>::BITS {
                let cmp_val: u16 = (1 << (bit + 1)) - 1;

                assert_eq!(kbr.key(i), false);
                kbr.key_down(i);
                assert_eq!(kbr.key(i), true);
                kbr.key_up(i);
                assert_eq!(kbr.key(i), false);
                kbr.key_down(i);
                assert_eq!(cmp_val, kbr.bytes[byte] as u16);

                i += 1;
            }
        }
    }

    #[test]
    fn test_press_codes() {
        let mut kbc = KbReg::new();
        assert!(!kbc.key(A));
        kbc.key_down(A);
        assert!(kbc.key(A));
        kbc.key_down(Z);
        assert!(kbc.key(A));
        assert!(kbc.key(Z));
        kbc.key_up(A);
        assert!(!kbc.key(A));
    }
}
