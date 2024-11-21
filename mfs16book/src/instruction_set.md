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

- **LDI \[bra\], imm16:** Load imm16 into address bra, then increment bra by two.  
  Opcode: 0x097a  
  Cycles:

- **LDD \[bra\], imm16:** Load imm16 into address bra, then decrement bra by two.  
  Opcode: 0x098a  
  Cycles:

- **LD \[imm32\], ra:** Load ra into address imm32.  
  Opcode: 0x099a  
  Cycles:

- **LD ra, \[imm32\]:** Load the value at imm32 into ra.  
  Opcode: 0x09Aa  
  Cycles:

- **VLD \[bra\], brb:** VRAM load. Faster 32-bit version of LD \[bra\], rb for VRAM addresses only.  
  Opcode: 0x0Aab  
  Cycles:

- **VLDI \[bra\], brb:** VRAM load. Faster 32-bit version of LDI \[bra\], rb for VRAM addresses only.  
  Opcode: 0x0Bab  
  Cycles:

- **VLDD \[bra\], brb:** VRAM load. Faster 32-bit version of LDD \[bra\], rb for VRAM addresses only.  
  Opcode: 0x0Cab  
  Cycles:

- **VLD \[bra\], imm32:** VRAM load. Faster 32-bit version of VLD \[bra\], imm32 for VRAM addresses only.  
  Opcode: 0x0C3a  
  Cycles:

- **VLDI \[bra\], imm32:** VRAM load. Faster 32-bit version of VLDI \[bra\], imm32 for VRAM addresses only.  
  Opcode: 0x0C4a  
  Cycles:

- **VLDD \[bra\], imm32:** VRAM load. Faster 32-bit version of VLDI \[bra\], imm32 for VRAM addresses only.  
  Opcode: 0x0C5a  
  Cycles:

- **ADD ra, rb:** ra += rb.  
  Opcode: 0x10ab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff the result exceeds the available bits.
  - Set O iff the signed result is too large or small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed value.

- **ADD bra, brb:** bra += brb.  
  Opcode: 0x10(a+7)(b+7)  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADD vra, vrb:** vra += vrb.  
  Opcode: 0x11ab  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC ra, rb:** ra += rb + C.  
  Opcode: 0x12ab  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC bra, brb:** bra += brb + C.  
  Opcode: 0x12(a+7)(b+7)  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC vra, vrb:** vra += vrb + C.  
  Opcode: 0x13ab  
  Cycles:  
  Flags: See ADD ra, rb.

- **SUB ra, rb:** ra -= rb.  
  Opcode: 0x14ab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff rb > ra for SUB instructions. Set C iff (rb + C) > ra for SBB instructions.
  - Set O iff the signed result is too large or too small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed value.

- **SUB bra, brb:** bra -= brb.  
  Opcode: 0x14(a+7)(b+7)  
  Cycles:  
  Flags: See SUB ra, rb.

- **SUB vra, vrb:** vra -= vrb.  
  Opcode: 0x15ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB ra, rb:** ra -= rb + C.  
  Opcode: 0x16ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB bra, brb:** bra -= brb + C.  
  Opcode: 0x16(a+7)(b+7)  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB vra, vrb:** vra -= vrb + C.  
  Opcode: 0x17ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **ADD ra, imm16:** ra += imm16.  
  Opcode: 0x180a  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC ra, imm16:** ra += imm16 + C.  
  Opcode: 0x181a  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADD bra, imm32:** bra += imm32.  
  Opcode: 0x182a  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC bra, imm32:** bra += imm32 + C.  
  Opcode: 0x183a  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADD vra, imm8:** vra += imm8.  
  Opcode: 0x184a  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC vra, imm8:** vra += imm8 + C.  
  Opcode: 0x185a  
  Cycles:  
  Flags: See ADD ra, rb.

- **SUB ra, imm16:** ra -= imm16.  
  Opcode: 0x186a  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB ra, imm16:** ra -= imm16 + C.  
  Opcode: 0x187a  
  Cycles:  
  Flags: See SUB ra, rb.

- **SUB bra, imm32:** bra -= imm32.  
  Opcode: 0x188a  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB bra, imm32:** bra -= imm32 + C.  
  Opcode: 0x189a  
  Cycles:  
  Flags: See SUB ra, rb.

- **SUB vra, imm8:** vra -= imm8.  
  Opcode: 0x18Aa  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB vra, imm8:** vra -= imm8 + C.  
  Opcode: 0x18Ba  
  Cycles:  
  Flags: See SUB ra, rb.

- **ADD ra, \[brb\]:** ra += (the value at brb).  
  Opcode: 0x19ab  
  Cycles:  
  Flags: See ADD ra, rb.

- **ADC ra, \[brb\]:** ra += (the value at brb) + C.  
  Opcode: 0x1Aab  
  Cycles:  
  Flags: See ADD ra, rb.

- **SUB ra, \[brb\]:** ra -= (the value at brb).  
  Opcode: 0x1Bab  
  Cycles:  
  Flags: See SUB ra, rb.

- **SBB ra, \[brb\]:** ra -= (the value at brb) + C.  
  Opcode: 0x1Cab  
  Cycles:  
  Flags: See SUB ra, rb.

- **TCP ra:** Two's complement ra. ra = -ra.  
  Opcode: 0x1D0a  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff the result != 0.
  - Set O iff the signed result is too large or too small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed value.

- **TCP bra:** Two's complement bra. bra = -bra.  
  Opcode: 0x1D1a  
  Cycles:  
  Flags: See TCP ra.

- **TCP vra:** Two's complement vra. vra = -vra.  
  Opcode: 0x1D2a  
  Cycles:  
  Flags: See TCP ra.

- **INC ra:** Increment ra. ra += 1.  
  Opcode: 0x1D3a  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set P iff the result is even.

- **INC bra:** Increment bra. bra += 1.  
  Opcode: 0x1D4a  
  Cycles:  
  Flags: See INC ra.

- **INC vra:** Increment vra. vra += 1.  
  Opcode: 0x1D5a  
  Cycles:  
  Flags: See INC ra.

- **DEC ra:** Decrement ra. ra -= 1.  
  Opcode: 0x1D6a  
  Cycles:  
  Flags: See INC ra.

- **DEC bra:** Decrement bra. bra -= 1.  
  Opcode: 0x1D7a  
  Cycles:  
  Flags: See INC ra.

- **DEC vra:** Decrement vra. vra -= 1.  
  Opcode: 0x1D8a  
  Cycles:  
  Flags: See INC ra.

- **PSS ra:** Set the CPU flags based on the value of ra.  
  Opcode: 0x1D9a  
  Cycles:  
  Flags:

  - Set Z iff the value == 0.
  - Set P iff the value is even.
  - Set N iff the value is negative when interpreted as a signed integer.

- **PSS bra:** Set the CPU flags based on the value of bra.  
  Opcode: 0x1DAa  
  Cycles:  
  Flags: See PSS ra.

- **PSS vra:** Set the CPU flags based on the value of vra.  
  Opcode: 0x1DBa  
  Cycles:  
  Flags: See PSS ra.

- **PSS imm16:** Set the CPU flags based on the value of imm16.  
  Opcode: 0x1DC0  
  Cycles:  
  Flags: See PSS ra.

- **PSS imm32:** Set the CPU flags based on the value of imm32.  
  Opcode: 0x1DC1  
  Cycles:  
  Flags: See PSS ra.

- **PSS imm8:** Set the CPU flags based on the value of imm8.  
  Opcode: 0x1DC2  
  Cycles:  
  Flags: See PSS ra.

- **AND ra, rb:** Bitwise AND. ra &= rb.  
  Opcode: 0x1Eab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Reset C.
  - Reset O.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **AND bra, brb:** Bitwise AND. bra &= brb.  
  Opcode: 0x1Fab  
  Cycles:  
  Flags: See AND ra, rb.

- **AND vra, vrb:** Bitwise AND. vra &= vrb.  
  Opcode: 0x20ab  
  Cycles:  
  Flags: See AND ra, rb.

- **AND ra, \[brb\]:** Bitwise AND. ra &= (the value at brb).  
  Opcode: 0x21ab  
  Cycles:  
  Flags: See AND ra, rb.

- **OR ra, rb:** Bitwise OR. ra |= rb.  
  Opcode: 0x22ab  
  Cycles:  
  Flags: See AND ra, rb.

- **OR bra, brb:** Bitwise OR. bra |= brb.  
  Opcode: 0x23ab  
  Cycles:  
  Flags: See AND ra, rb.

- **OR vra, vrb:** Bitwise OR. vra |= vrb.  
  Opcode: 0x24ab  
  Cycles:  
  Flags: See AND ra, rb.

- **OR ra, \[brb\]:** Bitwise OR. ra |= (the value at brb).  
  Opcode: 0x25ab  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR ra, rb:** Bitwise XOR. ra ^= rb.  
  Opcode: 0x26ab  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR bra, brb:** Bitwise XOR. bra ^= brb.  
  Opcode: 0x27ab  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR vra, vrb:** Bitwise XOR. vra ^= vrb.  
  Opcode: 0x28ab  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR ra, \[brb\]:** Bitwise XOR. ra ^= (the value at brb).  
  Opcode: 0x29ab  
  Cycles:  
  Flags: See AND ra, rb.

- **AND ra, imm16:** Bitwise AND. ra &= imm16.  
  Opcode: 0x2A0a  
  Cycles:  
  Flags: See AND ra, rb.

- **AND bra, imm32:** Bitwise AND. bra &= imm32.  
  Opcode: 0x2A1a  
  Cycles:  
  Flags: See AND ra, rb.

- **AND vra, imm8:** Bitwise AND. vra &= imm8.  
  Opcode: 0x2A2a  
  Cycles:  
  Flags: See AND ra, rb.

- **OR ra, imm16:** Bitwise OR. ra |= imm16.  
  Opcode: 0x2A3a  
  Cycles:  
  Flags: See AND ra, rb.

- **OR bra, imm32:** Bitwise OR. bra |= imm32.  
  Opcode: 0x2A4a  
  Cycles:  
  Flags: See AND ra, rb.

- **OR vra, imm8:** Bitwise OR. vra |= imm8.  
  Opcode: 0x2A5a  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR ra, imm16:** Bitwise XOR. ra ^= imm16.  
  Opcode: 0x2A6a  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR bra, imm32:** Bitwise XOR. bra ^= imm32.  
  Opcode: 0x2A7a  
  Cycles:  
  Flags: See AND ra, rb.

- **XOR vra, imm8:** Bitwise XOR. vra ^= imm8.  
  Opcode: 0x2A8a  
  Cycles:  
  Flags: See AND ra, rb.

- **NOT ra:** Flip all bits of ra. ra = !ra.  
  Opcode: 0x2A9a  
  Cycles:  
  Flags: See AND ra, rb.

- **NOT bra:** Flip all bits of bra. bra = !bra.  
  Opcode: 0x2AAa  
  Cycles:  
  Flags: See AND ra, rb.

- **NOT vra:** Flip all bits of vra. vra = !vra.  
  Opcode: 0x2ABa  
  Cycles:  
  Flags: See AND ra, rb.

- **ASR ra, u4:** Arithmetic shift. Shift ra right u4 bits, preserving the most significant bit.  
  Opcode: 0x2Bab  
  Cycles:  
  Flags:

  - Set C iff the last bit shifted out == 1.
  - Reset O.

- **ASR bra, u4:** Arithmetic shift. Shift bra right u4 bits, preserving the most significant bit.  
  Opcode: 0x2Cab  
  Cycles:  
  Flags: See ASR ra, u4.

- **ASR vra, u4:** Arithmetic shift. Shift vra right u4 bits, preserving the most significant bit.  
  Opcode: 0x2Dab  
  Cycles:  
  Flags: See ASR ra, u4.

- **ASL ra, u4:** Arithmetic shift. Shift ra left u4 bits, shifting on zeroes.  
  Opcode: 0x2Eab  
  Cycles:  
  Flags:

  - Set C iff the last bit shifted out == 1.
  - Set O iff the result's most significant bit is different than the original operand's most significant bit.

- **ASL bra, u4:** Arithmetic shift. Shift bra left u4 bits, shifting on zeroes.  
  Opcode: 0x2Fab  
  Cycles:  
  Flags: See ASL ra, u4.

- **ASL vra, u4:** Arithmetic shift. Shift vra left u4 bits, shifting on zeroes.  
  Opcode: 0x30ab  
  Cycles:  
  Flags: See ASL ra, u4.

- **LSR ra, u4:** Logical shift. Shift ra right u4 bits, shifting on zeroes.  
  Opcode: 0x31ab  
  Cycles:  
  Flags:

  - Set C iff the last bit shifted out == 1.
  - Set O iff the most significant bit of the original operand == 1.

