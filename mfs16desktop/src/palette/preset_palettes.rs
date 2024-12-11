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
    BLACK:          0x00_00_00,
    RED:			0x80_00_00,
    GREEN:			0x00_80_00,
    YELLOW:			0x80_80_00,
    BLUE:			0x00_00_80,
    MAGENTA:		0x80_00_80,
    CYAN:			0x00_80_80,
    WHITE:			0xC0_C0_C0,
    BRIGHT_BLACK:	0x80_80_80,
    BRIGHT_RED:		0xFF_00_00,
    BRIGHT_GREEN:	0x00_FF_00,
    BRIGHT_YELLOW:	0xFF_FF_00,
    BRIGHT_BLUE:	0x00_00_FF,
    BRIGHT_MAGENTA: 0xFF_00_FF,
    BRIGHT_CYAN:	0x00_FF_FF,
    BRIGHT_WHITE:	0xFF_FF_FF,
}

make_preset! {
    GRUVBOX,
    BLACK:	        0x28_28_28,
    RED:		    0xCC_24_1D,
    GREEN:	        0x98_97_1D,
    YELLOW:	        0xD7_99_21,
    BLUE:	        0x45_85_88,
    MAGENTA:        0xB1_62_86,
    CYAN:	        0x68_9D_6A,
    WHITE:	        0xA8_99_84,
    BRIGHT_BLACK:   0x92_83_74,
    BRIGHT_RED:		0xFB_49_34,
    BRIGHT_GREEN:	0xB8_BB_26,
    BRIGHT_YELLOW:	0xFA_BD_2F,
    BRIGHT_BLUE:	0x83_A5_98,
    BRIGHT_MAGENTA:	0xD3_86_9B,
    BRIGHT_CYAN:	0x8E_C0_7C,
    BRIGHT_WHITE:	0xEB_DB_B2,
}
