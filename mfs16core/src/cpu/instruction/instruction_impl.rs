use super::*;

impl Instruction {
    /// Get the [Instruction] from the given opcode.
    pub fn from_opcode(opcode: u16) -> Self {
        let nib_1 = (opcode >> 12) as u8;
        let nib_2 = ((opcode & 0x0F00) >> 8) as u8;
        let nib_3 = ((opcode & 0x00F0) >> 4) as u8;
        let nib_4 = (opcode & 0x000F) as u8;

        match (nib_1, nib_2, nib_3, nib_4) {
            (0x0, 0x0, _, _) => Nop,
            (0x0, 0x1, 0xA, 0x0) => LdSpImm32,
            (0x0, 0x1, 0xA, 0x1) => LdImm32Sp,
            (0x0, 0x1, 0xB, bra) => LdSpBra(Reg32::from_nib(bra)),
            (0x0, 0x1, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                LdRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x0, 0x1, bra, brb) => LdBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x0, 0x2, vra, vrb) => LdVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x0, 0x3, 0x0, ra) => LdRaImm16(Reg16::from_nib(ra)),
            (0x0, 0x3, 0x1, bra) => LdBraImm32(Reg32::from_nib(bra)),
            (0x0, 0x3, 0x2, vra) => LdVraImm8(Reg8::from_nib(vra)),
            (0x0, 0x3, 0x3, bra) => LdBraImm16(Reg32::from_nib(bra)),
            (0x0, 0x4, bra, rb) => LdBraRb(Reg32::from_nib(bra), Reg16::from_nib(rb)),
            (0x0, 0x5, ra, brb) => LdRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x0, 0x6, bra, rb) => LdiBraRb(Reg32::from_nib(bra), Reg16::from_nib(rb)),
            (0x0, 0x7, bra, rb) => LddBraRb(Reg32::from_nib(bra), Reg16::from_nib(rb)),
            (0x0, 0x8, ra, brb) => LdiRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x0, 0x9, ra, brb) => LddRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0x0, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                AddRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x0, bra, brb) => AddBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x1, vra, vrb) => AddVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x2, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                AdcRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x2, bra, brb) => AdcBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x3, vra, vrb) => AdcVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x4, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                SubRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x4, bra, brb) => SubBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x5, vra, vrb) => SubVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x6, ra, rb) if ra < NUM_REGS && rb < NUM_REGS => {
                SbbRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb))
            }
            (0x1, 0x6, bra, brb) => SbbBraBrb(
                Reg32::from_nib(bra - NUM_REGS),
                Reg32::from_nib(brb - NUM_REGS),
            ),
            (0x1, 0x7, vra, vrb) => SbbVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x1, 0x8, 0x0, ra) => AddRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x1, ra) => AdcRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x2, bra) => AddBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0x3, bra) => AdcBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0x4, vra) => AddVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x8, 0x5, vra) => AdcVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x8, 0x6, ra) => SubRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x7, ra) => SbbRaImm16(Reg16::from_nib(ra)),
            (0x1, 0x8, 0x8, bra) => SubBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0x9, bra) => SbbBraImm32(Reg32::from_nib(bra)),
            (0x1, 0x8, 0xA, vra) => SubVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x8, 0xB, vra) => SbbVraImm8(Reg8::from_nib(vra)),
            (0x1, 0x9, ra, brb) => AddRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xA, ra, brb) => AdcRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xB, ra, brb) => SubRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xC, ra, brb) => SbbRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x1, 0xD, 0x0, ra) => TcpRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0x1, bra) => TcpBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0x2, vra) => TcpVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0x3, ra) => IncRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0x4, bra) => IncBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0x5, vra) => IncVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0x6, ra) => DecRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0x7, bra) => DecBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0x8, vra) => DecVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0x9, ra) => PssRa(Reg16::from_nib(ra)),
            (0x1, 0xD, 0xA, bra) => PssBra(Reg32::from_nib(bra)),
            (0x1, 0xD, 0xB, vra) => PssVra(Reg8::from_nib(vra)),
            (0x1, 0xD, 0xC, 0x0) => PssImm16,
            (0x1, 0xD, 0xC, 0x1) => PssImm32,
            (0x1, 0xD, 0xC, 0x2) => PssImm8,
            (0x1, 0xE, ra, rb) => AndRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb)),
            (0x1, 0xF, bra, brb) => AndBraBrb(Reg32::from_nib(bra), Reg32::from_nib(brb)),
            (0x2, 0x0, vra, vrb) => AndVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x2, 0x1, ra, brb) => AndRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x2, 0x2, ra, rb) => OrRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb)),
            (0x2, 0x3, bra, brb) => OrBraBrb(Reg32::from_nib(bra), Reg32::from_nib(brb)),
            (0x2, 0x4, vra, vrb) => OrVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x2, 0x5, ra, brb) => OrRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x2, 0x6, ra, rb) => XorRaRb(Reg16::from_nib(ra), Reg16::from_nib(rb)),
            (0x2, 0x7, bra, brb) => XorBraBrb(Reg32::from_nib(bra), Reg32::from_nib(brb)),
            (0x2, 0x8, vra, vrb) => XorVraVrb(Reg8::from_nib(vra), Reg8::from_nib(vrb)),
            (0x2, 0x9, ra, brb) => XorRaBrb(Reg16::from_nib(ra), Reg32::from_nib(brb)),
            (0x2, 0xA, 0x0, ra) => AndRaImm16(Reg16::from_nib(ra)),
            (0x2, 0xA, 0x1, bra) => AndBraImm32(Reg32::from_nib(bra)),
            (0x2, 0xA, 0x2, vra) => AndVraImm8(Reg8::from_nib(vra)),
            (0x2, 0xA, 0x3, ra) => OrRaImm16(Reg16::from_nib(ra)),
            (0x2, 0xA, 0x4, bra) => OrBraImm32(Reg32::from_nib(bra)),
            (0x2, 0xA, 0x5, vra) => OrVraImm8(Reg8::from_nib(vra)),
            (0x2, 0xA, 0x6, ra) => XorRaImm16(Reg16::from_nib(ra)),
            (0x2, 0xA, 0x7, bra) => XorBraImm32(Reg32::from_nib(bra)),
            (0x2, 0xA, 0x8, vra) => XorVraImm8(Reg8::from_nib(vra)),
            (0x2, 0xA, 0x9, ra) => NotRa(Reg16::from_nib(ra)),
            (0x2, 0xA, 0xA, bra) => NotBra(Reg32::from_nib(bra)),
            (0x2, 0xA, 0xB, vra) => NotVra(Reg8::from_nib(vra)),
            (0x2, 0xB, ra, b) => AsrRaB(Reg16::from_nib(ra), b),
            (0x2, 0xC, bra, b) => AsrBraB(Reg32::from_nib(bra), b),
            (0x2, 0xD, vra, b) => AsrVraB(Reg8::from_nib(vra), b),
            (0x2, 0xE, ra, b) => AslRaB(Reg16::from_nib(ra), b),
            (0x2, 0xF, bra, b) => AslBraB(Reg32::from_nib(bra), b),
            (0x3, 0x0, vra, b) => AslVraB(Reg8::from_nib(vra), b),
            (0x3, 0x1, ra, b) => LsrRaB(Reg16::from_nib(ra), b),
            (0x3, 0x2, bra, b) => LsrBraB(Reg32::from_nib(bra), b),
            (0x3, 0x3, vra, b) => LsrVraB(Reg8::from_nib(vra), b),
            (0x3, 0x4, ra, b) => RtrRaB(Reg16::from_nib(ra), b),
            (0x3, 0x5, bra, b) => RtrBraB(Reg32::from_nib(bra), b),
            (0x3, 0x6, vra, b) => RtrVraB(Reg8::from_nib(vra), b),
            (0x3, 0x7, ra, b) => RtlRaB(Reg16::from_nib(ra), b),
            (0x3, 0x8, bra, b) => RtlBraB(Reg32::from_nib(bra), b),
            (0x3, 0x9, vra, b) => RtlVraB(Reg8::from_nib(vra), b),
            (0x3, 0xA, ra, b) => RcrRaB(Reg16::from_nib(ra), b),
            (0x3, 0xB, bra, b) => RcrBraB(Reg32::from_nib(bra), b),
            (0x3, 0xC, vra, b) => RcrVraB(Reg8::from_nib(vra), b),
            (0x3, 0xD, ra, b) => RclRaB(Reg16::from_nib(ra), b),
            (0x3, 0xE, bra, b) => RclBraB(Reg32::from_nib(bra), b),
            (0x3, 0xF, vra, b) => RclVraB(Reg8::from_nib(vra), b),
            _ => panic!("Opcode {:#04X} has no corresponding instruction.", opcode),
        }
    }

    /// Convert the [Instruction] into its opcode.
    pub fn into_opcode(&self) -> u16 {
        match self {
            Nop => 0x0000,
            LdRaRb(ra, rb) => opc_2arg(0x01_u16, *ra, *rb),
            LdBraBrb(bra, brb) => opc_2arg_off(0x01_u16, *bra, *brb, NUM_REGS as u16),
            LdSpImm32 => 0x01A0,
            LdImm32Sp => 0x01A1,
            LdSpBra(bra) => opc_1arg(0x01B_u16, *bra),
            LdVraVrb(vra, vrb) => opc_2arg(0x02_u16, *vra, *vrb),
            LdRaImm16(ra) => opc_1arg(0x030_u16, *ra),
            LdBraImm32(bra) => opc_1arg(0x031_u16, *bra),
            LdVraImm8(vra) => opc_1arg(0x032_u16, *vra),
            LdBraImm16(bra) => opc_1arg(0x033_u16, *bra),
            LdBraRb(bra, rb) => opc_2arg(0x04_u16, *bra, *rb),
            LdRaBrb(ra, brb) => opc_2arg(0x05_u16, *ra, *brb),
            LdiBraRb(bra, rb) => opc_2arg(0x06_u16, *bra, *rb),
            LddBraRb(bra, rb) => opc_2arg(0x07_u16, *bra, *rb),
            LdiRaBrb(ra, brb) => opc_2arg(0x08_u16, *ra, *brb),
            LddRaBrb(ra, brb) => opc_2arg(0x09_u16, *ra, *brb),
            AddRaRb(ra, rb) => opc_2arg(0x10_u16, *ra, *rb),
            AddBraBrb(bra, brb) => opc_2arg_off(0x10_u16, *bra, *brb, NUM_REGS as u16),
            AddVraVrb(vra, vrb) => opc_2arg(0x11_u16, *vra, *vrb),
            AdcRaRb(ra, rb) => opc_2arg(0x12_u16, *ra, *rb),
            AdcBraBrb(bra, brb) => opc_2arg_off(0x12_u16, *bra, *brb, NUM_REGS as u16),
            AdcVraVrb(vra, vrb) => opc_2arg(0x13_u16, *vra, *vrb),
            SubRaRb(ra, rb) => opc_2arg(0x14_u16, *ra, *rb),
            SubBraBrb(bra, brb) => opc_2arg_off(0x14_u16, *bra, *brb, NUM_REGS as u16),
            SubVraVrb(vra, vrb) => opc_2arg(0x15_u16, *vra, *vrb),
            SbbRaRb(ra, rb) => opc_2arg(0x16_u16, *ra, *rb),
            SbbBraBrb(bra, brb) => opc_2arg_off(0x16_u16, *bra, *brb, NUM_REGS as u16),
            SbbVraVrb(vra, vrb) => opc_2arg(0x17_u16, *vra, *vrb),
            AddRaImm16(ra) => opc_1arg(0x180_u16, *ra),
            AdcRaImm16(ra) => opc_1arg(0x181_u16, *ra),
            AddBraImm32(bra) => opc_1arg(0x182_u16, *bra),
            AdcBraImm32(bra) => opc_1arg(0x183_u16, *bra),
            AddVraImm8(vra) => opc_1arg(0x184_u16, *vra),
            AdcVraImm8(vra) => opc_1arg(0x185_u16, *vra),
            SubRaImm16(ra) => opc_1arg(0x186_u16, *ra),
            SbbRaImm16(ra) => opc_1arg(0x187_u16, *ra),
            SubBraImm32(bra) => opc_1arg(0x188_u16, *bra),
            SbbBraImm32(bra) => opc_1arg(0x189_u16, *bra),
            SubVraImm8(vra) => opc_1arg(0x18A_u16, *vra),
            SbbVraImm8(vra) => opc_1arg(0x18B_u16, *vra),
            AddRaBrb(ra, brb) => opc_2arg(0x19_u16, *ra, *brb),
            AdcRaBrb(ra, brb) => opc_2arg(0x1A_u16, *ra, *brb),
            SubRaBrb(ra, brb) => opc_2arg(0x1B_u16, *ra, *brb),
            SbbRaBrb(ra, brb) => opc_2arg(0x1C_u16, *ra, *brb),
            TcpRa(ra) => opc_1arg(0x1D0_u16, *ra),
            TcpBra(bra) => opc_1arg(0x1D1_u16, *bra),
            TcpVra(vra) => opc_1arg(0x1D2_u16, *vra),
            IncRa(ra) => opc_1arg(0x1D3_u16, *ra),
            IncBra(bra) => opc_1arg(0x1D4_u16, *bra),
            IncVra(vra) => opc_1arg(0x1D5_u16, *vra),
            DecRa(ra) => opc_1arg(0x1D6_u16, *ra),
            DecBra(bra) => opc_1arg(0x1D7_u16, *bra),
            DecVra(vra) => opc_1arg(0x1D8_u16, *vra),
            PssRa(ra) => opc_1arg(0x1D9_u16, *ra),
            PssBra(bra) => opc_1arg(0x1DA_u16, *bra),
            PssVra(vra) => opc_1arg(0x1DB_u16, *vra),
            PssImm16 => 0x1DC0,
            PssImm32 => 0x1DC1,
            PssImm8 => 0x1DC2,
            AndRaRb(ra, rb) => opc_2arg(0x1E_u16, *ra, *rb),
            AndBraBrb(bra, brb) => opc_2arg(0x1F_u16, *bra, *brb),
            AndVraVrb(vra, vrb) => opc_2arg(0x20_u16, *vra, *vrb),
            AndRaBrb(ra, brb) => opc_2arg(0x21_u16, *ra, *brb),
            OrRaRb(ra, rb) => opc_2arg(0x22_u16, *ra, *rb),
            OrBraBrb(bra, brb) => opc_2arg(0x23_u16, *bra, *brb),
            OrVraVrb(vra, vrb) => opc_2arg(0x24_u16, *vra, *vrb),
            OrRaBrb(ra, brb) => opc_2arg(0x25_u16, *ra, *brb),
            XorRaRb(ra, rb) => opc_2arg(0x26_u16, *ra, *rb),
            XorBraBrb(bra, brb) => opc_2arg(0x27_u16, *bra, *brb),
            XorVraVrb(vra, vrb) => opc_2arg(0x28_u16, *vra, *vrb),
            XorRaBrb(ra, brb) => opc_2arg(0x29_u16, *ra, *brb),
            AndRaImm16(ra) => opc_1arg(0x2A0_u16, *ra),
            AndBraImm32(bra) => opc_1arg(0x2A1_u16, *bra),
            AndVraImm8(vra) => opc_1arg(0x2A2_u16, *vra),
            OrRaImm16(ra) => opc_1arg(0x2A3_u16, *ra),
            OrBraImm32(bra) => opc_1arg(0x2A4_u16, *bra),
            OrVraImm8(vra) => opc_1arg(0x2A5_u16, *vra),
            XorRaImm16(ra) => opc_1arg(0x2A6_u16, *ra),
            XorBraImm32(bra) => opc_1arg(0x2A7_u16, *bra),
            XorVraImm8(vra) => opc_1arg(0x2A8_u16, *vra),
            NotRa(ra) => opc_1arg(0x2A9_u16, *ra),
            NotBra(bra) => opc_1arg(0x2AA_u16, *bra),
            NotVra(vra) => opc_1arg(0x2AB_u16, *vra),
            AsrRaB(ra, b) => opc_2arg(0x2B_u16, *ra, *b),
            AsrBraB(bra, b) => opc_2arg(0x2C_u16, *bra, *b),
            AsrVraB(vra, b) => opc_2arg(0x2D_u16, *vra, *b),
            AslRaB(ra, b) => opc_2arg(0x2E_u16, *ra, *b),
            AslBraB(bra, b) => opc_2arg(0x2F_u16, *bra, *b),
            AslVraB(vra, b) => opc_2arg(0x30_u16, *vra, *b),
            LsrRaB(ra, b) => opc_2arg(0x31_u16, *ra, *b),
            LsrBraB(bra, b) => opc_2arg(0x32_u16, *bra, *b),
            LsrVraB(vra, b) => opc_2arg(0x33_u16, *vra, *b),
            RtrRaB(ra, b) => opc_2arg(0x34_u16, *ra, *b),
            RtrBraB(bra, b) => opc_2arg(0x35_u16, *bra, *b),
            RtrVraB(vra, b) => opc_2arg(0x36_u16, *vra, *b),
            RtlRaB(ra, b) => opc_2arg(0x37_u16, *ra, *b),
            RtlBraB(bra, b) => opc_2arg(0x38_u16, *bra, *b),
            RtlVraB(vra, b) => opc_2arg(0x39_u16, *vra, *b),
            RcrRaB(ra, b) => opc_2arg(0x3A_u16, *ra, *b),
            RcrBraB(bra, b) => opc_2arg(0x3B_u16, *bra, *b),
            RcrVraB(vra, b) => opc_2arg(0x3C_u16, *vra, *b),
            RclRaB(ra, b) => opc_2arg(0x3D_u16, *ra, *b),
            RclBraB(bra, b) => opc_2arg(0x3E_u16, *bra, *b),
            RclVraB(vra, b) => opc_2arg(0x3F_u16, *vra, *b),
        }
    }

    /// Return the number of CPU steps this instruction takes to execute.
    pub fn num_steps(&self) -> u32 {
        match self {
            Nop => 2,
            LdRaRb(..) => 2,
            LdBraBrb(..) => 2,
            LdSpImm32 => 4,
            LdImm32Sp => 4,
            LdSpBra(..) => 2,
            LdVraVrb(..) => 2,
            LdRaImm16(..) => 3,
            LdBraImm32(..) => 4,
            LdVraImm8(..) => 3,
            LdBraImm16(..) => 3,
            LdBraRb(..) => 3,
            LdRaBrb(..) => 3,
            LdiBraRb(..) => 3,
            LddBraRb(..) => 3,
            LdiRaBrb(..) => 3,
            LddRaBrb(..) => 3,
            AddRaRb(..) => 2,
            AddVraVrb(..) => 2,
            AddBraBrb(..) => 2,
            AdcRaRb(..) => 2,
            AdcBraBrb(..) => 2,
            AdcVraVrb(..) => 2,
            SubRaRb(..) => 2,
            SubVraVrb(..) => 2,
            SubBraBrb(..) => 2,
            SbbRaRb(..) => 2,
            SbbBraBrb(..) => 2,
            SbbVraVrb(..) => 2,
            AddRaImm16(..) => 3,
            AdcRaImm16(..) => 3,
            AddBraImm32(..) => 4,
            AdcBraImm32(..) => 4,
            AddVraImm8(..) => 3,
            AdcVraImm8(..) => 3,
            SubRaImm16(..) => 3,
            SbbRaImm16(..) => 3,
            SubBraImm32(..) => 4,
            SbbBraImm32(..) => 4,
            SubVraImm8(..) => 3,
            SbbVraImm8(..) => 3,
            AddRaBrb(..) => 3,
            AdcRaBrb(..) => 3,
            SubRaBrb(..) => 3,
            SbbRaBrb(..) => 3,
            TcpRa(..) => 2,
            TcpBra(..) => 2,
            TcpVra(..) => 2,
            IncRa(..) => 2,
            IncBra(..) => 2,
            IncVra(..) => 2,
            DecRa(..) => 2,
            DecBra(..) => 2,
            DecVra(..) => 2,
            PssRa(..) => 2,
            PssBra(..) => 2,
            PssVra(..) => 2,
            PssImm16 => 3,
            PssImm32 => 4,
            PssImm8 => 3,
            AndRaRb(..) => 2,
            AndBraBrb(..) => 2,
            AndVraVrb(..) => 2,
            AndRaBrb(..) => 3,
            OrRaRb(..) => 2,
            OrBraBrb(..) => 2,
            OrVraVrb(..) => 2,
            OrRaBrb(..) => 3,
            XorRaRb(..) => 2,
            XorBraBrb(..) => 2,
            XorVraVrb(..) => 2,
            XorRaBrb(..) => 3,
            AndRaImm16(..) => 3,
            AndBraImm32(..) => 4,
            AndVraImm8(..) => 3,
            OrRaImm16(..) => 3,
            OrBraImm32(..) => 4,
            OrVraImm8(..) => 3,
            XorRaImm16(..) => 3,
            XorBraImm32(..) => 4,
            XorVraImm8(..) => 3,
            NotRa(..) => 2,
            NotBra(..) => 2,
            NotVra(..) => 2,
            AsrRaB(..) => 2,
            AsrBraB(..) => 2,
            AsrVraB(..) => 2,
            AslRaB(..) => 2,
            AslBraB(..) => 2,
            AslVraB(..) => 2,
            LsrRaB(..) => 2,
            LsrBraB(..) => 2,
            LsrVraB(..) => 2,
            RtrRaB(..) => 2,
            RtrBraB(..) => 2,
            RtrVraB(..) => 2,
            RtlRaB(..) => 2,
            RtlBraB(..) => 2,
            RtlVraB(..) => 2,
            RcrRaB(..) => 2,
            RcrBraB(..) => 2,
            RcrVraB(..) => 2,
            RclRaB(..) => 2,
            RclBraB(..) => 2,
            RclVraB(..) => 2,
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<10}",
            match self {
                Nop => String::from("NOP"),
                LdRaRb(ra, rb) => format!("LD {ra},{rb}"),
                LdBraBrb(bra, brb) => format!("LD {bra},{brb}"),
                LdSpImm32 => String::from("LD SP,imm32"),
                LdImm32Sp => String::from("LD [imm32],SP"),
                LdSpBra(bra) => format!("LD SP,{bra}"),
                LdVraVrb(vra, vrb) => format!("LD {vra},{vrb}"),
                LdRaImm16(ra) => format!("LD {ra},imm16"),
                LdBraImm32(bra) => format!("LD {bra},imm32"),
                LdVraImm8(vra) => format!("LD {vra},imm8"),
                LdBraImm16(bra) => format!("LD [{bra}],imm16"),
                LdBraRb(bra, rb) => format!("LD [{bra}],{rb}"),
                LdRaBrb(ra, brb) => format!("LD {ra},[{brb}]"),
                LdiBraRb(bra, rb) => format!("LDI [{bra}],{rb}"),
                LddBraRb(bra, rb) => format!("LDD [{bra}],{rb}"),
                LdiRaBrb(ra, brb) => format!("LDI {ra},[{brb}]"),
                LddRaBrb(ra, brb) => format!("LDD {ra},[{brb}]"),
                AddRaRb(ra, rb) => format!("ADD {ra},{rb}"),
                AddBraBrb(bra, brb) => format!("ADD {bra},{brb}"),
                AddVraVrb(vra, vrb) => format!("ADD {vra},{vrb}"),
                AdcRaRb(ra, rb) => format!("ADC {ra},{rb}"),
                AdcBraBrb(bra, brb) => format!("ADC {bra},{brb}"),
                AdcVraVrb(vra, vrb) => format!("ADC {vra},{vrb}"),
                SubRaRb(ra, rb) => format!("SUB {ra},{rb}"),
                SubBraBrb(bra, brb) => format!("SUB {bra},{brb}"),
                SubVraVrb(vra, vrb) => format!("SUB {vra},{vrb}"),
                SbbRaRb(ra, rb) => format!("SBB {ra},{rb}"),
                SbbBraBrb(bra, brb) => format!("SBB {bra},{brb}"),
                SbbVraVrb(vra, vrb) => format!("SBB {vra},{vrb}"),
                AddRaImm16(ra) => format!("ADD {ra},imm16"),
                AdcRaImm16(ra) => format!("ADC {ra},imm16"),
                AddBraImm32(bra) => format!("ADD {bra},imm32"),
                AdcBraImm32(bra) => format!("ADC {bra},imm32"),
                AddVraImm8(vra) => format!("ADD {vra},imm8"),
                AdcVraImm8(vra) => format!("ADC {vra},imm8"),
                SubRaImm16(ra) => format!("SUB {ra},imm16"),
                SbbRaImm16(ra) => format!("SBB {ra},imm16"),
                SubBraImm32(bra) => format!("SUB {bra},imm32"),
                SbbBraImm32(bra) => format!("SBB {bra},imm32"),
                SubVraImm8(vra) => format!("SUB {vra},imm8"),
                SbbVraImm8(vra) => format!("SBB {vra},imm8"),
                AddRaBrb(ra, brb) => format!("ADD {ra}[{brb}]"),
                AdcRaBrb(ra, brb) => format!("ADC {ra}[{brb}]"),
                SubRaBrb(ra, brb) => format!("SUB {ra}[{brb}]"),
                SbbRaBrb(ra, brb) => format!("SBB {ra}[{brb}]"),
                TcpRa(ra) => format!("TCP {ra}"),
                TcpBra(bra) => format!("TCP {bra}"),
                TcpVra(vra) => format!("TCP {vra}"),
                IncRa(ra) => format!("INC {ra}"),
                IncBra(bra) => format!("INC {bra}"),
                IncVra(vra) => format!("INC {vra}"),
                DecRa(ra) => format!("DEC {ra}"),
                DecBra(bra) => format!("DEC {bra}"),
                DecVra(vra) => format!("DEC {vra}"),
                PssRa(ra) => format!("PSS {ra}"),
                PssBra(bra) => format!("PSS {bra}"),
                PssVra(vra) => format!("PSS {vra}"),
                PssImm16 => String::from("PSS imm16"),
                PssImm32 => String::from("PSS imm32"),
                PssImm8 => String::from("PSS imm8"),
                AndRaRb(ra, rb) => format!("AND {ra},{rb}"),
                AndBraBrb(bra, brb) => format!("AND {bra},{brb}"),
                AndVraVrb(vra, vrb) => format!("AND {vra},{vrb}"),
                AndRaBrb(ra, brb) => format!("AND {ra},[{brb}]"),
                OrRaRb(ra, rb) => format!("OR {ra},{rb}"),
                OrBraBrb(bra, brb) => format!("OR {bra},{brb}"),
                OrVraVrb(vra, vrb) => format!("OR {vra},{vrb}"),
                OrRaBrb(ra, brb) => format!("OR {ra},[{brb}]"),
                XorRaRb(ra, rb) => format!("XOR {ra},{rb}"),
                XorBraBrb(bra, brb) => format!("XOR {bra},{brb}"),
                XorVraVrb(vra, vrb) => format!("XOR {vra},{vrb}"),
                XorRaBrb(ra, brb) => format!("XOR {ra},[{brb}]"),
                AndRaImm16(ra) => format!("AND {ra},imm16"),
                AndBraImm32(bra) => format!("AND {bra},imm32"),
                AndVraImm8(vra) => format!("AND {vra},imm8"),
                OrRaImm16(ra) => format!("OR {ra},imm16"),
                OrBraImm32(bra) => format!("OR {bra},imm32"),
                OrVraImm8(vra) => format!("OR {vra},imm8"),
                XorRaImm16(ra) => format!("XOR {ra},imm16"),
                XorBraImm32(bra) => format!("XOR {bra},imm32"),
                XorVraImm8(vra) => format!("XOR {vra},imm8"),
                NotRa(ra) => format!("NOT {ra}"),
                NotBra(bra) => format!("NOT {bra}"),
                NotVra(vra) => format!("NOT {vra}"),
                AsrRaB(ra, b) => format!("ASR {ra},{b}"),
                AsrBraB(bra, b) => format!("ASR {bra},{b}"),
                AsrVraB(vra, b) => format!("ASR {vra},{b}"),
                AslRaB(ra, b) => format!("ASL {ra},{b}"),
                AslBraB(bra, b) => format!("ASL {bra},{b}"),
                AslVraB(vra, b) => format!("ASL {vra},{b}"),
                LsrRaB(ra, b) => format!("LSR {ra},{b}"),
                LsrBraB(bra, b) => format!("LSR {bra},{b}"),
                LsrVraB(vra, b) => format!("LSR {vra},{b}"),
                RtrRaB(ra, b) => format!("RTR {ra},{b}"),
                RtrBraB(bra, b) => format!("RTR {bra},{b}"),
                RtrVraB(vra, b) => format!("RTR {vra},{b}"),
                RtlRaB(ra, b) => format!("RTL {ra},{b}"),
                RtlBraB(bra, b) => format!("RTL {bra},{b}"),
                RtlVraB(vra, b) => format!("RTL {vra},{b}"),
                RcrRaB(ra, b) => format!("RCR {ra},{b}"),
                RcrBraB(bra, b) => format!("RCR {bra},{b}"),
                RcrVraB(vra, b) => format!("RCR {vra},{b}"),
                RclRaB(ra, b) => format!("RCL {ra},{b}"),
                RclBraB(bra, b) => format!("RCL {bra},{b}"),
                RclVraB(vra, b) => format!("RCL {vra},{b}"),
            }
        )
    }
}