- **LSR bra, u4:** Logical shift. Shift bra right u4 bits, shifting on zeroes.  
  Opcode: 0x32ab  
  Cycles:  
  Flags: See LSR ra, u4.

- **LSR vra, u4:** Logical shift. Shift vra right u4 bits, shifting on zeroes.  
  Opcode: 0x33ab  
  Cycles:  
  Flags: See LSR ra, u4.

- **RTR ra, u4:** Rotate ra right u4 bits.  
  Opcode: 0x34ab  
  Cycles:  
  Flags:

  - Set C iff the last bit carried over to the other side == 1.
  - Set O iff the result's most significant bit is different than the original operand's most significant bit.

- **RTR bra, u4:** Rotate bra right u4 bits.  
  Opcode: 0x35ab  
  Cycles:  
  Flags: See RTR ra, u4.

- **RTR vra, u4:** Rotate vra right u4 bits.  
  Opcode: 0x36ab  
  Cycles:  
  Flags: See RTR ra, u4.

- **RTL ra, u4:** Rotate ra left u4 bits.  
  Opcode: 0x37ab  
  Cycles:  
  Flags: See RTR ra, u4.

- **RTL bra, u4:** Rotate bra left u4 bits.  
  Opcode: 0x38ab  
  Cycles:  
  Flags: See RTR ra, u4.

- **RTL vra, u4:** Rotate vra left u4 bits.  
  Opcode: 0x39ab  
  Cycles:  
  Flags: See RTR ra, u4.

- **RCR ra, u4:** Rotate ra right u4 bits through the carry flag.  
  Opcode: 0x3Aab  
  Cycles:  
  Flags:

  - C will be set iff the bit rotated into C == 1.
  - Set O iff the result's most significant bit is different than the original operand's most significant bit.

- **RCR bra, u4:** Rotate bra right u4 bits through the carry flag.  
  Opcode: 0x3Bab  
  Cycles:  
  Flags: See RCR ra, u4.

- **RCR vra, u4:** Rotate vra right u4 bits through the carry flag.  
  Opcode: 0x3Cab  
  Cycles:  
  Flags: See RCR ra, u4.

- **RCL ra, u4:** Rotate ra left u4 bits through the carry flag.  
  Opcode: 0x3Dab  
  Cycles:  
  Flags: See RCR ra, u4.

- **RCL bra, u4:** Rotate bra left u4 bits through the carry flag.  
  Opcode: 0x3Eab  
  Cycles:  
  Flags: See RCR ra, u4.

- **RCL vra, u4:** Rotate vra left u4 bits through the carry flag.  
  Opcode: 0x3Fab  
  Cycles:  
  Flags: See RCR ra, u4.

- **CMP ra, rb:** Set the flags according to the result of ra - rb, discarding the result.  
  Opcode: 0x40ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP bra, brb:** Set the flags according to the result of bra - brb, discarding the result.  
  Opcode: 0x40(a+7)(b+7)  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP vra, vrb:** Set the flags according to the result of vra - vrb, discarding the result.  
  Opcode: 0x41ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP ra, imm16:** Set the flags according to the result of ra - imm16, discarding the result.  
  Opcode: 0x420a  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP bra, imm32:** Set the flags according to the result of bra - imm32, discarding the result.  
  Opcode: 0x421a  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP vra, imm8:** Set the flags according to the result of vra - imm8, discarding the result.  
  Opcode: 0x422a  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP imm16, ra:** Set the flags according to the result of imm16 - ra, discarding the result.  
  Opcode: 0x423a  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP imm32, bra:** Set the flags according to the result of imm32 - bra, discarding the result.  
  Opcode: 0x424a  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP imm8, vra:** Set the flags according to the result of imm8 - vra, discarding the result.  
  Opcode: 0x425a  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP ra, \[brb\]:** Set the flags according to the result of ra - (the value at brb), discarding the result.  
  Opcode: 0x43ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **CMP \[bra\], rb:** Set the flags according to the result of (the value at bra) - rb, discarding the result.  
  Opcode: 0x44ab  
  Cycles:  
  Flags: See SUB ra, rb.

- **BIT ra, u4:** Set the Zero flag according to bit u4 of ra.  
  Opcode: 0x45ab  
  Cycles:  
  Flags:

  - Set Z iff bit u4 of the given value == 0.

