# Instruction Set

This is the list of MFS-16 CPU instructions. Each opcode is 16 bits + the length of the immediate value (if any).

Instructions do not affect CPU flags unless otherwise specified. Any flags omitted by an instruction's list of affected flags are unaffected by the instruction.

"iff" is short for "if and only if".

## Legend

- **rn**: 16-bit register _n_. (A, B, C, D, E, H, L).

- **brn**: 32-bit big register _n_. (BC, DE, HL).

- **vrn**: 8-bit virtual register _n_. (A1, A0, B1, B0, C1, C0, D1, D0, E1, E0, H1, H0, L1, L0).

- **imm16**: The 16-bit immediate value after this instruction.

- **imm32**: The 32-bit immediate value after this instruction.

- **imm8**: The 8-bit immediate value after this instruction.

- **u4**: 4-bit unsigned integer constant (0x0 to 0xF).

- **SP**: The stack pointer.

- **Z**: The Zero flag.

- **C**: The Carry flag.

- **O**: The Overflow flag.

- **P**: The Parity flag.

- **N**: The Negative flag.

Consider this example on reading the notation. instruction `LD ra, rb` with opcode `0x01ab` means that any combination of 16-bit registers can be entered. `LD A, B` has opcode `0x0101`, while `LD L, C` has opcode `0x0162`.

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
  Cycles: 4

- **LD SP, bra:** Load bra into SP.  
  Opcode: 0x01Ba  
  Cycles: 2

- **LD bra, SP:** Load SP into bra.  
  Opcode: 0x01Ca  
  Cycles: 2

- **LD vra, vrb:** Load vrb into vra.  
  Opcode: 0x02ab  
  Cycles: 2

- **LD ra, imm16:** Load imm16 into ra.  
  Opcode: 0x030a  
  Cycles: 3

- **LD bra, imm32:** Load imm32 into bra.  
  Opcode: 0x031a  
  Cycles: 4

- **LD vra, imm8:** Load imm8 into vra.  
  Opcode: 0x032a  
  Cycles: 3

- **LD \[bra\], imm16:** Load imm16 into address bra.  
  Opcode: 0x033a  
  Cycles: 3

- **LD \[bra\], rb:** Load rb into address bra.  
  Opcode: 0x04ab  
  Cycles: 3

- **LD ra, \[brb\]:** Load the value at address brb into ra.  
  Opcode: 0x05ab  
  Cycles: 3

- **LDR ra, imm32:** Load the value at (HL + imm32 interpreted as a signed integer) into ra.  
  Opcode: 0x057a  
  Cycles: 5

- **LDI \[bra\], rb:** Load rb into address bra, then increase bra by two.  
  Opcode: 0x06ab  
  Cycles: 3

- **LDD \[bra\], rb:** Load rb into address bra, then decrease bra by two.  
  Opcode: 0x07ab  
  Cycles: 3

- **LDI ra, \[brb\]:** Load the value at brb into address ra, then increase brb by two.  
  Opcode: 0x08ab  
  Cycles: 3

- **LDD ra, \[brb\]:** Load the value at brb into address ra, then decrease brb by two.  
  Opcode: 0x09ab  
  Cycles: 3

- **LDI \[bra\], imm16:** Load imm16 into address bra, then increment bra by two.  
  Opcode: 0x097a  
  Cycles: 3

- **LDD \[bra\], imm16:** Load imm16 into address bra, then decrement bra by two.  
  Opcode: 0x098a  
  Cycles: 3

- **LD \[imm32\], ra:** Load ra into address imm32.  
  Opcode: 0x099a  
  Cycles: 4

- **LD ra, \[imm32\]:** Load the value at imm32 into ra.  
  Opcode: 0x09Aa  
  Cycles: 4

- **VLD \[bra\], brb:** VRAM load. Faster 32-bit version of LD \[bra\], rb for VRAM addresses only.  
  Opcode: 0x0Aab  
  Cycles: 2

- **VLDI \[bra\], brb:** VRAM load. Faster 32-bit version of LDI \[bra\], rb for VRAM addresses only.  
  Opcode: 0x0Bab  
  Cycles: 2

- **VLDD \[bra\], brb:** VRAM load. Faster 32-bit version of LDD \[bra\], rb for VRAM addresses only.  
  Opcode: 0x0Cab  
  Cycles: 2

- **VLD \[bra\], imm32:** VRAM load. Faster 32-bit version of VLD \[bra\], imm32 for VRAM addresses only.  
  Opcode: 0x0C3a  
  Cycles: 4

- **VLDI \[bra\], imm32:** VRAM load. Faster 32-bit version of VLDI \[bra\], imm32 for VRAM addresses only.  
  Opcode: 0x0C4a  
  Cycles: 4

- **VLDD \[bra\], imm32:** VRAM load. Faster 32-bit version of VLDI \[bra\], imm32 for VRAM addresses only.  
  Opcode: 0x0C5a  
  Cycles: 4

