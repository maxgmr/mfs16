use std::{
    fs::File,
    io::{self, Seek, SeekFrom, Write},
};

use DriveFlag::*;

pub const HEADER_ADDR: usize = 0x00;
pub const HEADER_LEN: usize = 0x100;

pub const DRIVE_NUMBER_ADDR: usize = 0x10;
pub const DRIVE_NAME_START: usize = DRIVE_NUMBER_ADDR + 0x01;
pub const DRIVE_NAME_LEN: usize = 0x10;
pub const BLOCK_SIZE_START: usize = DRIVE_NAME_START + DRIVE_NAME_LEN;
pub const BLOCK_SIZE_LEN: usize = 0x02;
pub const BLOCK_COUNT_START: usize = BLOCK_SIZE_START + BLOCK_SIZE_LEN;
pub const BLOCK_COUNT_LEN: usize = 0x02;
pub const DRIVE_FLAGS_ADDR: usize = BLOCK_COUNT_START + BLOCK_COUNT_LEN;

pub const NAME_TOO_LONG_MSG: &str = "Drive name is too long.";

/// Get a slice of the given array starting at the given start_addr with length range_len.
/// Example: `len_range!(my_arr, 2, 4)` is equivalent to `my_arr[2..6]`.
macro_rules! len_range {
    ($array:ident[$start_addr:expr, $range_len:expr]) => {
        $array[$start_addr..($start_addr + $range_len)]
    };
}

/// Enum to access the different drive flags of the header.
#[derive(Debug, Clone, Copy)]
pub enum DriveFlag {
    /// This flag is set if a read operation failed.
    ReadFail,
    /// This flag is set if a write operation failed.
    WriteFail,
    /// This flag is set if the drive is currently performing an operation.
    Busy,
}
impl DriveFlag {
    /// Get the bit index associated with this [DriveFlag].
    pub fn bit_index(&self) -> u8 {
        match self {
            ReadFail => 0,
            WriteFail => 1,
            Busy => 2,
        }
    }
}

#[derive(Debug, Default)]
pub struct DriveHeader {
    drive_number: u8,
    drive_name: String,
    block_size: u16,
    block_count: u16,
    drive_flags: u8,
}
impl DriveHeader {
    /// Create a new drive header.
    pub fn new(
        drive_number: u8,
        drive_name: String,
        block_size: u16,
        block_count: u16,
    ) -> io::Result<Self> {
        Self::check_drive_name_len(&drive_name)?;
        Ok(Self {
            drive_number,
            drive_name,
            block_size,
            block_count,
            drive_flags: 0x00,
        })
    }

    /// Read the entire drive header from the given byte slice.
    pub fn read_header(header_bytes: &[u8; HEADER_LEN]) -> Self {
        let drive_number = header_bytes[DRIVE_NUMBER_ADDR];
        let drive_name =
            String::from_utf8_lossy(&len_range!(header_bytes[DRIVE_NAME_START, DRIVE_NAME_LEN]))
                .to_string()
                .replace("\0", "");
        let block_size = <u16>::from_le_bytes(
            len_range!(header_bytes[BLOCK_SIZE_START, BLOCK_SIZE_LEN])
                .try_into()
                .unwrap(),
        );
        let block_count = <u16>::from_le_bytes(
            len_range!(header_bytes[BLOCK_COUNT_START, BLOCK_COUNT_LEN])
                .try_into()
                .unwrap(),
        );
        let drive_flags = header_bytes[DRIVE_FLAGS_ADDR];
        Self {
            drive_number,
            drive_name,
            block_size,
            block_count,
            drive_flags,
        }
    }

