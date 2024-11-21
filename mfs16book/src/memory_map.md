# Memory Map

The MFS-16 has a 32-bit address bus used to address ROM, RAM and I/O. The start and end addresses are inclusive.

In total, the MFS-16 has 8 MiB of ROM, 8 MiB of RAM, and 150 KiB of VRAM.

| Start       | End         | Size    | Description                                                                                                                       |
| ----------- | ----------- | ------- | --------------------------------------------------------------------------------------------------------------------------------- |
| 0x0000_0000 | 0x007F_FFFF | 8 MiB   | Read-only memory (ROM). Used for loaded programs currently being executed.                                                        |
| 0x0080_0000 | 0x00FF_FFFF | 8 MiB   | Random-access memory (RAM). General-purpose memory which can be read from or written to.                                          |
| 0x0100_0000 | 0x0102_5800 | 150 KiB | Video RAM (VRAM). Used for setting the pixels of the screen.                                                                      |
| 0xFFFF_FFBB | 0xFFFF_FFBB | 1 B     | Manual frame update address. Write-only. Write to this address to send a manual frame update.                                     |
| 0xFFFF_FFBC | 0xFFFF_FFBC | 1 B     | Disable manual frame updates address. Write-only. Write to this address to disable manual frame updates.                          |
| 0xFFFF_FFBD | 0xFFFF_FFBD | 1 B     | Enable manual frame updates address. Write-only. Write to this address to enable manual frame updates.                            |
| 0xFFFF_FFBE | 0xFFFF_FFFD | 64 B    | Keyboard register. Read-only. Each bit corresponds to a keyboard key. A bit is set when its key is being pressed, and vice versa. |
| 0xFFFF_FFFE | 0xFFFF_FFFE | 1 B     | Interrupt enable register. Each bit corresponds to a different interrupt. If an interrupt's bit is set, then it can be triggered. |
| 0xFFFF_FFFF | 0xFFFF_FFFF | 1 B     | Interrupt register. Each bit corresponds to a different interrupt. If an interrupt's bit is set, then it has been triggered.      |