- **ADD ra, rb:** ra += rb.  
  Opcode: 0x10ab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff the result exceeds the available bits.
  - Set O iff the signed result is too large or small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed value.

- **ADD bra, brb:** bra += brb.  
  Opcode: 0x10(a+7)(b+7)  
  Cycles: 2  
  Flags: See ADD ra, rb.

- **ADD vra, vrb:** vra += vrb.  
  Opcode: 0x11ab  
  Cycles: 2  
  Flags: See ADD ra, rb.

- **ADC ra, rb:** ra += rb + C.  
  Opcode: 0x12ab  
  Cycles: 2  
  Flags: See ADD ra, rb.

- **ADC bra, brb:** bra += brb + C.  
  Opcode: 0x12(a+7)(b+7)  
  Cycles: 2  
  Flags: See ADD ra, rb.

- **ADC vra, vrb:** vra += vrb + C.  
  Opcode: 0x13ab  
  Cycles: 2  
  Flags: See ADD ra, rb.

- **SUB ra, rb:** ra -= rb.  
  Opcode: 0x14ab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff rb > ra for SUB instructions. Set C iff (rb + C) > ra for SBB instructions.
  - Set O iff the signed result is too large or too small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed value.

- **SUB bra, brb:** bra -= brb.  
  Opcode: 0x14(a+7)(b+7)  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **SUB vra, vrb:** vra -= vrb.  
  Opcode: 0x15ab  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **SBB ra, rb:** ra -= rb + C.  
  Opcode: 0x16ab  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **SBB bra, brb:** bra -= brb + C.  
  Opcode: 0x16(a+7)(b+7)  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **SBB vra, vrb:** vra -= vrb + C.  
  Opcode: 0x17ab  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **ADD ra, imm16:** ra += imm16.  
  Opcode: 0x180a  
  Cycles: 3  
  Flags: See ADD ra, rb.

- **ADC ra, imm16:** ra += imm16 + C.  
  Opcode: 0x181a  
  Cycles: 3  
  Flags: See ADD ra, rb.

- **ADD bra, imm32:** bra += imm32.  
  Opcode: 0x182a  
  Cycles: 4  
  Flags: See ADD ra, rb.

- **ADC bra, imm32:** bra += imm32 + C.  
  Opcode: 0x183a  
  Cycles: 4  
  Flags: See ADD ra, rb.

- **ADD vra, imm8:** vra += imm8.  
  Opcode: 0x184a  
  Cycles: 3  
  Flags: See ADD ra, rb.

- **ADC vra, imm8:** vra += imm8 + C.  
  Opcode: 0x185a  
  Cycles: 3  
  Flags: See ADD ra, rb.

- **SUB ra, imm16:** ra -= imm16.  
  Opcode: 0x186a  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **SBB ra, imm16:** ra -= imm16 + C.  
  Opcode: 0x187a  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **SUB bra, imm32:** bra -= imm32.  
  Opcode: 0x188a  
  Cycles: 4  
  Flags: See SUB ra, rb.

- **SBB bra, imm32:** bra -= imm32 + C.  
  Opcode: 0x189a  
  Cycles: 4  
  Flags: See SUB ra, rb.

- **SUB vra, imm8:** vra -= imm8.  
  Opcode: 0x18Aa  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **SBB vra, imm8:** vra -= imm8 + C.  
  Opcode: 0x18Ba  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **ADD ra, \[brb\]:** ra += (the value at brb).  
  Opcode: 0x19ab  
  Cycles: 3  
  Flags: See ADD ra, rb.

- **ADC ra, \[brb\]:** ra += (the value at brb) + C.  
  Opcode: 0x1Aab  
  Cycles: 3  
  Flags: See ADD ra, rb.

- **SUB ra, \[brb\]:** ra -= (the value at brb).  
  Opcode: 0x1Bab  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **SBB ra, \[brb\]:** ra -= (the value at brb) + C.  
  Opcode: 0x1Cab  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **TCP ra:** Two's complement ra. ra = -ra.  
  Opcode: 0x1D0a  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff the result != 0.
  - Set O iff the signed result is too large or too small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed value.

- **TCP bra:** Two's complement bra. bra = -bra.  
  Opcode: 0x1D1a  
  Cycles: 2  
  Flags: See TCP ra.

- **TCP vra:** Two's complement vra. vra = -vra.  
  Opcode: 0x1D2a  
  Cycles: 2  
  Flags: See TCP ra.

- **INC ra:** Increment ra. ra += 1.  
  Opcode: 0x1D3a  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set P iff the result is even.

- **INC bra:** Increment bra. bra += 1.  
  Opcode: 0x1D4a  
  Cycles: 2  
  Flags: See INC ra.

- **INC vra:** Increment vra. vra += 1.  
  Opcode: 0x1D5a  
  Cycles: 2  
  Flags: See INC ra.

- **DEC ra:** Decrement ra. ra -= 1.  
  Opcode: 0x1D6a  
  Cycles: 2  
  Flags: See INC ra.

