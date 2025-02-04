const HEADER_ADDR: usize = 0x00;
const HEADER_LEN: usize = 0x100;

const DRIVE_NUMBER_ADDR: usize = 0x10;
const DRIVE_NAME_START: usize = DRIVE_NUMBER_ADDR + 0x01;
const DRIVE_NAME_LEN: usize = 0x10;
const BLOCK_SIZE_START: usize = DRIVE_NAME_START + DRIVE_NAME_LEN;
const BLOCK_SIZE_LEN: usize = 0x02;
const BLOCK_COUNT_START: usize = BLOCK_SIZE_START + BLOCK_SIZE_LEN;
const BLOCK_COUNT_LEN: usize = 0x02;
const DRIVE_FLAGS_ADDR: usize = BLOCK_COUNT_START + BLOCK_COUNT_LEN;

#[derive(Debug, Default)]
pub struct DriveHeader {
    drive_number: u8,
    drive_name: String,
    block_size: u16,
    block_count: u16,
    drive_flags: u8,
}
impl DriveHeader {
    /// Read the entire drive header.
    pub fn read_header(header_bytes: &[u8; HEADER_LEN]) -> Self {
        let drive_number = header_bytes[DRIVE_NUMBER_ADDR];
        let drive_name = String::from_utf8_lossy(
            &header_bytes[DRIVE_NAME_START..(DRIVE_NAME_START + DRIVE_NAME_LEN)],
        )
        .to_string();
        let block_size = <u16>::from_le_bytes(
            header_bytes[BLOCK_SIZE_START..(BLOCK_SIZE_START + BLOCK_SIZE_LEN)]
                .try_into()
                .unwrap(),
        );
        let block_count = <u16>::from_le_bytes(
            header_bytes[BLOCK_COUNT_START..(BLOCK_COUNT_START + BLOCK_COUNT_LEN)]
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

    /// Return the total size of the drive in bytes.
    #[inline(always)]
    pub fn size(&self) -> usize {
        (self.block_size as usize) * (self.block_count as usize)
    }
}
