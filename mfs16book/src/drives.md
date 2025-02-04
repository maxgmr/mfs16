# Drives

Virtual MFS-16 hard drives are stored as the original `.mfsd` (MFS-Drive) file format.

## Location

By default, the virtual hard drive files can be found in one of the following locations:

- **Linux:** `~/.local/share/mfs16desktop/`
- **macOS:** `/Users/<USER>/Library/Application Support/ca.maxgmr.mfs16desktop/`
- **Windows:**`C:\Users\<USER>\AppData\Local\maxgmr\mfs16desktop\data\`

## Drive Header Format

The first 256 bytes (`0x00`..=`0xFF`) are devoted to the drive header.

### 0x10 - Drive Number

A number from 0-255 denoting the drive number.

### 0x11..=0x20 - Drive Name

The name of the drive. Up to 16 characters of ASCII-encoded text. A `null` byte signals an early end to the name.

### 0x21..=0x22 - Block Size

The number of bytes per block, stored little-endian.

### 0x23..=0x24 - Block Count

The number of blocks on the drive, stored little-endian.

### 0x25 - Drive Flags

The flags of the drive device itself. Each bit corresponds to a given flag:

| 7 6 5 4 3 | 2    | 1         | 0        |
| --------- | ---- | --------- | -------- |
|           | Busy | WriteFail | ReadFail |

- **ReadFail**: This flag is set if a read operation failed.
- **WriteFail**: This flag is set if a write operation failed.
- **Busy**: This flag is set if the drive is currently performing an operation.
