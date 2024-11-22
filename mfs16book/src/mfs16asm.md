# MFS-16 Assembly

MFS-16 assembly language shares many similarities with other variants of assembly, but has some features that make it unique.

## Comments

Comments can be single-line...

```
// Hello, I'm a comment. I am ignored by the MFS-16 assembler.
```

... or multi-line.

```
/*
    This is a big, multiline comment.

    It can be as long as you want.
*/
```

They can be located at any point in the line.

```
LD A,B; // This loads register B into register A.
```

## Variables

Variables may be assigned at any point with the following format:

```
variable_name = number:data_type;
```

Since a semicolon ends the statement, variable assignments may be multiple lines.

Variable assignments do not take up any space in the final binary.

Valid variable names consist of alphanumeric characters, underscores, and numbers, but they cannot start with a number.

```
// OK!
my_variable = 255;

// NO! Must not start with a number.
12_my_variable = 254;

// NO! Illegal characters in variable name.
my+variable = 253;
```

After assignment, variables can be used in place of literal values (u4, imm8, imm16, imm32, etc.):

```
// This...
my_variable = 255;
ld A1,my_variable;

// ...does the same thing as this.
ld A1,255;
```

## Literals

The data types of numbers must be explicitly specified by following the number with a colon, then a single letter denoting the data type. MFS-16 assembly supports four data types:

- **`:b` (byte)**: An 8-bit value. If no data type is given, the assembler will assume the value is a byte (and fail if the value is too large to be a byte!). Can be used with 8-bit virtual registers (A1, A0, B1, etc.).

- **`:w` (word)**: A 16-bit value. Used with 16-bit registers (A, B, C, etc.).

- **`:d` (double word)**: A 32-bit value. Used with 32-bit registers (BC, DE, HL).

- **`:q` (quad word)**: A 64-bit value.

Decimal, binary, hexadecimal, and octal notation are all supported, and underscores may be used anywhere to visually separate the digits:

```
// OK!
ADD B0,123;

// OK!
CMP 0x1234:d, DE;

// OK!
my_double = 1_234_567_890:d;

// OK!
my_byte = 0xAB;

// OK!
MY_OTHER_BYTE_1234 = 0xCD:b;

// OK!
myoctalword = 0o780:w;

// OK!
a_quad
    = 0b_0101_0101_0101_0101_0101:q;

// NO! value is too large to be a byte.
my_byte_4 = 0x100:b;
```

## Registers

The registers can be referenced by their (case-sensitive) names.

- **16-bit Registers:** `A`, `B`, `C`, `D`, `E`, `H`, `L`

- **32-bit Big Registers:** `BC`, `DE`, `HL`

- **8-bit Virtual Registers:** `A1`, `A0`, `B1`, `B0`, `C1`, `C0`, `D1`, `D0`, `E1`, `E0`, `H1`, `H0`, `L1`, `L0`

These names are reserved and cannot be used as variable names.

## Instructions

Instructions are based around the following format:

```
mnemonic [operand], [operand];
```

They always consist of the instruction mnemonic, optionally followed by its operands (separated by commas), and always ending with a semicolon.

Mnemonics are case-insensitive.

They can be spread across multiple lines because unlike other assembly variants, semicolons signal the end of the instruction:

```
// OK!
ld A, B;

// OK!
ld
    A,
    B;

// OK!
ld  A,  B;
```

Here are some more examples of instructions:

```
halt;

ld [0x0012_3456:d],BC;

JP my_address;

InC DE;
```

## Dereferences

Brackets `[`, `]` are used to dereference addresses:

```
// Load my_word into memory at address BC
LD [BC],my_word;

// Load the value stored at address HL into A
LD A,[HL];
```

## Named Labels

Named labels are globally-scoped identifiers that point to a specific location in the program ROM. Internally, the labels are 32-bit memory addresses which can be jumped to and referenced in other ways like a normal double word variable. The assembler places them as early as possible within memory.

Named labels follow the same rules for variable names and they take the following format:

```
label_name:
```

Their scope extends across all files used to assemble the final binary, and can be referenced before or after their declaration.

In the following example, the program counter travels from locations 1-4 in order:

```
// LOCATION 1 (START)
jp start;

// LOCATION 2
start:
jp label_2;

// LOCATION 4 (FINISH)
label_1:
stop;

// LOCATION 3
label_2:
jp label_1;

```

## Explicit Labels

Exact addresses in memory can be denoted through explicit labels. They follow the same format and scope as named labels, but they are double word literals instead of variable names:

```
// The assembled binary code after this label starts at address 0x200 in memory.
0x0200:d:
// This instruction will be located at address 0x200 in memory.
ld A1,123;
```

They are commonly used for interrupt handlers, because CPU interrupts always jump to the same memory address:

```
0x100:d:
    // Handle frame interrupts
    call my_frame_function;
reti;

0x200:d:
    // Handle keyboard interrupts
    call do_something_when_keys_pressed;
reti;
```

Keep in mind that the assembler will fail if there is an explicit label that is too small:

```
// OK! Since this is the start of the ROM, the first two bytes are simply empty.
0x00_0002:d:
ld A,0x123:w;
ld B,0x456:w;
add A,B;
/*
    NO! The instructions above start at 0x00_0002 and take up more than two bytes
    of memory. 0x00_0004 is too small of a value.
*/
0x00_0004:
divu A,B;
```

This also means that the order in which you give your files to the assembler matters- since files are assembled sequentially, the explicit labels of files later on in the sequence must take the size of files earlier in the sequence into account when definint explicit labels.

## Raw Byte Arrays

Raw bytes can be defined at any point in memory through raw byte arrays. Raw byte arrays have the following format:

```
[
  [bytes],
]
```

The bytes can be variables, the type suffix can be omitted, and commas are optional:

```
// OK!
[ 0x01:b, 0x23:b, 0x45:b ]

// OK!
[ 1, 0x23, 0x45 ]

// OK!
[
    0x01
    0x23
    0x45
]

// OK!
b1 = 0x01;
b2 = 0x23:b;
b3 = 0x45;
[ b1 b2 b3 ]
```

They are typically used in tandem with explicit labels in order to define some static data in ROM for future usage, such as raw bitmap data:

```
// Set DE to the start of sprite data
ld DE, sprite_data;
// Set HL to the start of VRAM
ld HL, 0x0100_0000:d;
// Write sprite data to VRAM
ldi BC,[DE];
vldi [HL],BC;
ldi BC,[DE];
vldi [HL],BC;

0x0000_1000:d:
sprite_data:
[
    0x12 0x34 0x56 0x78 0x9A 0xBC 0xDE 0xF0
]
```

## Assembler

The assembler is given a list of files to assemble. The order of the files matters, as the files are simply appended to each other and processed accordingly.

In the following example, `another_file.mfs16` is appended to the end of `file.mfs16`. Any variable assignments in `file.mfs16` apply to `another_file.mfs16`, but they can be overwritten!

```sh
mfs16assembler path/to/file.mfs16 path/to/another_file.mfs16
```

If the assembler is not given an output file, it sends the resulting binary to stdout. An output file can be given using the `-o` option:

```sh
mfs16assembler my_program.mfs16 -o bin/my_program
```

The assembler won't overwrite existing files by default. This behaviour can be overridden by adding the `-f` flag.
