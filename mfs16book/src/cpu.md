# CPU

The CPU runs at ~33.55 MHz (33 554 432 Hz), meaning it performs ~33 million cycles per second. It can read one 16-bit word per cycle.

## Registers

The CPU has 7 general-purpose registers: A, B, C, D, E, H, and L. Each register can store a single 16-bit word at a time.

Adjacent registers can be combined and accessed as a 32-bit double word "big register": BC, DE, and HL. These are most commonly used for indexing the 32-bit memory bus.

The high (_x1_) and low (_x0_) bytes of each register can be virtually accessed individually: A1, A0, B1, B0, C1, C0, D1, D0, E1, E0, H1, H0, L1, and L0.

The vast majority of CPU instructions are register-agnostic. Therefore, any register can be used as operands and/or outputs for all instructions.

## Flags

The CPU has 5 flags that can be set and/or reset by some CPU instructions. The 5 flags are as follows:

("iff" is short for "if and only if".)

- **Z (Zero):** This flag is typically set iff the result of the CPU instruction is equal to 0.

- **C (Carry):** This flag is typically set iff the result of the CPU instruction exceeds the available bits reserved for the output.

- **O (Overflow):** This flag is typically set iff the result of the CPU instruction is too large or too small to fit in the available bits when interpreted as a signed value. In contrast to the Carry flag, which usually only necessitates using the Carry flag in future operations, the Overflow flag usually means an error has occurred, assuming the goal of the instruction was to perform signed operations.

- **P (Parity):** This flag is typically set iff the result of the CPU instruction is even. Internally, the flag is set iff the lowest bit of the result is 0.

- **N (Negative):** This flag is typically set iff the result of the CPU instruction is negative when interpreted as a signed value. Internally, the flag is set iff the highest bit of the result is 1.