- **DEC bra:** Decrement bra. bra -= 1.  
  Opcode: 0x1D7a  
  Cycles: 2  
  Flags: See INC ra.

- **DEC vra:** Decrement vra. vra -= 1.  
  Opcode: 0x1D8a  
  Cycles: 2  
  Flags: See INC ra.

- **PSS ra:** Set the CPU flags based on the value of ra.  
  Opcode: 0x1D9a  
  Cycles: 2  
  Flags:

  - Set Z iff the value == 0.
  - Set P iff the value is even.
  - Set N iff the value is negative when interpreted as a signed integer.

- **PSS bra:** Set the CPU flags based on the value of bra.  
  Opcode: 0x1DAa  
  Cycles: 2  
  Flags: See PSS ra.

- **PSS vra:** Set the CPU flags based on the value of vra.  
  Opcode: 0x1DBa  
  Cycles: 2  
  Flags: See PSS ra.

- **PSS imm16:** Set the CPU flags based on the value of imm16.  
  Opcode: 0x1DC0  
  Cycles: 3  
  Flags: See PSS ra.

- **PSS imm32:** Set the CPU flags based on the value of imm32.  
  Opcode: 0x1DC1  
  Cycles: 4  
  Flags: See PSS ra.

- **PSS imm8:** Set the CPU flags based on the value of imm8.  
  Opcode: 0x1DC2  
  Cycles: 3  
  Flags: See PSS ra.

- **AND ra, rb:** Bitwise AND. ra &= rb.  
  Opcode: 0x1Eab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Reset C.
  - Reset O.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **AND bra, brb:** Bitwise AND. bra &= brb.  
  Opcode: 0x1Fab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **AND vra, vrb:** Bitwise AND. vra &= vrb.  
  Opcode: 0x20ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **AND ra, \[brb\]:** Bitwise AND. ra &= (the value at brb).  
  Opcode: 0x21ab  
  Cycles: 3  
  Flags: See AND ra, rb.

- **OR ra, rb:** Bitwise OR. ra |= rb.  
  Opcode: 0x22ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **OR bra, brb:** Bitwise OR. bra |= brb.  
  Opcode: 0x23ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **OR vra, vrb:** Bitwise OR. vra |= vrb.  
  Opcode: 0x24ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **OR ra, \[brb\]:** Bitwise OR. ra |= (the value at brb).  
  Opcode: 0x25ab  
  Cycles: 3  
  Flags: See AND ra, rb.

- **XOR ra, rb:** Bitwise XOR. ra ^= rb.  
  Opcode: 0x26ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **XOR bra, brb:** Bitwise XOR. bra ^= brb.  
  Opcode: 0x27ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **XOR vra, vrb:** Bitwise XOR. vra ^= vrb.  
  Opcode: 0x28ab  
  Cycles: 2  
  Flags: See AND ra, rb.

- **XOR ra, \[brb\]:** Bitwise XOR. ra ^= (the value at brb).  
  Opcode: 0x29ab  
  Cycles: 3  
  Flags: See AND ra, rb.

- **AND ra, imm16:** Bitwise AND. ra &= imm16.  
  Opcode: 0x2A0a  
  Cycles: 3  
  Flags: See AND ra, rb.

- **AND bra, imm32:** Bitwise AND. bra &= imm32.  
  Opcode: 0x2A1a  
  Cycles: 4  
  Flags: See AND ra, rb.

- **AND vra, imm8:** Bitwise AND. vra &= imm8.  
  Opcode: 0x2A2a  
  Cycles: 3  
  Flags: See AND ra, rb.

- **OR ra, imm16:** Bitwise OR. ra |= imm16.  
  Opcode: 0x2A3a  
  Cycles: 3  
  Flags: See AND ra, rb.

- **OR bra, imm32:** Bitwise OR. bra |= imm32.  
  Opcode: 0x2A4a  
  Cycles: 4  
  Flags: See AND ra, rb.

- **OR vra, imm8:** Bitwise OR. vra |= imm8.  
  Opcode: 0x2A5a  
  Cycles: 3  
  Flags: See AND ra, rb.

- **XOR ra, imm16:** Bitwise XOR. ra ^= imm16.  
  Opcode: 0x2A6a  
  Cycles: 3  
  Flags: See AND ra, rb.

- **XOR bra, imm32:** Bitwise XOR. bra ^= imm32.  
  Opcode: 0x2A7a  
  Cycles: 4  
  Flags: See AND ra, rb.

- **XOR vra, imm8:** Bitwise XOR. vra ^= imm8.  
  Opcode: 0x2A8a  
  Cycles: 3  
  Flags: See AND ra, rb.

- **NOT ra:** Flip all bits of ra. ra = !ra.  
  Opcode: 0x2A9a  
  Cycles: 2  
  Flags: See AND ra, rb.

- **NOT bra:** Flip all bits of bra. bra = !bra.  
  Opcode: 0x2AAa  
  Cycles: 2  
  Flags: See AND ra, rb.

- **NOT vra:** Flip all bits of vra. vra = !vra.  
  Opcode: 0x2ABa  
  Cycles: 2  
  Flags: See AND ra, rb.

- **ASR ra, u4:** Arithmetic shift. Shift ra right u4 bits, preserving the most significant bit.  
  Opcode: 0x2Bab  
  Cycles: 2  
  Flags:

  - Set C iff the last bit shifted out == 1.
  - Reset O.

- **ASR bra, u4:** Arithmetic shift. Shift bra right u4 bits, preserving the most significant bit.  
  Opcode: 0x2Cab  
  Cycles: 2  
  Flags: See ASR ra, u4.

- **ASR vra, u4:** Arithmetic shift. Shift vra right u4 bits, preserving the most significant bit.  
  Opcode: 0x2Dab  
  Cycles: 2  
  Flags: See ASR ra, u4.

- **ASL ra, u4:** Arithmetic shift. Shift ra left u4 bits, shifting on zeroes.  
  Opcode: 0x2Eab  
  Cycles: 2  
  Flags:

  - Set C iff the last bit shifted out == 1.
  - Set O iff the result's most significant bit is different than the original operand's most significant bit.

- **ASL bra, u4:** Arithmetic shift. Shift bra left u4 bits, shifting on zeroes.  
  Opcode: 0x2Fab  
  Cycles: 2  
  Flags: See ASL ra, u4.

- **ASL vra, u4:** Arithmetic shift. Shift vra left u4 bits, shifting on zeroes.  
  Opcode: 0x30ab  
  Cycles: 2  
  Flags: See ASL ra, u4.

- **LSR ra, u4:** Logical shift. Shift ra right u4 bits, shifting on zeroes.  
  Opcode: 0x31ab  
  Cycles: 2  
  Flags:

  - Set C iff the last bit shifted out == 1.
  - Set O iff the most significant bit of the original operand == 1.

- **LSR bra, u4:** Logical shift. Shift bra right u4 bits, shifting on zeroes.  
  Opcode: 0x32ab  
  Cycles: 2  
  Flags: See LSR ra, u4.

- **LSR vra, u4:** Logical shift. Shift vra right u4 bits, shifting on zeroes.  
  Opcode: 0x33ab  
  Cycles: 2  
  Flags: See LSR ra, u4.

- **RTR ra, u4:** Rotate ra right u4 bits.  
  Opcode: 0x34ab  
  Cycles: 2  
  Flags:

  - Set C iff the last bit carried over to the other side == 1.
  - Set O iff the result's most significant bit is different than the original operand's most significant bit.

- **RTR bra, u4:** Rotate bra right u4 bits.  
  Opcode: 0x35ab  
  Cycles: 2  
  Flags: See RTR ra, u4.

- **RTR vra, u4:** Rotate vra right u4 bits.  
  Opcode: 0x36ab  
  Cycles: 2  
  Flags: See RTR ra, u4.

- **RTL ra, u4:** Rotate ra left u4 bits.  
  Opcode: 0x37ab  
  Cycles: 2  
  Flags: See RTR ra, u4.

- **RTL bra, u4:** Rotate bra left u4 bits.  
  Opcode: 0x38ab  
  Cycles: 2  
  Flags: See RTR ra, u4.

- **RTL vra, u4:** Rotate vra left u4 bits.  
  Opcode: 0x39ab  
  Cycles: 2  
  Flags: See RTR ra, u4.

- **RCR ra, u4:** Rotate ra right u4 bits through the carry flag.  
  Opcode: 0x3Aab  
  Cycles: 2  
  Flags:

  - C will be set iff the bit rotated into C == 1.
  - Set O iff the result's most significant bit is different than the original operand's most significant bit.

- **RCR bra, u4:** Rotate bra right u4 bits through the carry flag.  
  Opcode: 0x3Bab  
  Cycles: 2  
  Flags: See RCR ra, u4.

- **RCR vra, u4:** Rotate vra right u4 bits through the carry flag.  
  Opcode: 0x3Cab  
  Cycles: 2  
  Flags: See RCR ra, u4.

- **RCL ra, u4:** Rotate ra left u4 bits through the carry flag.  
  Opcode: 0x3Dab  
  Cycles: 2  
  Flags: See RCR ra, u4.

- **RCL bra, u4:** Rotate bra left u4 bits through the carry flag.  
  Opcode: 0x3Eab  
  Cycles: 2  
  Flags: See RCR ra, u4.

- **RCL vra, u4:** Rotate vra left u4 bits through the carry flag.  
  Opcode: 0x3Fab  
  Cycles: 2  
  Flags: See RCR ra, u4.

- **CMP ra, rb:** Set the flags according to the result of ra - rb, discarding the result.  
  Opcode: 0x40ab  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **CMP bra, brb:** Set the flags according to the result of bra - brb, discarding the result.  
  Opcode: 0x40(a+7)(b+7)  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **CMP vra, vrb:** Set the flags according to the result of vra - vrb, discarding the result.  
  Opcode: 0x41ab  
  Cycles: 2  
  Flags: See SUB ra, rb.

