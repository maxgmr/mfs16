# Instruction Set

This is the list of MFS-16 CPU instructions. Each opcode is 16 bits + the length of the immediate value (if any).

## Legend

- **rn**: 16-bit register _n_. (A, B, C, D, E, H, L).

- **brn**: 32-bit big register _n_. (BC, DE, HL).

- **vrn**: 8-bit virtual register _n_. (A1, A0, B1, B0, C1, C0, D1, D0, E1, E0, H1, H0, L1, L0).

- **imm16**: The 16-bit immediate value after this instruction.

- **imm32**: The 32-bit immediate value after this instruction.

- **imm8**: The 8-bit immediate value after this instruction.

- **u4**: 4-bit unsigned integer constant (0x0 to 0xF).

- **SP**: The stack pointer.

## Instructions

- **NOP:** Do nothing.  
  Opcode: 0x0000  
  Cycles: 2

- **LD ra, rb:** Load rb into ra.  
  Opcode: 0x01ab  
  Cycles: 2

- **LD bra, brb:** Load brb into bra.  
  Opcode: 0x01(a+7)(b+7)  
  Cycles: 2

- **LD SP,imm32:** Load imm32 into SP.  
  Opcode: 0x01A0  
  Cycles: 4

- **LD \[imm32\], SP:** Load SP into address imm32.  
  Opcode: 0x01A1  
  Cycles:

- **LD SP, bra:** Load bra into SP.  
  Opcode: 0x01Ba  
  Cycles:

- **LD bra, SP:** Load SP into bra.  
  Opcode: 0x01Ca  
  Cycles:

- **LD vra, vrb:** Load vrb into vra.  
  Opcode: 0x02ab  
  Cycles:

- **LD ra, imm16:** Load imm16 into ra.  
  Opcode: 0x030a  
  Cycles:

- **LD bra, imm32:** Load imm32 into bra.  
  Opcode: 0x031a  
  Cycles:

- **LD vra, imm8:** Load imm8 into vra.  
  Opcode: 0x032a  
  Cycles:

- **LD \[bra\], imm16:** Load imm16 into address bra.  
  Opcode: 0x033a  
  Cycles:

- **LD ra, \[brb\]:** Load the value at address brb into ra.  
  Opcode: 0x05ab  
  Cycles:

- **LDR ra, imm32:** Load the value at (HL + imm32 interpreted as a signed integer) into ra.  
  Opcode: 0x057a  
  Cycles:

- **LDI \[bra\], rb:** Load rb into address bra, then increase bra by two.  
  Opcode: 0x06ab  
  Cycles:

- **LDD \[bra\], rb:** Load rb into address bra, then decrease bra by two.  
  Opcode: 0x07ab  
  Cycles:

- **LDI ra, \[brb\]:** Load the value at brb into address ra, then increase brb by two.  
  Opcode: 0x08ab  
  Cycles:

- **LDD ra, \[brb\]:** Load the value at brb into address ra, then decrease brb by two.  
  Opcode: 0x09ab  
  Cycles:

- **:** .  
  Opcode: 0x097a  
  Cycles:

- **:** .  
  Opcode: 0x098a  
  Cycles:

- **:** .  
  Opcode: 0x099a  
  Cycles:

- **:** .  
  Opcode: 0x09Aa  
  Cycles:

- **:** .  
  Opcode: 0x0Aab  
  Cycles:

- **:** .  
  Opcode: 0x0Bab  
  Cycles:

- **:** .  
  Opcode: 0x0Cab  
  Cycles:

- **:** .  
  Opcode: 0x0C3a  
  Cycles:

- **:** .  
  Opcode: 0x0C4a  
  Cycles:

- **:** .  
  Opcode: 0x0C5a  
  Cycles:

- **:** .  
  Opcode: 0x10ab  
  Cycles:

- **:** .  
  Opcode: 0x10(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x11ab  
  Cycles:

- **:** .  
  Opcode: 0x12ab  
  Cycles:

- **:** .  
  Opcode: 0x12(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x13ab  
  Cycles:

- **:** .  
  Opcode: 0x14ab  
  Cycles:

- **:** .  
  Opcode: 0x14(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x15ab  
  Cycles:

- **:** .  
  Opcode: 0x16ab  
  Cycles:

- **:** .  
  Opcode: 0x16(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x17ab  
  Cycles:

- **:** .  
  Opcode: 0x180a  
  Cycles:

- **:** .  
  Opcode: 0x181a  
  Cycles:

- **:** .  
  Opcode: 0x182a  
  Cycles:

- **:** .  
  Opcode: 0x183a  
  Cycles:

- **:** .  
  Opcode: 0x184a  
  Cycles:

- **:** .  
  Opcode: 0x185a  
  Cycles:

- **:** .  
  Opcode: 0x186a  
  Cycles:

- **:** .  
  Opcode: 0x187a  
  Cycles:

- **:** .  
  Opcode: 0x188a  
  Cycles:

- **:** .  
  Opcode: 0x189a  
  Cycles:

- **:** .  
  Opcode: 0x18Aa  
  Cycles:

- **:** .  
  Opcode: 0x18Ba  
  Cycles:

- **:** .  
  Opcode: 0x19ab  
  Cycles:

- **:** .  
  Opcode: 0x1Aab  
  Cycles:

- **:** .  
  Opcode: 0x1Bab  
  Cycles:

- **:** .  
  Opcode: 0x1Cab  
  Cycles:

- **:** .  
  Opcode: 0x1D0a  
  Cycles:

- **:** .  
  Opcode: 0x1D1a  
  Cycles:

- **:** .  
  Opcode: 0x1D2a  
  Cycles:

- **:** .  
  Opcode: 0x1D3a  
  Cycles:

- **:** .  
  Opcode: 0x1D4a  
  Cycles:

- **:** .  
  Opcode: 0x1D5a  
  Cycles:

- **:** .  
  Opcode: 0x1D6a  
  Cycles:

- **:** .  
  Opcode: 0x1D7a  
  Cycles:

- **:** .  
  Opcode: 0x1D8a  
  Cycles:

- **:** .  
  Opcode: 0x1D9a  
  Cycles:

- **:** .  
  Opcode: 0x1DAa  
  Cycles:

- **:** .  
  Opcode: 0x1DBa  
  Cycles:

- **:** .  
  Opcode: 0x1DC0  
  Cycles:

- **:** .  
  Opcode: 0x1DC1  
  Cycles:

- **:** .  
  Opcode: 0x1DC2  
  Cycles:

- **:** .  
  Opcode: 0x1Eab  
  Cycles:

- **:** .  
  Opcode: 0x1Fab  
  Cycles:

- **:** .  
  Opcode: 0x20ab  
  Cycles:

- **:** .  
  Opcode: 0x21ab  
  Cycles:

- **:** .  
  Opcode: 0x22ab  
  Cycles:

- **:** .  
  Opcode: 0x23ab  
  Cycles:

- **:** .  
  Opcode: 0x24ab  
  Cycles:

- **:** .  
  Opcode: 0x25ab  
  Cycles:

- **:** .  
  Opcode: 0x26ab  
  Cycles:

- **:** .  
  Opcode: 0x27ab  
  Cycles:

- **:** .  
  Opcode: 0x28ab  
  Cycles:

- **:** .  
  Opcode: 0x29ab  
  Cycles:

- **:** .  
  Opcode: 0x2A0a  
  Cycles:

- **:** .  
  Opcode: 0x2A1a  
  Cycles:

- **:** .  
  Opcode: 0x2A2a  
  Cycles:

- **:** .  
  Opcode: 0x2A3a  
  Cycles:

- **:** .  
  Opcode: 0x2A4a  
  Cycles:

- **:** .  
  Opcode: 0x2A5a  
  Cycles:

- **:** .  
  Opcode: 0x2A6a  
  Cycles:

- **:** .  
  Opcode: 0x2A7a  
  Cycles:

- **:** .  
  Opcode: 0x2A8a  
  Cycles:

- **:** .  
  Opcode: 0x2A9a  
  Cycles:

- **:** .  
  Opcode: 0x2AAa  
  Cycles:

- **:** .  
  Opcode: 0x2ABa  
  Cycles:

- **:** .  
  Opcode: 0x2Bab  
  Cycles:

- **:** .  
  Opcode: 0x2Cab  
  Cycles:

- **:** .  
  Opcode: 0x2Dab  
  Cycles:

- **:** .  
  Opcode: 0x2Eab  
  Cycles:

- **:** .  
  Opcode: 0x2Fab  
  Cycles:

- **:** .  
  Opcode: 0x30ab  
  Cycles:

- **:** .  
  Opcode: 0x31ab  
  Cycles:

- **:** .  
  Opcode: 0x32ab  
  Cycles:

- **:** .  
  Opcode: 0x33ab  
  Cycles:

- **:** .  
  Opcode: 0x34ab  
  Cycles:

- **:** .  
  Opcode: 0x35ab  
  Cycles:

- **:** .  
  Opcode: 0x36ab  
  Cycles:

- **:** .  
  Opcode: 0x37ab  
  Cycles:

- **:** .  
  Opcode: 0x38ab  
  Cycles:

- **:** .  
  Opcode: 0x39ab  
  Cycles:

- **:** .  
  Opcode: 0x3Aab  
  Cycles:

- **:** .  
  Opcode: 0x3Bab  
  Cycles:

- **:** .  
  Opcode: 0x3Cab  
  Cycles:

- **:** .  
  Opcode: 0x3Dab  
  Cycles:

- **:** .  
  Opcode: 0x3Eab  
  Cycles:

- **:** .  
  Opcode: 0x3Fab  
  Cycles:

- **:** .  
  Opcode: 0x40ab  
  Cycles:

- **:** .  
  Opcode: 0x40(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x41ab  
  Cycles:

- **:** .  
  Opcode: 0x420a  
  Cycles:

- **:** .  
  Opcode: 0x421a  
  Cycles:

- **:** .  
  Opcode: 0x422a  
  Cycles:

- **:** .  
  Opcode: 0x423a  
  Cycles:

- **:** .  
  Opcode: 0x424a  
  Cycles:

- **:** .  
  Opcode: 0x425a  
  Cycles:

- **:** .  
  Opcode: 0x43ab  
  Cycles:

- **:** .  
  Opcode: 0x44ab  
  Cycles:

- **:** .  
  Opcode: 0x45ab  
  Cycles:

- **:** .  
  Opcode: 0x46ab  
  Cycles:

- **:** .  
  Opcode: 0x47ab  
  Cycles:

- **:** .  
  Opcode: 0x48ab  
  Cycles:

- **:** .  
  Opcode: 0x49ab  
  Cycles:

- **:** .  
  Opcode: 0x4Aab  
  Cycles:

- **:** .  
  Opcode: 0x4Bab  
  Cycles:

- **:** .  
  Opcode: 0x4Cab  
  Cycles:

- **:** .  
  Opcode: 0x4D0a  
  Cycles:

- **:** .  
  Opcode: 0x4D1a  
  Cycles:

- **:** .  
  Opcode: 0x4D20  
  Cycles:

- **:** .  
  Opcode: 0x4D21  
  Cycles:

- **:** .  
  Opcode: 0x4D22  
  Cycles:

- **:** .  
  Opcode: 0x4D23  
  Cycles:

- **:** .  
  Opcode: 0x4D24  
  Cycles:

- **:** .  
  Opcode: 0x4D25  
  Cycles:

- **:** .  
  Opcode: 0x4D26  
  Cycles:

- **:** .  
  Opcode: 0x4D27  
  Cycles:

- **:** .  
  Opcode: 0x4D28  
  Cycles:

- **:** .  
  Opcode: 0x4D29  
  Cycles:

- **:** .  
  Opcode: 0x4D2A  
  Cycles:

- **:** .  
  Opcode: 0x4D2B  
  Cycles:

- **:** .  
  Opcode: 0x4D2C  
  Cycles:

- **:** .  
  Opcode: 0x4D2D  
  Cycles:

- **:** .  
  Opcode: 0x4D2E  
  Cycles:

- **:** .  
  Opcode: 0x4D2F  
  Cycles:

- **:** .  
  Opcode: 0x4D30  
  Cycles:

- **:** .  
  Opcode: 0x50ab  
  Cycles:

- **:** .  
  Opcode: 0x51ab  
  Cycles:

- **:** .  
  Opcode: 0x52ab  
  Cycles:

- **:** .  
  Opcode: 0x53ab  
  Cycles:

- **:** .  
  Opcode: 0x50(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x51(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x52(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x53(a+7)(b+7)  
  Cycles:

- **:** .  
  Opcode: 0x54ab  
  Cycles:

- **:** .  
  Opcode: 0x55ab  
  Cycles:

- **:** .  
  Opcode: 0x56ab  
  Cycles:

- **:** .  
  Opcode: 0x57ab  
  Cycles:

- **:** .  
  Opcode: 0x58ab  
  Cycles:

- **:** .  
  Opcode: 0x59ab  
  Cycles:

- **:** .  
  Opcode: 0x5Aab  
  Cycles:

- **:** .  
  Opcode: 0x5Bab  
  Cycles:

- **:** .  
  Opcode: 0x5C0a  
  Cycles:

- **:** .  
  Opcode: 0x5C1a  
  Cycles:

- **:** .  
  Opcode: 0x5C2a  
  Cycles:

- **:** .  
  Opcode: 0x5C3a  
  Cycles:

- **:** .  
  Opcode: 0x5C4a  
  Cycles:

- **:** .  
  Opcode: 0x5C5a  
  Cycles:

- **:** .  
  Opcode: 0x5C6a  
  Cycles:

- **:** .  
  Opcode: 0x5C7a  
  Cycles:

- **:** .  
  Opcode: 0x5C8a  
  Cycles:

- **:** .  
  Opcode: 0x5C9a  
  Cycles:

- **:** .  
  Opcode: 0x5CAa  
  Cycles:

- **:** .  
  Opcode: 0x5CBa  
  Cycles:

- **:** .  
  Opcode: 0x600a  
  Cycles:

- **:** .  
  Opcode: 0x601a  
  Cycles:

- **:** .  
  Opcode: 0x602a  
  Cycles:

- **:** .  
  Opcode: 0x8000  
  Cycles:

- **:** .  
  Opcode: 0x8001  
  Cycles:

- **:** .  
  Opcode: 0x8002  
  Cycles:

- **:** .  
  Opcode: 0x8003  
  Cycles:

- **:** .  
  Opcode: 0x8004  
  Cycles:

- **:** .  
  Opcode: 0x8005  
  Cycles:

- **:** .  
  Opcode: 0x8006  
  Cycles:

- **:** .  
  Opcode: 0x8007  
  Cycles:

- **:** .  
  Opcode: 0x8008  
  Cycles:

- **:** .  
  Opcode: 0x8009  
  Cycles:

- **:** .  
  Opcode: 0x800A  
  Cycles:

- **:** .  
  Opcode: 0x800B  
  Cycles:

- **:** .  
  Opcode: 0x801a  
  Cycles:

- **:** .  
  Opcode: 0x802a  
  Cycles:

- **:** .  
  Opcode: 0x803a  
  Cycles:

- **:** .  
  Opcode: 0x804a  
  Cycles:

- **:** .  
  Opcode: 0x805a  
  Cycles:

- **:** .  
  Opcode: 0x806a  
  Cycles:

- **:** .  
  Opcode: 0x807a  
  Cycles:

- **:** .  
  Opcode: 0x808a  
  Cycles:

- **:** .  
  Opcode: 0x809a  
  Cycles:

- **:** .  
  Opcode: 0x80Aa  
  Cycles:

- **:** .  
  Opcode: 0x80Ba  
  Cycles:

- **:** .  
  Opcode: 0x80Ca  
  Cycles:

- **:** .  
  Opcode: 0x8100  
  Cycles:

- **:** .  
  Opcode: 0x8101  
  Cycles:

- **:** .  
  Opcode: 0x8102  
  Cycles:

- **:** .  
  Opcode: 0x8103  
  Cycles:

- **:** .  
  Opcode: 0x8104  
  Cycles:

- **:** .  
  Opcode: 0x8105  
  Cycles:

- **:** .  
  Opcode: 0x8106  
  Cycles:

- **:** .  
  Opcode: 0x8107  
  Cycles:

- **:** .  
  Opcode: 0x8108  
  Cycles:

- **:** .  
  Opcode: 0x8109  
  Cycles:

- **:** .  
  Opcode: 0x810A  
  Cycles:

- **:** .  
  Opcode: 0x811a  
  Cycles:

- **:** .  
  Opcode: 0x8113  
  Cycles:

- **:** .  
  Opcode: 0x8114  
  Cycles:

- **:** .  
  Opcode: 0x8115  
  Cycles:

- **:** .  
  Opcode: 0x8116  
  Cycles:

- **:** .  
  Opcode: 0x8117  
  Cycles:

- **:** .  
  Opcode: 0x8118  
  Cycles:

- **:** .  
  Opcode: 0x8119  
  Cycles:

- **:** .  
  Opcode: 0x811A  
  Cycles:

- **:** .  
  Opcode: 0x811B  
  Cycles:

- **:** .  
  Opcode: 0x811C  
  Cycles:

- **:** .  
  Opcode: 0x811D  
  Cycles:

- **:** .  
  Opcode: 0x811E  
  Cycles:

- **:** .  
  Opcode: 0x812a  
  Cycles:

- **:** .  
  Opcode: 0x813a  
  Cycles:

- **:** .  
  Opcode: 0x814a  
  Cycles:

- **:** .  
  Opcode: 0x815a  
  Cycles:

- **:** .  
  Opcode: 0x816a  
  Cycles:

- **:** .  
  Opcode: 0x817a  
  Cycles:

- **:** .  
  Opcode: 0x818a  
  Cycles:

- **:** .  
  Opcode: 0x819a  
  Cycles:

- **:** .  
  Opcode: 0x81Aa  
  Cycles:

- **:** .  
  Opcode: 0x81Ba  
  Cycles:

- **:** .  
  Opcode: 0x820a  
  Cycles:

- **:** .  
  Opcode: 0x820(a+3)  
  Cycles:

- **:** .  
  Opcode: 0x820(a+6)  
  Cycles:

- **:** .  
  Opcode: 0x8209  
  Cycles:

- **:** .  
  Opcode: 0xFFFB  
  Cycles:

- **:** .  
  Opcode: 0xFFFC  
  Cycles:

- **:** .  
  Opcode: 0xFFFD  
  Cycles:

- **:** .  
  Opcode: 0xFFFE  
  Cycles:

- **:** .  
  Opcode: 0xFFFF  
  Cycles:
