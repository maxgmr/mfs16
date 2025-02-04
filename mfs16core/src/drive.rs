use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use camino::Utf8Path;

mod drive_header;

use drive_header::DriveHeader;

#[derive(Debug)]
pub struct Drive {
    file: File,
    header: DriveHeader,
}
impl Drive {
    /// Create a new virtual hard drive at the given path, overwriting any existing file at that
    /// path.
    fn try_new<P: AsRef<Utf8Path>>(path: P, header: DriveHeader) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())?;
        file.set_len(header.size() as u64)?;
        // TODO write header contents to file
        Ok(Drive { file, header })
    }

    /// Attempt to load an existing virtual hard drive at the given path.
    fn try_init<P: AsRef<Utf8Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path.as_ref())?;
        // Init with dummy header to call methods
        let mut drive = Self {
            file,
            header: DriveHeader::default(),
        };
        // TODO load header
        Ok(drive)
    }

    /// Read a single block of data.
    pub fn read_block(&mut self, block_index: usize, buffer: &mut [u8]) -> io::Result<()> {
        // TODO
        todo!();
        Ok(())
    }

    /// Write a single block of data.
    pub fn write_block(&mut self, block_index: usize, data: &[u8]) -> io::Result<()> {
        // TODO
        todo!();
        Ok(())
    }
}