- **CMP ra, imm16:** Set the flags according to the result of ra - imm16, discarding the result.  
  Opcode: 0x420a  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **CMP bra, imm32:** Set the flags according to the result of bra - imm32, discarding the result.  
  Opcode: 0x421a  
  Cycles: 4  
  Flags: See SUB ra, rb.

- **CMP vra, imm8:** Set the flags according to the result of vra - imm8, discarding the result.  
  Opcode: 0x422a  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **CMP imm16, ra:** Set the flags according to the result of imm16 - ra, discarding the result.  
  Opcode: 0x423a  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **CMP imm32, bra:** Set the flags according to the result of imm32 - bra, discarding the result.  
  Opcode: 0x424a  
  Cycles: 4  
  Flags: See SUB ra, rb.

- **CMP imm8, vra:** Set the flags according to the result of imm8 - vra, discarding the result.  
  Opcode: 0x425a  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **CMP ra, \[brb\]:** Set the flags according to the result of ra - (the value at brb), discarding the result.  
  Opcode: 0x43ab  
  Cycles: 3  
  Flags: See SUB ra, rb.

- **CMP \[bra\], rb:** Set the flags according to the result of (the value at bra) - rb, discarding the result.  
  Opcode: 0x44ab  
  Cycles: 5  
  Flags: See SUB ra, rb.

- **BIT ra, u4:** Set the Zero flag according to bit u4 of ra.  
  Opcode: 0x45ab  
  Cycles: 2  
  Flags:

  - Set Z iff bit u4 of the given value == 0.

- **BIT \[bra\], u4:** Set the Zero flag according to bit u4 of the value at bra.  
  Opcode: 0x46ab  
  Cycles: 3  
  Flags: See BIT ra, u4.

- **STB ra, u4:** Set bit u4 of ra.  
  Opcode: 0x47ab  
  Cycles: 2

- **STB \[bra\], u4:** Set bit u4 of the value at bra.  
  Opcode: 0x48ab  
  Cycles: 3

- **RSB ra, u4:** Reset bit u4 of ra.  
  Opcode: 0x49ab  
  Cycles: 2

- **RSB \[bra\], u4:** Reset bit u4 of the value at bra.  
  Opcode: 0x4Aab  
  Cycles: 3

- **TGB ra, u4:** Toggle bit u4 of ra.  
  Opcode: 0x4Bab  
  Cycles: 2

- **TGB \[bra\], u4:** Toggle bit u4 of the value at bra.  
  Opcode: 0x4Cab  
  Cycles: 3

- **SWP ra:** Swap the high and low bytes of ra.  
  Opcode: 0x4D0a  
  Cycles: 2

- **SWP \[bra\]:** Swap the high and low bytes of the value at bra.  
  Opcode: 0x4D1a  
  Cycles: 3

- **SZF:** Set the Zero flag.  
  Opcode: 0x4D20  
  Cycles: 2  
  Flags:

  - Set Z.

- **RZF:** Reset the Zero flag.  
  Opcode: 0x4D21  
  Cycles: 2  
  Flags:

  - Reset Z.

- **TZF:** Toggle the Zero flag.  
  Opcode: 0x4D22  
  Cycles: 2  
  Flags:

  - Set Z iff Z is currently reset.

- **SCF:** Set the Carry flag.  
  Opcode: 0x4D23  
  Cycles: 2  
  Flags:

  - Set C.

- **RCF:** Reset the Carry flag.  
  Opcode: 0x4D24  
  Cycles: 2  
  Flags:

  - Reset C.

- **TCF:** Toggle the Carry flag.  
  Opcode: 0x4D25  
  Cycles: 2  
  Flags:

  - Set C iff C is currently reset.

- **SOF:** Set the Overflow flag.  
  Opcode: 0x4D26  
  Cycles: 2  
  Flags:

  - Set O.

- **ROF:** Reset the Overflow flag.  
  Opcode: 0x4D27  
  Cycles: 2  
  Flags:

  - Reset O.

- **TOF:** Toggle the Overflow flag.  
  Opcode: 0x4D28  
  Cycles: 2  
  Flags:

  - Set O iff O is currently reset.

- **SPF:** Set the Parity flag.  
  Opcode: 0x4D29  
  Cycles: 2  
  Flags:

  - Set P.

- **RPF:** Reset the Parity flag.  
  Opcode: 0x4D2A  
  Cycles: 2  
  Flags:

  - Reset P.

- **TPF:** Toggle the Parity flag.  
  Opcode: 0x4D2B  
  Cycles: 2  
  Flags:

  - Set P iff P is currently reset.

- **SNF:** Set the Negative flag.  
  Opcode: 0x4D2C  
  Cycles: 2  
  Flags:

  - Set N.

