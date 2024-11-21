# mfs16

A 16-bit virtual computer.

## TODO

- Kernel-only instructions

  - Interactions with I/O devices

  - MMU manipulation

  - (Re)define how interrupts are handled

- Man page for assembler

- Arithmetic evaluation for assembler

- MFS-16 program header standard

- Give notice if the old default config is different than the new one

- Boot program, hard-coded to be loaded at the start of memory, sets up everything else then frees up the space it took up.

- Merge CLI tools into one with different subcommands

- Sprite data format and converter

- Add offset argument to parsing error for more accurate errors

- Change break conditions structure to be more toml-friendly (e.g. replace enums with structs with named fields)

- Sound!
