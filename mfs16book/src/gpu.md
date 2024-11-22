# GPU

Frames are displayed by periodically reading the VRAM and translating said VRAM into pixels.

The screen resolution is 640x480. Each pixel takes up 4 bits of VRAM, so VRAM is 153 600 bytes in size.

The MFS-16 has a display of 16 colours. Each pixel's 4 bits in VRAM denotes which palette colour that pixel is.

![Default Colours](./image/default_colours.png "The default MFS-16 palette.")

_The default MFS-16 palette._

0x0 is the conventional background colour, and 0xF is the conventional foreground colour.

The 16 colours can be anything, but by convention they map to the 16 standard ANSI terminal colours:

| VRAM Pixel Value | ANSI Colour    |
| ---------------- | -------------- |
| 0x0              | Black          |
| 0x1              | Red            |
| 0x2              | Green          |
| 0x3              | Yellow         |
| 0x4              | Blue           |
| 0x5              | Magenta        |
| 0x6              | Cyan           |
| 0x7              | White          |
| 0x8              | Bright Black   |
| 0x9              | Bright Red     |
| 0xA              | Bright Green   |
| 0xB              | Bright Yellow  |
| 0xC              | Bright Blue    |
| 0xD              | Bright Magenta |
| 0xE              | Bright Cyan    |
| 0xF              | Bright White   |

Since the VRAM bytes are processed sequentially by the screen, and words are written to memory in little-endian form, programs must account for the little-endian ordering when writing data to VRAM.

For example, to write the first 8 colours in the above table ordered left-to-right horizontally (i.e., black, red, green, ... cyan, white), the following instructions could be executed:

```asm
// Note the order of the nibbles!
// This will be written to VRAM as [0x01, 0x23, 0x45, 0x67].
LD BC, 0x67_45_23_01:d;
VLD [DE], BC;
```