- **RNF:** Reset the Negative flag.  
  Opcode: 0x4D2D  
  Cycles: 2  
  Flags:

  - Reset N.

- **TNF:** Toggle the Negative flag.  
  Opcode: 0x4D2E  
  Cycles: 2  
  Flags:

  - Set N iff N is currently reset.

- **SAF:** Set all flags.  
  Opcode: 0x4D2F  
  Cycles: 2  
  Flags:

  - Set Z.
  - Set C.
  - Set O.
  - Set P.
  - Set N.

- **RAF:** Reset all flags.  
  Opcode: 0x4D30  
  Cycles: 2  
  Flags:

  - Reset Z.
  - Reset C.
  - Reset O.
  - Reset P.
  - Reset N.

- **MULU ra, rb:** Unsigned multiplication. ra \*= rb.  
  Opcode: 0x50ab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff the result exceeds the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **MULI ra, rb:** Signed multiplication. ra \*= rb.  
  Opcode: 0x51ab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set O iff the result is too large or small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **DIVU ra, rb:** Unsigned division. Does nothing if rb == 0. ra /= rb.  
  Opcode: 0x52ab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **DIVI ra, rb:** Signed division. Does nothing if rb == 0. ra /= rb.  
  Opcode: 0x53ab  
  Cycles: 2  
  Flags:

  - Set Z iff the result == 0.
  - Set O iff the numerator is the biggest negative number of its data type and the denominator is -1.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **MULU bra, brb:** Unsigned multiplication. bra \*= brb.  
  Opcode: 0x50(a+7)(b+7)  
  Cycles: 2  
  Flags: See MULU ra, rb.

- **MULI bra, brb:** Signed multiplication. bra \*= brb.  
  Opcode: 0x51(a+7)(b+7)  
  Cycles: 2  
  Flags: See MULI ra, rb.

- **DIVU bra, brb:** Unsigned division. bra /= brb.  
  Opcode: 0x52(a+7)(b+7)  
  Cycles: 2  
  Flags: See DIVU ra, rb.

- **DIVI bra, brb:** Signed division. bra /= brb.  
  Opcode: 0x53(a+7)(b+7)  
  Cycles: 2  
  Flags: See DIVI ra, rb.

- **MULU vra, vrb:** Unsigned multiplication. vra \*= vrb.  
  Opcode: 0x54ab  
  Cycles: 2  
  Flags: See MULU ra, rb.

- **MULI vra, vrb:** Signed multiplication. vra \*= vrb.  
  Opcode: 0x55ab  
  Cycles: 2  
  Flags: See MULI ra, rb.

- **DIVU vra, vrb:** Unsigned division. vra /= vrb.  
  Opcode: 0x56ab  
  Cycles: 2  
  Flags: See DIVU ra, rb.

- **DIVI vra, vrb:** Signed division. vra /= vrb.  
  Opcode: 0x57ab  
  Cycles: 2  
  Flags: See DIVI ra, rb.

- **MULU ra, \[brb\]:** Unsigned multiplication. ra \*= (the value at brb).  
  Opcode: 0x58ab  
  Cycles: 3  
  Flags: See MULU ra, rb.

- **MULI ra, \[brb\]:** Signed multiplication. ra \*= (the value at brb).  
  Opcode: 0x59ab  
  Cycles: 3  
  Flags: See MULI ra, rb.

- **DIVU ra, \[brb\]:** Unsigned division. ra \*= (the value at brb).  
  Opcode: 0x5Aab  
  Cycles: 3  
  Flags: See DIVU ra, rb.

- **DIVI ra, \[brb\]:** Signed division. ra /= (the value at brb).  
  Opcode: 0x5Bab  
  Cycles: 3  
  Flags: See DIVI ra, rb.

- **MULU ra, imm16:** Unsigned multiplication. ra \*= imm16.  
  Opcode: 0x5C0a  
  Cycles: 3  
  Flags: See MULU ra, rb.

- **MULI ra, imm16:** Signed multiplication. ra \*= imm16.  
  Opcode: 0x5C1a  
  Cycles: 3  
  Flags: See MULI ra, rb.

- **DIVU ra, imm16:** Unsigned division. ra /= imm16.  
  Opcode: 0x5C2a  
  Cycles: 3  
  Flags: See DIVU ra, rb.

- **DIVI ra, imm16:** Signed division. ra /= imm16.  
  Opcode: 0x5C3a  
  Cycles: 3  
  Flags: See DIVI ra, rb.

- **MULU bra, imm32:** Unsigned multiplication. bra \*= imm32.  
  Opcode: 0x5C4a  
  Cycles: 4  
  Flags: See MULU ra, rb.

- **MULI bra, imm32:** Signed multiplication. bra \*= imm32.  
  Opcode: 0x5C5a  
  Cycles: 4  
  Flags: See MULI ra, rb.

- **DIVU bra, imm32:** Unsigned division. bra /= imm32.  
  Opcode: 0x5C6a  
  Cycles: 4  
  Flags: See DIVU ra, rb.

