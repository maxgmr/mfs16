use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

const OUT_FILE_NAME: &str = "generated_opcodes.rs";

const NUM_REG16: u8 = 7;
const NUM_REG32: u8 = 3;
const NUM_REG8: u8 = 14;

static U4_REG16: [&str; NUM_REG16 as usize] = ["A", "B", "C", "D", "E", "H", "L"];
static U4_REG32: [&str; NUM_REG32 as usize] = ["BC", "DE", "HL"];
static U4_REG8: [&str; NUM_REG8 as usize] = [
    "A1", "A0", "B1", "B0", "C1", "C0", "D1", "D0", "E1", "E0", "H1", "H0", "L1", "L0",
];

fn main() -> io::Result<()> {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join(OUT_FILE_NAME);
    let mut file = File::create(out_path)?;

    writeln!(file, "use phf::phf_map;\n")?;

    writeln!(file, "use crate::{{Reg16::*, Reg32::*, Reg8::*}};\n")?;

    writeln!(
        file,
        "static OPCODE_MAP: phf::Map<u16, Instruction> = phf_map! {{"
    )?;

    macro_rules! gen_instr {
        // No args
        ($opcode:expr => $instr:literal) => {
            writeln!(file, "\t0x{:04X}_u16 => {},", $opcode, $instr)?;
            writeln!(file, "")?;
        };
        // ra
        ($opcode:expr => $instr:literal(RA)) => {
            for arg in 0..NUM_REG16 {
                let opcode = $opcode | (arg as u16);
                writeln!(
                    file,
                    "\t0x{:04X}_u16 => {}({}),",
                    opcode, $instr, U4_REG16[arg as usize]
                )?;
            }
            writeln!(file, "")?;
        };
        // bra
        ($opcode:expr => $instr:literal(BRA)) => {
            for arg in 0..NUM_REG32 {
                let opcode = $opcode | (arg as u16);
                writeln!(
                    file,
                    "\t0x{:04X}_u16 => {}({}),",
                    opcode, $instr, U4_REG32[arg as usize]
                )?;
            }
            writeln!(file, "")?;
        };
        // bra with offset
        ($opcode:expr => $instr:literal(BRA + $offset:expr)) => {
            for arg in $offset..(NUM_REG32 + $offset) {
                let opcode = $opcode | (arg as u16);
                writeln!(
                    file,
                    "\t0x{:04X}_u16 => {}({}),",
                    opcode,
                    $instr,
                    U4_REG32[(arg - $offset) as usize]
                )?;
            }
            writeln!(file, "")?;
        };
        // vra
        ($opcode:expr => $instr:literal(VRA)) => {
            for arg in 0..NUM_REG8 {
                let opcode = $opcode | (arg as u16);
                writeln!(
                    file,
                    "\t0x{:04X}_u16 => {}({}),",
                    opcode, $instr, U4_REG8[arg as usize]
                )?;
            }
            writeln!(file, "")?;
        };
        // ra, rb
        ($opcode:expr => $instr:literal(RA, RB)) => {
            for arg1 in 0..NUM_REG16 {
                for arg2 in 0..NUM_REG16 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG16[arg1 as usize], U4_REG16[arg2 as usize],
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // bra, brb
        ($opcode:expr => $instr:literal(BRA, BRB)) => {
            for arg1 in 0..NUM_REG32 {
                for arg2 in 0..NUM_REG32 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG32[arg1 as usize], U4_REG32[arg2 as usize],
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // bra with offset, brb with offset
        ($opcode:expr => $instr:literal(BRA + $offset1:expr, BRB + $offset2:expr)) => {
            for arg1 in $offset1..(NUM_REG32 + $offset1) {
                for arg2 in $offset2..(NUM_REG32 + $offset2) {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode,
                        $instr,
                        U4_REG32[(arg1 - $offset1) as usize],
                        U4_REG32[(arg2 - $offset2) as usize],
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // vra, vrb
        ($opcode:expr => $instr:literal(VRA, VRB)) => {
            for arg1 in 0..NUM_REG8 {
                for arg2 in 0..NUM_REG8 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG8[arg1 as usize], U4_REG8[arg2 as usize],
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // bra, rb
        ($opcode:expr => $instr:literal(BRA, RB)) => {
            for arg1 in 0..NUM_REG32 {
                for arg2 in 0..NUM_REG16 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG32[arg1 as usize], U4_REG16[arg2 as usize],
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // ra, brb
        ($opcode:expr => $instr:literal(RA, BRB)) => {
            for arg1 in 0..NUM_REG16 {
                for arg2 in 0..NUM_REG32 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG16[arg1 as usize], U4_REG32[arg2 as usize],
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // ra, b
        ($opcode:expr => $instr:literal(RA, B)) => {
            for arg1 in 0..NUM_REG16 {
                for arg2 in 0..0x10 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG16[arg1 as usize], arg2,
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // bra, b
        ($opcode:expr => $instr:literal(BRA, B)) => {
            for arg1 in 0..NUM_REG32 {
                for arg2 in 0..0x10 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG32[arg1 as usize], arg2,
                    )?;
                }
            }
            writeln!(file, "")?;
        };
        // vra, b
        ($opcode:expr => $instr:literal(VRA, B)) => {
            for arg1 in 0..NUM_REG8 {
                for arg2 in 0..0x10 {
                    let opcode = $opcode | ((arg1 as u16) << 4) | (arg2 as u16);
                    writeln!(
                        file,
                        "\t0x{:04X}_u16 => {}({}, {}),",
                        opcode, $instr, U4_REG8[arg1 as usize], arg2,
                    )?;
                }
            }
            writeln!(file, "")?;
        };
    }

    gen_instr!(0x0000 => "Nop");
    gen_instr!(0x0100 => "LdRaRb"(RA, RB));
    gen_instr!(0x0100 => "LdBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x01A0 => "LdSpImm32");
    gen_instr!(0x01A1 => "LdImm32Sp");
    gen_instr!(0x01B0 => "LdSpBra"(BRA));
    gen_instr!(0x01C0 => "LdBraSp"(BRA));
    gen_instr!(0x0200 => "LdVraVrb"(VRA, VRB));
    gen_instr!(0x0300 => "LdRaImm16"(RA));
    gen_instr!(0x0310 => "LdBraImm32"(BRA));
    gen_instr!(0x0320 => "LdVraImm8"(VRA));
    gen_instr!(0x0330 => "LdBraImm16"(BRA));
    gen_instr!(0x0400 => "LdBraRb"(BRA, RB));
    gen_instr!(0x0500 => "LdRaBrb"(RA, BRB));
    gen_instr!(0x0570 => "LdrRaImm32"(RA));
    gen_instr!(0x0600 => "LdiBraRb"(BRA, RB));
    gen_instr!(0x0700 => "LddBraRb"(BRA, RB));
    gen_instr!(0x0800 => "LdiRaBrb"(RA, BRB));
    gen_instr!(0x0900 => "LddRaBrb"(RA, BRB));
    gen_instr!(0x0970 => "LdiBraImm16"(BRA));
    gen_instr!(0x0980 => "LddBraImm16"(BRA));
    gen_instr!(0x1000 => "AddRaRb"(RA, RB));
    gen_instr!(0x1000 => "AddBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x1100 => "AddVraVrb"(VRA, VRB));
    gen_instr!(0x1200 => "AdcRaRb"(RA, RB));
    gen_instr!(0x1200 => "AdcBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x1300 => "AdcVraVrb"(VRA, VRB));
    gen_instr!(0x1400 => "SubRaRb"(RA, RB));
    gen_instr!(0x1400 => "SubBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x1500 => "SubVraVrb"(VRA, VRB));
    gen_instr!(0x1600 => "SbbRaRb"(RA, RB));
    gen_instr!(0x1600 => "SbbBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x1700 => "SbbVraVrb"(VRA, VRB));
    gen_instr!(0x1800 => "AddRaImm16"(RA));
    gen_instr!(0x1810 => "AdcRaImm16"(RA));
    gen_instr!(0x1820 => "AddBraImm32"(BRA));
    gen_instr!(0x1830 => "AdcBraImm32"(BRA));
    gen_instr!(0x1840 => "AddVraImm8"(VRA));
    gen_instr!(0x1850 => "AdcVraImm8"(VRA));
    gen_instr!(0x1860 => "SubRaImm16"(RA));
    gen_instr!(0x1870 => "SbbRaImm16"(RA));
    gen_instr!(0x1880 => "SubBraImm32"(BRA));
    gen_instr!(0x1890 => "SbbBraImm32"(BRA));
    gen_instr!(0x18A0 => "SubVraImm8"(VRA));
    gen_instr!(0x18B0 => "SbbVraImm8"(VRA));
    gen_instr!(0x1900 => "AddRaBrb"(RA, BRB));
    gen_instr!(0x1A00 => "AdcRaBrb"(RA, BRB));
    gen_instr!(0x1B00 => "SubRaBrb"(RA, BRB));
    gen_instr!(0x1C00 => "SbbRaBrb"(RA, BRB));
    gen_instr!(0x1D00 => "TcpRa"(RA));
    gen_instr!(0x1D10 => "TcpBra"(BRA));
    gen_instr!(0x1D20 => "TcpVra"(VRA));
    gen_instr!(0x1D30 => "IncRa"(RA));
    gen_instr!(0x1D40 => "IncBra"(BRA));
    gen_instr!(0x1D50 => "IncVra"(VRA));
    gen_instr!(0x1D60 => "DecRa"(RA));
    gen_instr!(0x1D70 => "DecBra"(BRA));
    gen_instr!(0x1D80 => "DecVra"(VRA));
    gen_instr!(0x1D90 => "PssRa"(RA));
    gen_instr!(0x1DA0 => "PssBra"(BRA));
    gen_instr!(0x1DB0 => "PssVra"(VRA));
    gen_instr!(0x1DC0 => "PssImm16");
    gen_instr!(0x1DC1 => "PssImm32");
    gen_instr!(0x1DC2 => "PssImm8");
    gen_instr!(0x1E00 => "AndRaRb"(RA, RB));
    gen_instr!(0x1F00 => "AndBraBrb"(BRA, BRB));
    gen_instr!(0x2000 => "AndVraVrb"(VRA, VRB));
    gen_instr!(0x2100 => "AndRaBrb"(RA, BRB));
    gen_instr!(0x2200 => "OrRaRb"(RA, RB));
    gen_instr!(0x2300 => "OrBraBrb"(BRA, BRB));
    gen_instr!(0x2400 => "OrVraVrb"(VRA, VRB));
    gen_instr!(0x2500 => "OrRaBrb"(RA, BRB));
    gen_instr!(0x2600 => "XorRaRb"(RA, RB));
    gen_instr!(0x2700 => "XorBraBrb"(BRA, BRB));
    gen_instr!(0x2800 => "XorVraVrb"(VRA, VRB));
    gen_instr!(0x2900 => "XorRaBrb"(RA, BRB));
    gen_instr!(0x2A00 => "AndRaImm16"(RA));
    gen_instr!(0x2A10 => "AndBraImm32"(BRA));
    gen_instr!(0x2A20 => "AndVraImm8"(VRA));
    gen_instr!(0x2A30 => "OrRaImm16"(RA));
    gen_instr!(0x2A40 => "OrBraImm32"(BRA));
    gen_instr!(0x2A50 => "OrVraImm8"(VRA));
    gen_instr!(0x2A60 => "XorRaImm16"(RA));
    gen_instr!(0x2A70 => "XorBraImm32"(BRA));
    gen_instr!(0x2A80 => "XorVraImm8"(VRA));
    gen_instr!(0x2A90 => "NotRa"(RA));
    gen_instr!(0x2AA0 => "NotBra"(BRA));
    gen_instr!(0x2AB0 => "NotVra"(VRA));
    gen_instr!(0x2B00 => "AsrRaB"(RA,B));
    gen_instr!(0x2C00 => "AsrBraB"(BRA,B));
    gen_instr!(0x2D00 => "AsrVraB"(VRA,B));
    gen_instr!(0x2E00 => "AslRaB"(RA,B));
    gen_instr!(0x2F00 => "AslBraB"(BRA,B));
    gen_instr!(0x3000 => "AslVraB"(VRA,B));
    gen_instr!(0x3100 => "LsrRaB"(RA,B));
    gen_instr!(0x3200 => "LsrBraB"(BRA,B));
    gen_instr!(0x3300 => "LsrVraB"(VRA,B));
    gen_instr!(0x3400 => "RtrRaB"(RA,B));
    gen_instr!(0x3500 => "RtrBraB"(BRA,B));
    gen_instr!(0x3600 => "RtrVraB"(VRA,B));
    gen_instr!(0x3700 => "RtlRaB"(RA,B));
    gen_instr!(0x3800 => "RtlBraB"(BRA,B));
    gen_instr!(0x3900 => "RtlVraB"(VRA,B));
    gen_instr!(0x3A00 => "RcrRaB"(RA,B));
    gen_instr!(0x3B00 => "RcrBraB"(BRA,B));
    gen_instr!(0x3C00 => "RcrVraB"(VRA,B));
    gen_instr!(0x3D00 => "RclRaB"(RA,B));
    gen_instr!(0x3E00 => "RclBraB"(BRA,B));
    gen_instr!(0x3F00 => "RclVraB"(VRA,B));
    gen_instr!(0x4000 => "CmpRaRb"(RA,RB));
    gen_instr!(0x4000 => "CmpBraBrb"(BRA + NUM_REG16,BRB + NUM_REG16));
    gen_instr!(0x4100 => "CmpVraVrb"(VRA,VRB));
    gen_instr!(0x4200 => "CmpRaImm16"(RA));
    gen_instr!(0x4210 => "CmpBraImm32"(BRA));
    gen_instr!(0x4220 => "CmpVraImm8"(VRA));
    gen_instr!(0x4230 => "CmpImm16Ra"(RA));
    gen_instr!(0x4240 => "CmpImm32Bra"(BRA));
    gen_instr!(0x4250 => "CmpImm8Vra"(VRA));
    gen_instr!(0x4300 => "CmpRaBrb"(RA, BRB));
    gen_instr!(0x4400 => "CmpBraRb"(BRA, RB));
    gen_instr!(0x4500 => "BitRaB"(RA, B));
    gen_instr!(0x4600 => "BitBraB"(BRA, B));
    gen_instr!(0x4700 => "StbRaB"(RA, B));
    gen_instr!(0x4800 => "StbBraB"(BRA, B));
    gen_instr!(0x4900 => "RsbRaB"(RA, B));
    gen_instr!(0x4A00 => "RsbBraB"(BRA, B));
    gen_instr!(0x4B00 => "TgbRaB"(RA, B));
    gen_instr!(0x4C00 => "TgbBraB"(BRA, B));
    gen_instr!(0x4D00 => "SwpRa"(RA));
    gen_instr!(0x4D10 => "SwpBra"(BRA));
    gen_instr!(0x4D20 => "Szf");
    gen_instr!(0x4D21 => "Rzf");
    gen_instr!(0x4D22 => "Tzf");
    gen_instr!(0x4D23 => "Scf");
    gen_instr!(0x4D24 => "Rcf");
    gen_instr!(0x4D25 => "Tcf");
    gen_instr!(0x4D26 => "Sof");
    gen_instr!(0x4D27 => "Rof");
    gen_instr!(0x4D28 => "Tof");
    gen_instr!(0x4D29 => "Spf");
    gen_instr!(0x4D2A => "Rpf");
    gen_instr!(0x4D2B => "Tpf");
    gen_instr!(0x4D2C => "Snf");
    gen_instr!(0x4D2D => "Rnf");
    gen_instr!(0x4D2E => "Tnf");
    gen_instr!(0x4D2F => "Saf");
    gen_instr!(0x4D30 => "Raf");
    gen_instr!(0x5000 => "MuluRaRb"(RA, RB));
    gen_instr!(0x5100 => "MuliRaRb"(RA, RB));
    gen_instr!(0x5200 => "DivuRaRb"(RA, RB));
    gen_instr!(0x5300 => "DiviRaRb"(RA, RB));
    gen_instr!(0x5000 => "MuluBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x5100 => "MuliBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x5200 => "DivuBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x5300 => "DiviBraBrb"(BRA + NUM_REG16, BRB + NUM_REG16));
    gen_instr!(0x5400 => "MuluVraVrb"(VRA, VRB));
    gen_instr!(0x5500 => "MuliVraVrb"(VRA, VRB));
    gen_instr!(0x5600 => "DivuVraVrb"(VRA, VRB));
    gen_instr!(0x5700 => "DiviVraVrb"(VRA, VRB));
    gen_instr!(0x5800 => "MuluRaBrb"(RA, BRB));
    gen_instr!(0x5900 => "MuliRaBrb"(RA, BRB));
    gen_instr!(0x5A00 => "DivuRaBrb"(RA, BRB));
    gen_instr!(0x5B00 => "DiviRaBrb"(RA, BRB));
    gen_instr!(0x5C00 => "MuluRaImm16"(RA));
    gen_instr!(0x5C10 => "MuliRaImm16"(RA));
    gen_instr!(0x5C20 => "DivuRaImm16"(RA));
    gen_instr!(0x5C30 => "DiviRaImm16"(RA));
    gen_instr!(0x5C40 => "MuluBraImm32"(BRA));
    gen_instr!(0x5C50 => "MuliBraImm32"(BRA));
    gen_instr!(0x5C60 => "DivuBraImm32"(BRA));
    gen_instr!(0x5C70 => "DiviBraImm32"(BRA));
    gen_instr!(0x5C80 => "MuluVraImm8"(VRA));
    gen_instr!(0x5C90 => "MuliVraImm8"(VRA));
    gen_instr!(0x5CA0 => "DivuVraImm8"(VRA));
    gen_instr!(0x5CB0 => "DiviVraImm8"(VRA));
    gen_instr!(0x6000 => "RandRa"(RA));
    gen_instr!(0x6010 => "RandBra"(BRA));
    gen_instr!(0x6020 => "RandVra"(VRA));
    gen_instr!(0x8000 => "JpImm32");
    gen_instr!(0x8001 => "JrImm32");
    gen_instr!(0x8002 => "JpzImm32");
    gen_instr!(0x8003 => "JnzImm32");
    gen_instr!(0x8004 => "JpcImm32");
    gen_instr!(0x8005 => "JncImm32");
    gen_instr!(0x8006 => "JpoImm32");
    gen_instr!(0x8007 => "JnoImm32");
    gen_instr!(0x8008 => "JppImm32");
    gen_instr!(0x8009 => "JnpImm32");
    gen_instr!(0x800A => "JpnImm32");
    gen_instr!(0x800B => "JnnImm32");
    gen_instr!(0x8010 => "JpBra"(BRA));
    gen_instr!(0x8020 => "JrBra"(BRA));
    gen_instr!(0x8030 => "JpzBra"(BRA));
    gen_instr!(0x8040 => "JnzBra"(BRA));
    gen_instr!(0x8050 => "JpcBra"(BRA));
    gen_instr!(0x8060 => "JncBra"(BRA));
    gen_instr!(0x8070 => "JpoBra"(BRA));
    gen_instr!(0x8080 => "JnoBra"(BRA));
    gen_instr!(0x8090 => "JppBra"(BRA));
    gen_instr!(0x80A0 => "JnpBra"(BRA));
    gen_instr!(0x80B0 => "JpnBra"(BRA));
    gen_instr!(0x80C0 => "JnnBra"(BRA));
    gen_instr!(0x8100 => "CallImm32");
    gen_instr!(0x8101 => "ClzImm32");
    gen_instr!(0x8102 => "CnzImm32");
    gen_instr!(0x8103 => "ClcImm32");
    gen_instr!(0x8104 => "CncImm32");
    gen_instr!(0x8105 => "CloImm32");
    gen_instr!(0x8106 => "CnoImm32");
    gen_instr!(0x8107 => "ClpImm32");
    gen_instr!(0x8108 => "CnpImm32");
    gen_instr!(0x8109 => "ClnImm32");
    gen_instr!(0x810A => "CnnImm32");
    gen_instr!(0x8110 => "CallBra"(BRA));
    gen_instr!(0x8113 => "Ret");
    gen_instr!(0x8114 => "Rtz");
    gen_instr!(0x8115 => "Rnz");
    gen_instr!(0x8116 => "Rtc");
    gen_instr!(0x8117 => "Rnc");
    gen_instr!(0x8118 => "Rto");
    gen_instr!(0x8119 => "Rno");
    gen_instr!(0x811A => "Rtp");
    gen_instr!(0x811B => "Rnp");
    gen_instr!(0x811C => "Rtn");
    gen_instr!(0x811D => "Rnn");
    gen_instr!(0x811E => "Reti");
    gen_instr!(0x8120 => "ClzBra"(BRA));
    gen_instr!(0x8130 => "CnzBra"(BRA));
    gen_instr!(0x8140 => "ClcBra"(BRA));
    gen_instr!(0x8150 => "CncBra"(BRA));
    gen_instr!(0x8160 => "CloBra"(BRA));
    gen_instr!(0x8170 => "CnoBra"(BRA));
    gen_instr!(0x8180 => "ClpBra"(BRA));
    gen_instr!(0x8190 => "CnpBra"(BRA));
    gen_instr!(0x81A0 => "ClnBra"(BRA));
    gen_instr!(0x81B0 => "CnnBra"(BRA));
    gen_instr!(0x8200 => "PushBra"(BRA));
    gen_instr!(0x8200 => "PopBra"(BRA + NUM_REG32));
    gen_instr!(0x8200 => "PeekBra"(BRA + (NUM_REG32 * 2)));
    gen_instr!(0x8209 => "PushImm32");
    gen_instr!(0xFFFC => "Stop");
    gen_instr!(0xFFFD => "Ei");
    gen_instr!(0xFFFE => "Di");
    gen_instr!(0xFFFF => "Halt");

    writeln!(file, "}};")?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
