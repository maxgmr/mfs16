use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use camino::Utf8Path;

mod drive_header;

use drive_header::{DriveFlag::*, DriveHeader, HEADER_ADDR, HEADER_LEN};

macro_rules! BAD_BUF_SIZE_MSG {
    () => {
        "Buffer size {} != block size {}."
    };
}

macro_rules! BAD_INDEX_MSG {
    () => {
        "Block index {} is greater than max block index {}."
    };
}

#[derive(Debug)]
pub struct Drive {
    file: File,
    header: DriveHeader,
}
impl Drive {
    /// Create a new virtual hard drive at the given path, overwriting any existing file at that
    /// path.
    fn try_new<P: AsRef<Utf8Path>>(path: P, header: DriveHeader) -> io::Result<Self> {
        // Create a new file with the given size
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.as_ref())?;
        file.set_len(header.size() as u64)?;

        // Write the header to the file
        let header_bytes = header.to_bytes();
        file.seek(SeekFrom::Start(HEADER_ADDR as u64))?;
        file.write_all(&header_bytes)?;

        Ok(Self { file, header })
    }

    #[cfg(test)]
    /// Create a new virtual hard drive using a tempfile.
    fn try_new_temp(header: DriveHeader) -> io::Result<Self> {
        let mut file = tempfile::tempfile()?;
        file.set_len(header.size() as u64)?;

        // Write the header to the file
        let header_bytes = header.to_bytes();
        file.seek(SeekFrom::Start(HEADER_ADDR as u64))?;
        file.write_all(&header_bytes)?;

        Ok(Self { file, header })
    }

    /// Attempt to load an existing virtual hard drive at the given path.
    fn try_init<P: AsRef<Utf8Path>>(path: P) -> io::Result<Self> {
        // Open the file associated with this drive
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path.as_ref())?;

        // Read the header from the file
        let mut header_buffer = [0x00_u8; HEADER_LEN];
        file.seek(SeekFrom::Start(HEADER_ADDR as u64))?;
        file.read_exact(&mut header_buffer)?;
        let header = DriveHeader::read_header(&header_buffer);

        Ok(Self { file, header })
    }

    /// Read a single block of data.
    pub fn read_block(&mut self, block_index: usize, buffer: &mut [u8]) {
        // Attempt to read from the file, setting the ReadFail flag if something goes wrong.
        if self.prepare_file(block_index, buffer).is_err() || self.file.read_exact(buffer).is_err()
        {
            self.header.set_flag(&mut self.file, ReadFail).unwrap();
        }
    }

    /// Write a single block of data.
    pub fn write_block(&mut self, block_index: usize, data: &[u8]) {
        // Attempt to write to the file, setting the WriteFail flag if something goes wrong.
        if self.prepare_file(block_index, data).is_err() || self.file.write_all(data).is_err() {
            self.header.set_flag(&mut self.file, WriteFail).unwrap();
        }
    }

    // Helper fn to prepare the file for reading/writing- checks to make sure the parameters are
    // okay, then moves the cursor to the right spot.
    fn prepare_file(&mut self, block_index: usize, buffer: &[u8]) -> io::Result<()> {
        // Ensure the chosen index is within the range of the drive's blocks
        if block_index >= self.header.block_count().into() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(BAD_INDEX_MSG!(), block_index, self.header.block_count() - 1),
            ));
        }

        // Ensure the buffer size matches the block size
        if buffer.len() != (self.header.block_size() as usize) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(BAD_BUF_SIZE_MSG!(), buffer.len(), self.header.block_size(),),
            ));
        }

        // Prepare the cursor, taking the header size into account. Should panic if bad access- we
        // just checked to make sure that the access would be okay!
        self.file
            .seek(SeekFrom::Start(
                ((block_index as u64) * (self.header.block_size() as u64)) + (HEADER_LEN as u64),
            ))
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_new_drive() {
        const DUMMY_BYTE: u8 = 0xFF;
        let drive_number = 7;
        let drive_name = String::from("test_drive");
        const BLOCK_SIZE: usize = 512;
        const BLOCK_COUNT: usize = 4;
        let drive_flags = 0;
        let mut drive = Drive::try_new_temp(
            DriveHeader::new(
                drive_number,
                drive_name.clone(),
                BLOCK_SIZE as u16,
                BLOCK_COUNT as u16,
            )
            .unwrap(),
        )
        .unwrap();

        // Set up block data
        let mut block_data: [u8; BLOCK_SIZE] = [0x00; BLOCK_SIZE];
        let mut count: u8 = 0;
        for byte in block_data.iter_mut() {
            *byte = count;
            count = count.wrapping_add(1);
        }

        // Make sure able to read and write data blocks without any issues
        let mut read_block_data = [DUMMY_BYTE; BLOCK_SIZE];
        for block_num in 0..BLOCK_COUNT {
            // Ensure all drive blocks are all zero
            for block_num_2 in 0..BLOCK_COUNT {
                drive.read_block(block_num_2, &mut read_block_data);
                for byte in read_block_data {
                    assert_eq!(byte, 0x00);
                }
            }

            // Write block with block data (ascending bytes wrapping on overflow)
            drive.write_block(block_num, &block_data);

            for block_num_2 in 0..BLOCK_COUNT {
                drive.read_block(block_num_2, &mut read_block_data);

                if block_num_2 == block_num {
                    // Ensure that the current block has the right data
                    count = 0;
                    for byte in read_block_data {
                        assert_eq!(byte, count);
                        count = count.wrapping_add(1);
                    }
                } else {
                    // Ensure all other blocks are still all zero
                    for byte in read_block_data {
                        assert_eq!(byte, 0x00);
                    }
                }
            }

            // Check to make sure nothing messed with the header
            assert_eq!(drive.header.drive_number(), drive_number);
            assert_eq!(drive.header.drive_name(), &drive_name);
            assert_eq!(drive.header.block_size(), BLOCK_SIZE as u16);
            assert_eq!(drive.header.block_count(), BLOCK_COUNT as u16);
            assert_eq!(drive.header.drive_flags(), drive_flags);

            // "Reset" current block to all zeroes
            drive.write_block(block_num, &[0x00; BLOCK_SIZE]);
        }
    }

    #[test]
    fn test_bad_reads_writes() {
        const BLOCK_SIZE: usize = 512;
        const BLOCK_COUNT: usize = 16;
        const START_VAL: u8 = 0xFF;
        let mut drive = Drive::try_new_temp(
            DriveHeader::new(
                0,
                "my_drive".to_string(),
                BLOCK_SIZE as u16,
                BLOCK_COUNT as u16,
            )
            .unwrap(),
        )
        .unwrap();

        let mut badly_sized_buf = [START_VAL; BLOCK_SIZE + 1];
        let orig_badly_sized_buf = badly_sized_buf;
        let mut bad_index_buf = [START_VAL; BLOCK_SIZE];
        let orig_bad_index_buf = bad_index_buf;

        // Test bad buffer size err msg
        let err = drive.prepare_file(0, &badly_sized_buf).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!(BAD_BUF_SIZE_MSG!(), BLOCK_SIZE + 1, BLOCK_SIZE)
        );

        // Test bad index err msg
        let err = drive.prepare_file(BLOCK_COUNT, &bad_index_buf).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!(BAD_INDEX_MSG!(), BLOCK_COUNT, BLOCK_COUNT - 1)
        );

        // Test bad reads: shouldn't change badly_sized_buf and should set ReadFail
        // Bad buffer size
        drive.read_block(0, &mut badly_sized_buf);
        assert!(drive.header.flag(ReadFail));
        assert!(!drive.header.flag(WriteFail));
        assert_eq!(badly_sized_buf, orig_badly_sized_buf);
        drive.header.reset_flag(&mut drive.file, ReadFail).unwrap();
        // Bad block index
        drive.read_block(BLOCK_COUNT, &mut bad_index_buf);
        assert!(drive.header.flag(ReadFail));
        assert!(!drive.header.flag(WriteFail));
        assert_eq!(bad_index_buf, orig_bad_index_buf);
        drive.header.reset_flag(&mut drive.file, ReadFail).unwrap();

        // Test bad writes: shouldn't change the block and should set WriteFail
        // Bad buffer size
        drive.write_block(0, &badly_sized_buf);
        assert!(drive.header.flag(WriteFail));
        assert!(!drive.header.flag(ReadFail));
        let mut chk_buf = [START_VAL; BLOCK_SIZE];
        drive.read_block(0, &mut chk_buf);
        assert!(!drive.header.flag(ReadFail));
        assert_eq!(chk_buf, [0x00; BLOCK_SIZE]);
        drive.header.reset_flag(&mut drive.file, WriteFail).unwrap();
        // Bad block index
        drive.write_block(BLOCK_COUNT, &bad_index_buf);
        assert!(drive.header.flag(WriteFail));
        assert!(!drive.header.flag(ReadFail));
    }
}
