# Memory Map

The MFS-16 has a 32-bit address bus used to address ROM, RAM and I/O. The start and end addresses are inclusive.

| Start       | End         | Size    | Description                                                                              |
| ----------- | ----------- | ------- | ---------------------------------------------------------------------------------------- |
| 0x0000_0000 | 0x007F_FFFF | 512 KiB | Read-only memory (ROM). Used for loaded programs currently being executed.               |
| 0x0080_0000 | 0x00FF_FFFF | 512 KiB | Random-access memory (RAM). General-purpose memory which can be read from or written to. |
| 0x0100_0000 | 0x0100_95FF | 9600 B  | Video RAM (VRAM). Used for setting the pixels of the screen.                             |