- **BIT \[bra\], u4:** Set the Zero flag according to bit u4 of the value at bra.  
  Opcode: 0x46ab  
  Cycles:  
  Flags: See BIT ra, u4.

- **STB ra, u4:** Set bit u4 of ra.  
  Opcode: 0x47ab  
  Cycles:

- **STB \[bra\], u4:** Set bit u4 of the value at bra.  
  Opcode: 0x48ab  
  Cycles:

- **RSB ra, u4:** Reset bit u4 of ra.  
  Opcode: 0x49ab  
  Cycles:

- **RSB \[bra\], u4:** Reset bit u4 of the value at bra.  
  Opcode: 0x4Aab  
  Cycles:

- **TGB ra, u4:** Toggle bit u4 of ra.  
  Opcode: 0x4Bab  
  Cycles:

- **TGB \[bra\], u4:** Toggle bit u4 of the value at bra.  
  Opcode: 0x4Cab  
  Cycles:

- **SWP ra:** Swap the high and low bytes of ra.  
  Opcode: 0x4D0a  
  Cycles:

- **SWP \[bra\]:** Swap the high and low bytes of the value at bra.  
  Opcode: 0x4D1a  
  Cycles:

- **SZF:** Set the Zero flag.  
  Opcode: 0x4D20  
  Cycles:  
  Flags:

  - Set Z.

- **RZF:** Reset the Zero flag.  
  Opcode: 0x4D21  
  Cycles:  
  Flags:

  - Reset Z.

- **TZF:** Toggle the Zero flag.  
  Opcode: 0x4D22  
  Cycles:  
  Flags:

  - Set Z iff Z is currently reset.

- **SCF:** Set the Carry flag.  
  Opcode: 0x4D23  
  Cycles:  
  Flags:

  - Set C.

- **RCF:** Reset the Carry flag.  
  Opcode: 0x4D24  
  Cycles:  
  Flags:

  - Reset C.

- **TCF:** Toggle the Carry flag.  
  Opcode: 0x4D25  
  Cycles:  
  Flags:

  - Set C iff C is currently reset.

- **SOF:** Set the Overflow flag.  
  Opcode: 0x4D26  
  Cycles:  
  Flags:

  - Set O.

- **ROF:** Reset the Overflow flag.  
  Opcode: 0x4D27  
  Cycles:  
  Flags:

  - Reset O.

- **TOF:** Toggle the Overflow flag.  
  Opcode: 0x4D28  
  Cycles:  
  Flags:

  - Set O iff O is currently reset.

- **SPF:** Set the Parity flag.  
  Opcode: 0x4D29  
  Cycles:  
  Flags:

  - Set P.

- **RPF:** Reset the Parity flag.  
  Opcode: 0x4D2A  
  Cycles:  
  Flags:

  - Reset P.

- **TPF:** Toggle the Parity flag.  
  Opcode: 0x4D2B  
  Cycles:  
  Flags:

  - Set P iff P is currently reset.

- **SNF:** Set the Negative flag.  
  Opcode: 0x4D2C  
  Cycles:  
  Flags:

  - Set N.

- **RNF:** Reset the Negative flag.  
  Opcode: 0x4D2D  
  Cycles:  
  Flags:

  - Reset N.

- **TNF:** Toggle the Negative flag.  
  Opcode: 0x4D2E  
  Cycles:  
  Flags:

  - Set N iff N is currently reset.

- **SAF:** Set all flags.  
  Opcode: 0x4D2F  
  Cycles:  
  Flags:

  - Set Z.
  - Set C.
  - Set O.
  - Set P.
  - Set N.

- **RAF:** Reset all flags.  
  Opcode: 0x4D30  
  Cycles:  
  Flags:

  - Reset Z.
  - Reset C.
  - Reset O.
  - Reset P.
  - Reset N.

- **MULU ra, rb:** Unsigned multiplication. ra \*= rb.  
  Opcode: 0x50ab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set C iff the result exceeds the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **MULI ra, rb:** Signed multiplication. ra \*= rb.  
  Opcode: 0x51ab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set O iff the result is too large or small to fit in the available bits.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **DIVU ra, rb:** Unsigned division. Does nothing if rb == 0. ra /= rb.  
  Opcode: 0x52ab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **DIVI ra, rb:** Signed division. Does nothing if rb == 0. ra /= rb.  
  Opcode: 0x53ab  
  Cycles:  
  Flags:

  - Set Z iff the result == 0.
  - Set O iff the numerator is the biggest negative number of its data type and the denominator is -1.
  - Set P iff the result is even.
  - Set N iff the result is negative when interpreted as a signed integer.

