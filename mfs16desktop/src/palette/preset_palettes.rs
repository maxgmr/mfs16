use hex_color::HexColor;

use super::HexPalette;

macro_rules! make_preset {
    {
        $name:ident,

        BLACK:			$c0:expr,
        RED:			$c1:expr,
        GREEN:			$c2:expr,
        YELLOW:			$c3:expr,
        BLUE:			$c4:expr,
        MAGENTA:		$c5:expr,
        CYAN:			$c6:expr,
        WHITE:			$c7:expr,
        BRIGHT_BLACK:	$c8:expr,
        BRIGHT_RED:		$c9:expr,
        BRIGHT_GREEN:	$cA:expr,
        BRIGHT_YELLOW:	$cB:expr,
        BRIGHT_BLUE:	$cC:expr,
        BRIGHT_MAGENTA:	$cD:expr,
        BRIGHT_CYAN:	$cE:expr,
        BRIGHT_WHITE:	$cF:expr,
    } => {
       pub const $name: HexPalette = HexPalette {
            black: HexColor::from_u24($c0),
            red: HexColor::from_u24($c1),
            green: HexColor::from_u24($c2),
            yellow: HexColor::from_u24($c3),
            blue: HexColor::from_u24($c4),
            magenta: HexColor::from_u24($c5),
            cyan: HexColor::from_u24($c6),
            white: HexColor::from_u24($c7),
            bright_black: HexColor::from_u24($c8),
            bright_red: HexColor::from_u24($c9),
            bright_green: HexColor::from_u24($cA),
            bright_yellow: HexColor::from_u24($cB),
            bright_blue: HexColor::from_u24($cC),
            bright_magenta: HexColor::from_u24($cD),
            bright_cyan: HexColor::from_u24($cE),
            bright_white: HexColor::from_u24($cF),
       };
    }
}

make_preset! {
    DEFAULT,
    BLACK:          0x000000,
    RED:			0x800000,
    GREEN:			0x008000,
    YELLOW:			0x808000,
    BLUE:			0x000080,
    MAGENTA:		0x800080,
    CYAN:			0x008080,
    WHITE:			0xC0C0C0,
    BRIGHT_BLACK:	0x808080,
    BRIGHT_RED:		0xFF0000,
    BRIGHT_GREEN:	0x00FF00,
    BRIGHT_YELLOW:	0xFFFF00,
    BRIGHT_BLUE:	0x0000FF,
    BRIGHT_MAGENTA: 0xFF00FF,
    BRIGHT_CYAN:	0x00FFFF,
    BRIGHT_WHITE:	0xFFFFFF,
}

make_preset! {
    GRUVBOX,
    BLACK:	        0x282828,
    RED:		    0xCC241D,
    GREEN:	        0x98971D,
    YELLOW:	        0xD79921,
    BLUE:	        0x458588,
    MAGENTA:        0xB16286,
    CYAN:	        0x689D6A,
    WHITE:	        0xA89984,
    BRIGHT_BLACK:   0x928374,
    BRIGHT_RED:		0xFB4934,
    BRIGHT_GREEN:	0xB8BB26,
    BRIGHT_YELLOW:	0xFABD2F,
    BRIGHT_BLUE:	0x83A598,
    BRIGHT_MAGENTA:	0xD3869B,
    BRIGHT_CYAN:	0x8EC07C,
    BRIGHT_WHITE:	0xEBDBB2,
}

make_preset! {
    GRUVBOX_LIGHT,
    BLACK:	        0xebdbb2,
    RED:		    0xCC241D,
    GREEN:	        0x98971D,
    YELLOW:	        0xD79921,
    BLUE:	        0x458588,
    MAGENTA:        0xB16286,
    CYAN:	        0x689D6A,
    WHITE:	        0x7C6F64,
    BRIGHT_BLACK:   0x928374,
    BRIGHT_RED:		0x9D0006,
    BRIGHT_GREEN:	0x79740E,
    BRIGHT_YELLOW:	0xB57614,
    BRIGHT_BLUE:	0x076678,
    BRIGHT_MAGENTA:	0x8F3F71,
    BRIGHT_CYAN:	0x427B58,
    BRIGHT_WHITE:	0x3C3836,
}