- **DIVI bra, imm32:** Signed division. bra /= imm32.  
  Opcode: 0x5C7a  
  Cycles: 4  
  Flags: See DIVI ra, rb.

- **MULU vra, imm8:** Unsigned multiplication. vra \*= imm8.  
  Opcode: 0x5C8a  
  Cycles: 3  
  Flags: See MULU ra, rb.

- **MULI vra, imm8:** Signed multiplication. vra \*= imm8.  
  Opcode: 0x5C9a  
  Cycles: 3  
  Flags: See MULI ra, rb.

- **DIVU vra, imm8:** Unsigned division. vra /= imm8.  
  Opcode: 0x5CAa  
  Cycles: 3  
  Flags: See DIVU ra, rb.

- **DIVI vra, imm8:** Signed division. vra /= imm8.  
  Opcode: 0x5CBa  
  Cycles: 3  
  Flags: See DIVI ra, rb.

- **RAND ra:** Fill ra with a pseudorandom LFSR-based random number.  
  Opcode: 0x600a  
  Cycles: 2

- **RAND bra:** Fill bra with a pseudorandom LFSR-based random number.  
  Opcode: 0x601a  
  Cycles: 2

- **RAND vra:** Fill vra with a pseudorandom LFSR-based random number.  
  Opcode: 0x602a  
  Cycles: 2

- **JP imm32:** Jump to address imm32.  
  Opcode: 0x8000  
  Cycles: 4

- **JR imm32:** Relative jump imm32 (interpreted as a signed integer) bytes forwards/backwards.  
  Opcode: 0x8001  
  Cycles: 4

- **JPZ imm32:** Jump to address imm32 iff the Zero flag is set.  
  Opcode: 0x8002  
  Cycles: 5

- **JNZ imm32:** .Jump to address imm32 iff the Zero flag is reset.  
  Opcode: 0x8003  
  Cycles: 5

- **JPC imm32:** Jump to address imm32 iff the Carry flag is set.  
  Opcode: 0x8004  
  Cycles: 5

- **JNC imm32:** Jump to address imm32 iff the Carry flag is reset.  
  Opcode: 0x8005  
  Cycles: 5

- **JPO imm32:** Jump to address imm32 iff the Overflow flag is set.  
  Opcode: 0x8006  
  Cycles: 5

- **JNO imm32:** Jump to address imm32 iff the Overflow flag is reset.  
  Opcode: 0x8007  
  Cycles: 5

- **JPP imm32:** Jump to address imm32 iff the Parity flag is set.  
  Opcode: 0x8008  
  Cycles: 5

- **JNP imm32:** Jump to address imm32 iff the Parity flag is reset.  
  Opcode: 0x8009  
  Cycles: 5

- **JPN imm32:** Jump to address imm32 iff the Negative flag is set.  
  Opcode: 0x800A  
  Cycles: 5

- **JNN imm32:** Jump to address imm32 iff the Negative flag is reset.  
  Opcode: 0x800B  
  Cycles: 5

- **JP bra:** Jump to address bra.  
  Opcode: 0x801a  
  Cycles: 2

- **JR bra:** Relative jump bra (interpreted as a signed integer) bytes forwards/backwards.  
  Opcode: 0x802a  
  Cycles: 2

- **JPZ bra:** Jump to address bra iff the Zero flag is set.  
  Opcode: 0x803a  
  Cycles: 3

- **JNZ bra:** Jump to address bra iff the Zero flag is reset.  
  Opcode: 0x804a  
  Cycles: 3

- **JPC bra:** Jump to address bra iff the Carry flag is set.  
  Opcode: 0x805a  
  Cycles: 3

- **JNC bra:** Jump to address bra iff the Carry flag is reset.  
  Opcode: 0x806a  
  Cycles: 3

- **JPO bra:** Jump to address bra iff the Overflow flag is set.  
  Opcode: 0x807a  
  Cycles: 3

- **JNO bra:** Jump to address bra iff the Overflow flag is reset.  
  Opcode: 0x808a  
  Cycles: 3

- **JPP bra:** Jump to address bra iff the Parity flag is set.  
  Opcode: 0x809a  
  Cycles: 3

- **JNP bra:** Jump to address bra iff the Parity flag is reset.  
  Opcode: 0x80Aa  
  Cycles: 3

- **JPN bra:** Jump to address bra iff the Negative flag is set.  
  Opcode: 0x80Ba  
  Cycles: 3

- **JNN bra:** Jump to address bra iff the Negative flag is reset.  
  Opcode: 0x80Ca  
  Cycles: 3

- **CALL imm32:** Push the address of the instruction after CALL imm32 onto the stack, then jump to imm32.  
  Opcode: 0x8100  
  Cycles: 5

- **CLZ imm32:** Call imm32 if the Zero flag is set.  
  Opcode: 0x8101  
  Cycles: 6

- **CNZ imm32** Call imm32 if the Zero flag is reset.  
  Opcode: 0x8102  
  Cycles: 6

- **CLC imm32** Call imm32 if the Carry flag is set.  
  Opcode: 0x8103  
  Cycles: 6

- **CNC imm32** Call imm32 if the Carry flag is reset.  
  Opcode: 0x8104  
  Cycles: 6

- **CLO imm32** Call imm32 if the Overflow flag is set.  
  Opcode: 0x8105  
  Cycles: 6

- **CNO imm32** Call imm32 if the Overflow flag is reset.  
  Opcode: 0x8106  
  Cycles: 6

- **CLP imm32** Call imm32 if the Parity flag is set.  
  Opcode: 0x8107  
  Cycles: 6

- **CNP imm32** Call imm32 if the Parity flag is reset.  
  Opcode: 0x8108  
  Cycles: 6

- **CLN imm32** Call imm32 if the Negative flag is set.  
  Opcode: 0x8109  
  Cycles: 6

- **CNN imm32** Call imm32 if the Negative flag is reset.  
  Opcode: 0x810A  
  Cycles: 6

- **CALL bra:** Push the address of the instruction after CALL bra onto the stack, then jump to bra.  
  Opcode: 0x811a  
  Cycles: 3

- **RET:** Return from subroutine, setting the program counter to the value popped off the stack.  
  Opcode: 0x8113  
  Cycles: 2

- **RTZ:** Return if the Zero flag is set.  
  Opcode: 0x8114  
  Cycles: 3

- **RNZ:** Return if the Zero flag is reset.  
  Opcode: 0x8115  
  Cycles: 3

- **RTC:** Return if the Carry flag is set.  
  Opcode: 0x8116  
  Cycles: 3

- **RNC:** Return if the Carry flag is reset.  
  Opcode: 0x8117  
  Cycles: 3

- **RTO:** Return if the Overflow flag is set.  
  Opcode: 0x8118  
  Cycles: 3

- **RNO:** Return if the Overflow flag is reset.  
  Opcode: 0x8119  
  Cycles: 3

- **RTP:** Return if the Parity flag is set.  
  Opcode: 0x811A  
  Cycles: 3

- **RNP:** Return if the Parity flag is reset.  
  Opcode: 0x811B  
  Cycles: 3

- **RTN:** Return if the Negative flag is set.  
  Opcode: 0x811C  
  Cycles: 3

- **RNN:** Return if the Negative flag is reset.  
  Opcode: 0x811D  
  Cycles: 3

- **RETI:** Return from subroutine, then enable interrupts.  
  Opcode: 0x811E  
  Cycles: 3

- **CLZ bra:** Call bra if the Zero flag is set.  
  Opcode: 0x812a  
  Cycles: 4

- **CNZ bra:** Call bra if the Zero flag is reset.  
  Opcode: 0x813a  
  Cycles: 4

- **CLC bra:** Call bra if the Carry flag is set.  
  Opcode: 0x814a  
  Cycles: 4

- **CNC bra:** Call bra if the Carry flag is reset.  
  Opcode: 0x815a  
  Cycles: 4

- **CLO bra:** Call bra if the Overflow flag is set.  
  Opcode: 0x816a  
  Cycles: 4

- **CNO bra:** Call bra if the Overflow flag is reset.  
  Opcode: 0x817a  
  Cycles: 4

- **CLP bra:** Call bra if the Parity flag is set.  
  Opcode: 0x818a  
  Cycles: 4

- **CNP bra:** Call bra if the Parity flag is reset.  
  Opcode: 0x819a  
  Cycles: 4

- **CLN bra:** Call bra if the Negative flag is set.  
  Opcode: 0x81Aa  
  Cycles: 4

- **CNN bra:** Call bra if the Negative flag is reset.  
  Opcode: 0x81Ba  
  Cycles: 4

- **PUSH bra:** Push bra to the stack.  
  Opcode: 0x820a  
  Cycles: 2

- **POP bra:** Pop the top of the stack into bra.  
  Opcode: 0x820(a+3)  
  Cycles: 2

- **PEEK bra:** Load the top of the stack into bra without popping off the value.  
  Opcode: 0x820(a+6)  
  Cycles: 2

- **PUSH imm32:** Push imm32 to the stack.  
  Opcode: 0x8209  
  Cycles: 4

- **CLV:** Clear VRAM. Resets all bits in VRAM to 0.  
  Opcode: 0xFFFB  
  Cycles: 2

- **STOP:** Stop the CPU. Essentially a power-off message.  
  Opcode: 0xFFFC  
  Cycles: 3

- **EI:** Enable interrupts.  
  Opcode: 0xFFFD  
  Cycles: 2

- **DI:** Disable interrupts.  
  Opcode: 0xFFFE  
  Cycles: 2

- **HALT:** Halt the CPU, stopping CPU cycles until an external interrupt is received.  
  Opcode: 0xFFFF  
  Cycles: 2