- **MULU bra, brb:** Unsigned multiplication. bra \*= brb.  
  Opcode: 0x50(a+7)(b+7)  
  Cycles:  
  Flags: See MULU ra, rb.

- **MULI bra, brb:** Signed multiplication. bra \*= brb.  
  Opcode: 0x51(a+7)(b+7)  
  Cycles:  
  Flags: See MULI ra, rb.

- **DIVU bra, brb:** Unsigned division. bra /= brb.  
  Opcode: 0x52(a+7)(b+7)  
  Cycles:  
  Flags: See DIVU ra, rb.

- **DIVI bra, brb:** Signed division. bra /= brb.  
  Opcode: 0x53(a+7)(b+7)  
  Cycles:  
  Flags: See DIVI ra, rb.

- **MULU vra, vrb:** Unsigned multiplication. vra \*= vrb.  
  Opcode: 0x54ab  
  Cycles:  
  Flags: See MULU ra, rb.

- **MULI vra, vrb:** Signed multiplication. vra \*= vrb.  
  Opcode: 0x55ab  
  Cycles:  
  Flags: See MULI ra, rb.

- **DIVU vra, vrb:** Unsigned division. vra /= vrb.  
  Opcode: 0x56ab  
  Cycles:  
  Flags: See DIVU ra, rb.

- **DIVI vra, vrb:** Signed division. vra /= vrb.  
  Opcode: 0x57ab  
  Cycles:  
  Flags: See DIVI ra, rb.

- **MULU ra, \[brb\]:** Unsigned multiplication. ra \*= (the value at brb).  
  Opcode: 0x58ab  
  Cycles:  
  Flags: See MULU ra, rb.

- **MULI ra, \[brb\]:** Signed multiplication. ra \*= (the value at brb).  
  Opcode: 0x59ab  
  Cycles:  
  Flags: See MULI ra, rb.

- **DIVU ra, \[brb\]:** Unsigned division. ra \*= (the value at brb).  
  Opcode: 0x5Aab  
  Cycles:  
  Flags: See DIVU ra, rb.

- **DIVI ra, \[brb\]:** Signed division. ra /= (the value at brb).  
  Opcode: 0x5Bab  
  Cycles:  
  Flags: See DIVI ra, rb.

- **MULU ra, imm16:** Unsigned multiplication. ra \*= imm16.  
  Opcode: 0x5C0a  
  Cycles:  
  Flags: See MULU ra, rb.

- **MULI ra, imm16:** Signed multiplication. ra \*= imm16.  
  Opcode: 0x5C1a  
  Cycles:  
  Flags: See MULI ra, rb.

- **DIVU ra, imm16:** Unsigned division. ra /= imm16.  
  Opcode: 0x5C2a  
  Cycles:  
  Flags: See DIVU ra, rb.

- **DIVI ra, imm16:** Signed division. ra /= imm16.  
  Opcode: 0x5C3a  
  Cycles:  
  Flags: See DIVI ra, rb.

- **MULU bra, imm32:** Unsigned multiplication. bra \*= imm32.  
  Opcode: 0x5C4a  
  Cycles:  
  Flags: See MULU ra, rb.

- **MULI bra, imm32:** Signed multiplication. bra \*= imm32.  
  Opcode: 0x5C5a  
  Cycles:  
  Flags: See MULI ra, rb.

- **DIVU bra, imm32:** Unsigned division. bra /= imm32.  
  Opcode: 0x5C6a  
  Cycles:  
  Flags: See DIVU ra, rb.

- **DIVI bra, imm32:** Signed division. bra /= imm32.  
  Opcode: 0x5C7a  
  Cycles:  
  Flags: See DIVI ra, rb.

- **MULU vra, imm8:** Unsigned multiplication. vra \*= imm8.  
  Opcode: 0x5C8a  
  Cycles:  
  Flags: See MULU ra, rb.

- **MULI vra, imm8:** Signed multiplication. vra \*= imm8.  
  Opcode: 0x5C9a  
  Cycles:  
  Flags: See MULI ra, rb.

- **DIVU vra, imm8:** Unsigned division. vra /= imm8.  
  Opcode: 0x5CAa  
  Cycles:  
  Flags: See DIVU ra, rb.