    /// Output this header in its raw byte form.
    pub fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut bytes = [0x00; HEADER_LEN];
        // Set drive number
        bytes[DRIVE_NUMBER_ADDR] = self.drive_number();
        // Set drive name
        len_range!(bytes[DRIVE_NAME_START, DRIVE_NAME_LEN])
            .copy_from_slice(&Self::drive_name_bytes(&self.drive_name));
        // Set block size
        len_range!(bytes[BLOCK_SIZE_START, BLOCK_SIZE_LEN])
            .copy_from_slice(&self.block_size().to_le_bytes());
        // Set block count
        len_range!(bytes[BLOCK_COUNT_START, BLOCK_COUNT_LEN])
            .copy_from_slice(&self.block_count().to_le_bytes());
        // Set drive flags
        bytes[DRIVE_FLAGS_ADDR] = self.drive_flags();
        bytes
    }

    /// Return the total size of the drive in bytes.
    #[inline(always)]
    pub fn size(&self) -> usize {
        ((self.block_size as usize) * (self.block_count as usize)) + HEADER_LEN
    }

    /// Get the value of the given [DriveFlag].
    #[inline(always)]
    pub fn flag(&self, drive_flag: DriveFlag) -> bool {
        (self.drive_flags & (1 << drive_flag.bit_index())) != 0
    }

    /// Helper function to ensure that the length of the given drive name isn't too long.
    fn check_drive_name_len(drive_name: &str) -> io::Result<()> {
        if drive_name.len() > DRIVE_NAME_LEN {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                NAME_TOO_LONG_MSG,
            ));
        }

        Ok(())
    }

    /// Helper function to convert a given string into a byte array with the proper drive name
    /// length.
    /// Panics if `drive_name` is larger than the length of the drive name header field.
    fn drive_name_bytes(drive_name: &str) -> [u8; DRIVE_NAME_LEN] {
        if drive_name.len() > DRIVE_NAME_LEN {
            panic!();
        }
        let mut result = [0x00; DRIVE_NAME_LEN];
        len_range!(result[0, drive_name.len()]).copy_from_slice(drive_name.as_bytes());
        result
    }

    // SETTERS
    // DON'T change the fields of this DriveHeader in ANY OTHER WAY, otherwise the file data and
    // the values of this struct might not match!

    /// Set the given [DriveFlag].
    pub fn set_flag(&mut self, file: &mut File, drive_flag: DriveFlag) -> io::Result<()> {
        self.change_flag(file, drive_flag, true)?;
        Ok(())
    }

    /// Reset the given [DriveFlag].
    pub fn reset_flag(&mut self, file: &mut File, drive_flag: DriveFlag) -> io::Result<()> {
        self.change_flag(file, drive_flag, false)?;
        Ok(())
    }

    /// Change the given [DriveFlag] to the given value.
    pub fn change_flag(
        &mut self,
        file: &mut File,
        drive_flag: DriveFlag,
        value: bool,
    ) -> io::Result<()> {
        let new_flags = if value {
            self.drive_flags | 1 << drive_flag.bit_index()
        } else {
            self.drive_flags & !(1 << drive_flag.bit_index())
        };
        self.set_drive_flags(file, new_flags)?;
        Ok(())
    }

    /// Set the number of this drive.
    pub fn set_drive_number(&mut self, file: &mut File, new_drive_number: u8) -> io::Result<()> {
        Self::write_data(file, DRIVE_NUMBER_ADDR as u64, &[new_drive_number])?;
        self.drive_number = new_drive_number;
        Ok(())
    }

    /// Set the name of this drive.
    pub fn set_drive_name(&mut self, file: &mut File, new_drive_name: String) -> io::Result<()> {
        Self::check_drive_name_len(&new_drive_name)?;
        Self::write_data(file, DRIVE_NAME_START as u64, new_drive_name.as_bytes())?;
        self.drive_name = new_drive_name;
        Ok(())
    }

    /// Set the flags of this drive.
    pub fn set_drive_flags(&mut self, file: &mut File, new_drive_flags: u8) -> io::Result<()> {
        Self::write_data(file, DRIVE_FLAGS_ADDR as u64, &[new_drive_flags])?;
        self.drive_flags = new_drive_flags;
        Ok(())
    }

    // Helper function to write header data to drive file.
    fn write_data(file: &mut File, start_addr: u64, data: &[u8]) -> io::Result<()> {
        file.seek(SeekFrom::Start(start_addr))?;
        file.write_all(data)?;
        Ok(())
    }

    // Block size and block count should NOT be changed!! :)

    // GETTERS
    // Important to restrict API as much as possible to avoid desync issues between this struct and
    // the actual data stored on the drive.

    /// Get the number of this drive.
    #[inline(always)]
    pub fn drive_number(&self) -> u8 {
        self.drive_number
    }

    /// Get the name of this drive.
    #[inline(always)]
    pub fn drive_name(&self) -> &str {
        &self.drive_name
    }

    /// Get the block size of this drive.
    #[inline(always)]
    pub fn block_size(&self) -> u16 {
        self.block_size
    }

    /// Get the block count of this drive.
    #[inline(always)]
    pub fn block_count(&self) -> u16 {
        self.block_count
    }

    /// Get the flags of this drive.
    #[inline(always)]
    pub fn drive_flags(&self) -> u8 {
        self.drive_flags
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use pretty_assertions::assert_eq;
    use tempfile::tempfile;

    use super::*;

    #[test]
    fn test_name_too_big() {
        let err = DriveHeader::new(0, "this drive name is wayyy too big.".to_owned(), 512, 512)
            .unwrap_err();
        assert_eq!(&err.to_string(), NAME_TOO_LONG_MSG);
    }

    #[test]
    fn test_read_header() {
        let drive_number = 0x02;
        let drive_name = "hello_world";
        let drive_name_bytes = DriveHeader::drive_name_bytes(drive_name);
        let block_size: u16 = 512;
        let block_size_bytes = block_size.to_le_bytes();
        let block_count: u16 = 128;
        let block_count_bytes = block_count.to_le_bytes();
        let drive_flags = 1 << WriteFail.bit_index();

        // Set up header bytes
        let mut test_bytes = [0x00_u8; HEADER_LEN];
        // Set drive number
        test_bytes[DRIVE_NUMBER_ADDR] = drive_number;
        // Set drive name
        len_range!(test_bytes[DRIVE_NAME_START, DRIVE_NAME_LEN]).copy_from_slice(&drive_name_bytes);
        // Set block size
        len_range!(test_bytes[BLOCK_SIZE_START, BLOCK_SIZE_LEN]).copy_from_slice(&block_size_bytes);
        // Set block count
        len_range!(test_bytes[BLOCK_COUNT_START, BLOCK_COUNT_LEN])
            .copy_from_slice(&block_count_bytes);
        // Set WriteFail flag
        test_bytes[DRIVE_FLAGS_ADDR] = drive_flags;

        let header = DriveHeader::read_header(&test_bytes);

        // Ensure all the values were read correctly
        assert_eq!(drive_number, header.drive_number());
        assert_eq!(drive_name, header.drive_name());
        assert_eq!(block_size, header.block_size());
        assert_eq!(block_count, header.block_count());
        assert_eq!(drive_flags, header.drive_flags());
    }

    #[test]
    fn test_header_as_bytes() {
        let drive_number = 7;
        let drive_name = String::from("my_awesome_drive");
        let drive_name_bytes = DriveHeader::drive_name_bytes(&drive_name);
        let block_size: u16 = 2048;
        let block_size_bytes = block_size.to_le_bytes();
        let block_count: u16 = 512;
        let block_count_bytes = block_count.to_le_bytes();
        let drive_flags = 1 << Busy.bit_index();
        let mut header =
            DriveHeader::new(drive_number, drive_name.clone(), block_size, block_count).unwrap();
        let mut tempfile = tempfile().unwrap();
        tempfile.set_len(HEADER_LEN as u64).unwrap();
        header.set_drive_flags(&mut tempfile, drive_flags).unwrap();

        let bytes = header.to_bytes();

        assert_eq!(drive_number, bytes[DRIVE_NUMBER_ADDR]);
        assert_eq!(
            drive_name_bytes,
            &len_range!(bytes[DRIVE_NAME_START, DRIVE_NAME_LEN])
        );
        assert_eq!(
            block_size_bytes,
            len_range!(bytes[BLOCK_SIZE_START, BLOCK_SIZE_LEN])
        );
        assert_eq!(
            block_count_bytes,
            len_range!(bytes[BLOCK_COUNT_START, BLOCK_COUNT_LEN])
        );
        assert_eq!(drive_flags, bytes[DRIVE_FLAGS_ADDR]);
    }

    #[test]
    fn test_header_size() {
        let drive_number = 2;
        let drive_name = String::from("test_drive");
        let block_size = 4096;
        let block_count = 128;
        let drive_flags = 0;
        let header = DriveHeader {
            drive_number,
            drive_name,
            block_size,
            block_count,
            drive_flags,
        };

        assert_eq!(
            header.size(),
            ((block_size as usize) * (block_count as usize)) + HEADER_LEN
        );
    }

    #[test]
    fn test_header_setters() {
        let mut drive_number = 0;
        let mut drive_name = "a";
        let block_size = 512;
        let block_count = 512;
        let mut drive_flags = 0;

        let mut drive_number_buf: [u8; 1] = [0x00];
        let mut drive_name_buf: [u8; 16] = [0x00; 16];
        let mut block_size_buf: [u8; 2] = [0x00; 2];
        let mut block_count_buf: [u8; 2] = [0x00; 2];
        let mut drive_flags_buf: [u8; 1] = [0x00];

        let mut header = DriveHeader {
            drive_number,
            drive_name: drive_name.to_owned(),
            block_size,
            block_count,
            drive_flags,
        };
        let mut tempfile = tempfile().unwrap();
        tempfile.set_len(HEADER_LEN as u64).unwrap();
        tempfile.seek(SeekFrom::Start(0)).unwrap();
        tempfile.write_all(&header.to_bytes()).unwrap();

        macro_rules! chk_header_val {
            ($start_addr:expr, $buf:expr, $expected:expr) => {
                tempfile.seek(SeekFrom::Start($start_addr as u64)).unwrap();
                tempfile.read_exact(&mut $buf).unwrap();
                assert_eq!($buf, $expected)
            };
        }

        macro_rules! chk_header {
            () => {
                // Check header values
                assert_eq!(drive_number, header.drive_number);

                // Check file values
                chk_header_val!(DRIVE_NUMBER_ADDR, drive_number_buf, [drive_number]);
                chk_header_val!(
                    DRIVE_NAME_START,
                    drive_name_buf,
                    DriveHeader::drive_name_bytes(drive_name)
                );
                chk_header_val!(BLOCK_SIZE_START, block_size_buf, block_size.to_le_bytes());
                chk_header_val!(
                    BLOCK_COUNT_START,
                    block_count_buf,
                    block_count.to_le_bytes()
                );
                chk_header_val!(DRIVE_FLAGS_ADDR, drive_flags_buf, [drive_flags]);
            };
        }

        chk_header!();

        drive_number = 7;
        header
            .set_drive_number(&mut tempfile, drive_number)
            .unwrap();
        chk_header!();

        drive_name = "my_drive";
        header
            .set_drive_name(&mut tempfile, drive_name.to_string())
            .unwrap();
        chk_header!();

        drive_flags |= 1 << Busy.bit_index();
        header.set_flag(&mut tempfile, Busy).unwrap();
        chk_header!();

        header.set_flag(&mut tempfile, Busy).unwrap();
        chk_header!();

        drive_flags &= !(1 << WriteFail.bit_index());
        header.reset_flag(&mut tempfile, WriteFail).unwrap();
        chk_header!();

        drive_flags |= 1 << WriteFail.bit_index();
        header.change_flag(&mut tempfile, WriteFail, true).unwrap();
        chk_header!();

        header.change_flag(&mut tempfile, WriteFail, true).unwrap();
        chk_header!();

        header.change_flag(&mut tempfile, ReadFail, false).unwrap();
        chk_header!();

        drive_flags &= !(1 << WriteFail.bit_index());
        header.change_flag(&mut tempfile, WriteFail, false).unwrap();
        chk_header!();
    }
}