- **DIVI vra, imm8:** Signed division. vra /= imm8.  
  Opcode: 0x5CBa  
  Cycles:  
  Flags: See DIVI ra, rb.

- **RAND ra:** Fill ra with a pseudorandom LFSR-based random number.  
  Opcode: 0x600a  
  Cycles:

- **RAND bra:** Fill bra with a pseudorandom LFSR-based random number.  
  Opcode: 0x601a  
  Cycles:

- **RAND vra:** Fill vra with a pseudorandom LFSR-based random number.  
  Opcode: 0x602a  
  Cycles:

- **JP imm32:** Jump to address imm32.  
  Opcode: 0x8000  
  Cycles:

- **JR imm32:** Relative jump imm32 (interpreted as a signed integer) bytes forwards/backwards.  
  Opcode: 0x8001  
  Cycles:

- **JPZ imm32:** Jump to address imm32 iff the Zero flag is set.  
  Opcode: 0x8002  
  Cycles:

- **JNZ:** .Jump to address imm32 iff the Zero flag is reset.  
  Opcode: 0x8003  
  Cycles:

- **JPC:** Jump to address imm32 iff the Carry flag is set.  
  Opcode: 0x8004  
  Cycles:

- **JNC:** Jump to address imm32 iff the Carry flag is reset.  
  Opcode: 0x8005  
  Cycles:

- **JPO:** Jump to address imm32 iff the Overflow flag is set.  
  Opcode: 0x8006  
  Cycles:

- **JNO:** Jump to address imm32 iff the Overflow flag is reset.  
  Opcode: 0x8007  
  Cycles:

- **JPP:** Jump to address imm32 iff the Parity flag is set.  
  Opcode: 0x8008  
  Cycles:

- **JNP:** Jump to address imm32 iff the Parity flag is reset.  
  Opcode: 0x8009  
  Cycles:

- **JPN:** Jump to address imm32 iff the Negative flag is set.  
  Opcode: 0x800A  
  Cycles:

- **JNN:** Jump to address imm32 iff the Negative flag is reset.  
  Opcode: 0x800B  
  Cycles:

- **JP bra:** Jump to address bra.  
  Opcode: 0x801a  
  Cycles:

- **JR bra:** Relative jump bra (interpreted as a signed integer) bytes forwards/backwards.  
  Opcode: 0x802a  
  Cycles:

- **JPZ bra:** Jump to address bra iff the Zero flag is set.  
  Opcode: 0x803a  
  Cycles:

- **JNZ bra:** Jump to address bra iff the Zero flag is reset.  
  Opcode: 0x804a  
  Cycles:

- **JPC bra:** Jump to address bra iff the Carry flag is set.  
  Opcode: 0x805a  
  Cycles:

- **JNC bra:** Jump to address bra iff the Carry flag is reset.  
  Opcode: 0x806a  
  Cycles:

- **JPO bra:** Jump to address bra iff the Overflow flag is set.  
  Opcode: 0x807a  
  Cycles:

- **JNO bra:** Jump to address bra iff the Overflow flag is reset.  
  Opcode: 0x808a  
  Cycles:

- **JPP bra:** Jump to address bra iff the Parity flag is set.  
  Opcode: 0x809a  
  Cycles:

- **JNP bra:** Jump to address bra iff the Parity flag is reset.  
  Opcode: 0x80Aa  
  Cycles:

- **JPN bra:** Jump to address bra iff the Negative flag is set.  
  Opcode: 0x80Ba  
  Cycles:

- **JNN bra:** Jump to address bra iff the Negative flag is reset.  
  Opcode: 0x80Ca  
  Cycles:

- **CALL imm32:** Push the address of the instruction after CALL imm32 onto the stack, then jump to imm32.  
  Opcode: 0x8100  
  Cycles:

- **CLZ imm32:** Call imm32 if the Zero flag is set.  
  Opcode: 0x8101  
  Cycles:

- **CNZ imm32** Call imm32 if the Zero flag is reset.  
  Opcode: 0x8102  
  Cycles:

- **CLC imm32** Call imm32 if the Carry flag is set.  
  Opcode: 0x8103  
  Cycles:

- **CNC imm32** Call imm32 if the Carry flag is reset.  
  Opcode: 0x8104  
  Cycles:

- **CLO imm32** Call imm32 if the Overflow flag is set.  
  Opcode: 0x8105  
  Cycles:

- **CNO imm32** Call imm32 if the Overflow flag is reset.  
  Opcode: 0x8106  
  Cycles:

- **CLP imm32** Call imm32 if the Parity flag is set.  
  Opcode: 0x8107  
  Cycles:

- **CNP imm32** Call imm32 if the Parity flag is reset.  
  Opcode: 0x8108  
  Cycles:

- **CLN imm32** Call imm32 if the Negative flag is set.  
  Opcode: 0x8109  
  Cycles:

- **CNN imm32** Call imm32 if the Negative flag is reset.  
  Opcode: 0x810A  
  Cycles:

- **CALL bra:** Push the address of the instruction after CALL bra onto the stack, then jump to bra.  
  Opcode: 0x811a  
  Cycles:

- **RET:** Return from subroutine, setting the program counter to the value popped off the stack.  
  Opcode: 0x8113  
  Cycles:

- **RTZ:** Return if the Zero flag is set.  
  Opcode: 0x8114  
  Cycles:

- **RNZ:** Return if the Zero flag is reset.  
  Opcode: 0x8115  
  Cycles:

- **RTC:** Return if the Carry flag is set.  
  Opcode: 0x8116  
  Cycles:

- **RNC:** Return if the Carry flag is reset.  
  Opcode: 0x8117  
  Cycles:

- **RTO:** Return if the Overflow flag is set.  
  Opcode: 0x8118  
  Cycles:

- **RNO:** Return if the Overflow flag is reset.  
  Opcode: 0x8119  
  Cycles:

- **RTP:** Return if the Parity flag is set.  
  Opcode: 0x811A  
  Cycles:

- **RNP:** Return if the Parity flag is reset.  
  Opcode: 0x811B  
  Cycles:

- **RTN:** Return if the Negative flag is set.  
  Opcode: 0x811C  
  Cycles:

- **RNN:** Return if the Negative flag is reset.  
  Opcode: 0x811D  
  Cycles:

- **RETI:** Return from subroutine, then enable interrupts.  
  Opcode: 0x811E  
  Cycles:

- **CLZ bra:** Call bra if the Zero flag is set.  
  Opcode: 0x812a  
  Cycles:

- **CNZ bra:** Call bra if the Zero flag is reset.  
  Opcode: 0x813a  
  Cycles:

- **CLC bra:** Call bra if the Carry flag is set.  
  Opcode: 0x814a  
  Cycles:

- **CNC bra:** Call bra if the Carry flag is reset.  
  Opcode: 0x815a  
  Cycles:

- **CLO bra:** Call bra if the Overflow flag is set.  
  Opcode: 0x816a  
  Cycles:

- **CNO bra:** Call bra if the Overflow flag is reset.  
  Opcode: 0x817a  
  Cycles:

- **CLP bra:** Call bra if the Parity flag is set.  
  Opcode: 0x818a  
  Cycles:

- **CNP bra:** Call bra if the Parity flag is reset.  
  Opcode: 0x819a  
  Cycles:

- **CLN bra:** Call bra if the Negative flag is set.  
  Opcode: 0x81Aa  
  Cycles:

- **CNN bra:** Call bra if the Negative flag is reset.  
  Opcode: 0x81Ba  
  Cycles:

- **PUSH bra:** Push bra to the stack.  
  Opcode: 0x820a  
  Cycles:

- **POP bra:** Pop the top of the stack into bra.  
  Opcode: 0x820(a+3)  
  Cycles:

- **PEEK bra:** Load the top of the stack into bra without popping off the value.  
  Opcode: 0x820(a+6)  
  Cycles:

- **PUSH imm32:** Push imm32 to the stack.  
  Opcode: 0x8209  
  Cycles:

- **CLV:** Clear VRAM. Resets all bits in VRAM to 0.  
  Opcode: 0xFFFB  
  Cycles:

- **STOP:** Stop the CPU. Essentially a power-off message.  
  Opcode: 0xFFFC  
  Cycles:

- **EI:** Enable interrupts.  
  Opcode: 0xFFFD  
  Cycles:

- **DI:** Disable interrupts.  
  Opcode: 0xFFFE  
  Cycles:

- **HALT:** Halt the CPU, stopping CPU cycles until an external interrupt is received.  
  Opcode: 0xFFFF  
  Cycles:
