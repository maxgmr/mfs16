//! Memory management unit. Responsible for memory reads and writes across all components of the
//! system.
use std::{default::Default, fmt::Display};

use crate::{
    computer::{BLOCK_SIZE, DMA_BYTES_PER_CYCLE},
    drive::Drive,
    gpu::Gpu,
    keyboard::{KbReg, KB_REG_SIZE},
    memory::Memory,
    DriveFlag, DRIVE_FLAGS_ADDR, RAM_OFFSET, RAM_SIZE, ROM_OFFSET, ROM_SIZE, VRAM_OFFSET,
    VRAM_SIZE,
};

/// This byte is returned when the memory can't be read for any reason.
pub const NOT_READABLE_BYTE: u8 = 0xFF;

/// The number of cycles the DMA transfer takes.
pub const DMA_TRANSFER_CYCLES: usize = BLOCK_SIZE / DMA_BYTES_PER_CYCLE;

const ROM_END: usize = ROM_OFFSET + ROM_SIZE;
const RAM_END: usize = RAM_OFFSET + RAM_SIZE;
const VRAM_END: usize = VRAM_OFFSET + VRAM_SIZE;

const DMA_R_RAM_ADDR_SIZE: usize = 4;
/// Write to this address to initiate a drive DMA read.
pub const DMA_R_INIT_ADDR: usize = DMA_R_DRIVE_NUM_ADDR - 1;
/// This address stores the number of the read drive.
pub const DMA_R_DRIVE_NUM_ADDR: usize = DMA_R_BLOCK_ADDR - 1;
/// This address stores the number of the block which will be read.
pub const DMA_R_BLOCK_ADDR: usize = DMA_R_RAM_ADDR_START - 1;
/// This area stores the location in RAM to which the drive DMA will be written (little-endian).
pub const DMA_R_RAM_ADDR_START: usize = DMA_R_RAM_ADDR_END - DMA_R_RAM_ADDR_SIZE;
const DMA_R_RAM_ADDR_END: usize = DMA_W_INIT_ADDR;

const DMA_W_RAM_ADDR_SIZE: usize = 4;
/// Write to this address to initiate a drive DMA write.
pub const DMA_W_INIT_ADDR: usize = DMA_W_DRIVE_NUM_ADDR - 1;
/// This address stores the number of the write drive.
pub const DMA_W_DRIVE_NUM_ADDR: usize = DMA_W_BLOCK_ADDR - 1;
/// This address stores the number of the block which will be written to.
pub const DMA_W_BLOCK_ADDR: usize = DMA_W_RAM_ADDR_START - 1;
/// This area stores the location in RAM which will be written to the drive (little-endian).
pub const DMA_W_RAM_ADDR_START: usize = DMA_W_RAM_ADDR_END - DMA_W_RAM_ADDR_SIZE;
const DMA_W_RAM_ADDR_END: usize = VRAM_DMA_R_INIT_ADDR;

// TODO
const VRAM_DMA_R_RAM_ADDR_SIZE: usize = 4;
/// Write to this address to initiate a VRAM DMA read.
pub const VRAM_DMA_R_INIT_ADDR: usize = VRAM_DMA_R_RAM_ADDR_START - 1;
/// This address stores the location in RAM to which the VRAM will be read (little-endian).
pub const VRAM_DMA_R_RAM_ADDR_START: usize = VRAM_DMA_R_RAM_ADDR_END - VRAM_DMA_R_RAM_ADDR_SIZE;
const VRAM_DMA_R_RAM_ADDR_END: usize = VRAM_DMA_W_INIT_ADDR;

const VRAM_DMA_W_RAM_ADDR_SIZE: usize = 4;
/// Write to this address to initiate a VRAM DMA write.
pub const VRAM_DMA_W_INIT_ADDR: usize = VRAM_DMA_W_RAM_ADDR_START - 1;
/// This area stores the location in RAM from which the VRAM will be written (little-endian).
pub const VRAM_DMA_W_RAM_ADDR_START: usize = VRAM_DMA_W_RAM_ADDR_END - VRAM_DMA_W_RAM_ADDR_SIZE;
const VRAM_DMA_W_RAM_ADDR_END: usize = ERR_REG_ADDR;

/// Address of the error register.
pub const ERR_REG_ADDR: usize = MAN_FRAME_UPDATE_ADDR - 1;

/// Write to this address to send a manual frame update.
pub const MAN_FRAME_UPDATE_ADDR: usize = MAN_FRAME_DISABLE_ADDR - 1;
/// Write to this address to turn off manual frame updates.
pub const MAN_FRAME_DISABLE_ADDR: usize = MAN_FRAME_ENABLE_ADDR - 1;
/// Write to this address to turn on manual frame updates.
pub const MAN_FRAME_ENABLE_ADDR: usize = KB_REG_START - 1;

/// Start address of the keyboard register.
pub const KB_REG_START: usize = IE_REGISTER_ADDR - KB_REG_SIZE;

const KB_REG_END: usize = IE_REGISTER_ADDR;

/// Address of the interrupt enable register.
pub const IE_REGISTER_ADDR: usize = INTERRUPT_REGISTER_ADDR - 1;
/// Address of the interrupt register.
pub const INTERRUPT_REGISTER_ADDR: usize = 0xFFFF_FFFF;

/// The memory management unit. Routes reads/writes and controls computer state.
#[derive(Debug, PartialEq, Clone)]
pub struct Mmu {
    /// The read-only memory of the computer.
    pub rom: Memory,
    /// The random-access memory of the computer.
    pub ram: Memory,
    /// The graphics processing unit of the computer.
    pub gpu: Gpu,
    /// DMA read: the number of the drive to read from.
    pub dma_r_drive_num_reg: u8,
    /// DMA read: the number of the drive block to read.
    pub dma_r_block_num_reg: u8,
    /// DMA read: the drive block data is read into RAM starting at this address.
    pub dma_r_ram_start_reg: u32,
    /// DMA write: the number of the drive to write to.
    pub dma_w_drive_num_reg: u8,
    /// DMA write: the number of the drive block to be overwritten.
    pub dma_w_block_num_reg: u8,
    /// DMA write: the data in RAM starting at this address overwrites the chosen drive block.
    pub dma_w_ram_start_reg: u32,
    /// The error register. Bits are toggled on when an error is triggered, and bits are toggled
    /// off when errors are consumed.
    pub err_reg: u8,
    /// The keyboard I/O register. 256 bits. Bits are toggled on/off then their respective keys are
    /// pressed/released.
    pub kb_reg: KbReg,
    /// The interrupt enable register. Serves as a bitmask for the interrupt register.
    pub ie_register: u8,
    /// The interrupt register. Denotes which interrupts have been triggered.
    pub interrupt_register: u8,
    /// The number of cycles until the current DMA read is complete. If 0, then no DMA read is
    /// currently underway.
    pub dma_read_cycles_remaining: usize,
    /// The number of cycles until the current DMA write is complete. If 0, then no DMA write is
    /// currently underway.
    pub dma_write_cycles_remaining: usize,
    /// The current block being read to or written from in the DMA transfer.
    pub current_dma_block: [u8; BLOCK_SIZE],
    /// If true, print debug messages to stderr.
    pub debug: bool,
}
impl Mmu {
    /// Create a new memory management unit.
    pub fn new() -> Self {
        Self {
            rom: Memory::new_empty(ROM_SIZE, true, false),
            ram: Memory::new_empty(RAM_SIZE, true, true),
            ..Self::default()
        }
    }

    /// Perform one clock cycle.
    pub fn cycle(&mut self, drives: &mut [Drive]) {
        if self.dma_read_cycles_remaining > 0 {
            if self.dma_read_cycles_remaining == DMA_TRANSFER_CYCLES {
                // Emulate DMA by simply reading the whole block into a temporary buffer on the
                // first cycle of a new DMA read
                if let Some(drive) = drives
                    .iter_mut()
                    .find(|drive| drive.drive_number() == self.dma_r_drive_num_reg)
                {
                    // Set drive as busy
                    if drive.set_flag(DriveFlag::Busy).is_err() {
                        self.illegal_write(DRIVE_FLAGS_ADDR as u32, "set busy header flag")
                    }
                    drive.read_block(self.dma_r_block_num_reg.into(), &mut self.current_dma_block);
                }
            } else if self.dma_read_cycles_remaining == 1
                && (self.dma_r_ram_start_reg as usize) > RAM_OFFSET
            {
                // Emulate DMA by simply writing the whole block directly into memory

                // Don't go out of bounds!
                let relative_start = (self.dma_r_ram_start_reg as usize) - RAM_OFFSET;
                let max_copy_len = RAM_SIZE.saturating_sub(relative_start);
                let copy_len = BLOCK_SIZE.min(max_copy_len);
                if copy_len > 0 {
                    self.ram
                        .direct_write(relative_start as u32, &self.current_dma_block[..copy_len]);
                }

                if (self.dma_r_ram_start_reg as usize) + BLOCK_SIZE > RAM_END {
                    // Only write until the end of RAM
                    self.ram.direct_write(
                        self.dma_r_ram_start_reg - (RAM_OFFSET as u32),
                        &self.current_dma_block[..],
                    );
                }
                // Set drive as not busy
                if let Some(drive) = drives
                    .iter_mut()
                    .find(|drive| drive.drive_number() == self.dma_r_drive_num_reg)
                {
                    // Set drive as busy
                    if drive.reset_flag(DriveFlag::Busy).is_err() {
                        self.illegal_write(DRIVE_FLAGS_ADDR as u32, "reset busy header flag")
                    }
                }
            }
            self.dma_read_cycles_remaining -= 1;
        } else if self.dma_write_cycles_remaining > 0 {
            self.dma_write_cycles_remaining -= 1;
        }
    }

    /// Check if the MMU is locked.
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        (self.dma_read_cycles_remaining > 0) || (self.dma_write_cycles_remaining > 0)
    }

    /// Enable debug mode.
    pub fn enable_debug(&mut self) {
        self.debug = true;
        self.rom.debug = true;
        self.ram.debug = true;
        self.kb_reg.debug = true;
    }

    /// Set an [Interrupt].
    pub fn set_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt_register |= 1 << interrupt.into_byte();
    }

    /// Set an [MfsError], triggering an [Interrupt::Error] interrupt if a new error occurs.
    pub fn set_error(&mut self, mfs_error: MfsError) {
        let old_val = self.err_reg;
        self.err_reg |= 1 << mfs_error.into_byte();
        if old_val != self.err_reg {
            self.set_interrupt(Interrupt::Error);
        }
    }

    /// Consume the error register, returning the value and resetting all the register bits.
    fn consume_err_reg(&mut self) -> u8 {
        let result = self.err_reg;
        self.err_reg = 0;
        result
    }

    /// Start a DMA read. The MMU is ticked before the CPU, so 1 must be added so the CPU cannot
    /// access the MMU for the entire DMA time.
    fn dma_read(&mut self) {
        self.dma_read_cycles_remaining = DMA_TRANSFER_CYCLES + 1;
    }

    /// Start a DMA write. The MMU is ticked before the CPU, so 1 must be added so the CPU cannot
    /// access the MMU for the entire DMA time.
    fn dma_write(&mut self) {
        self.dma_write_cycles_remaining = DMA_TRANSFER_CYCLES + 1;
    }

    /// Read a byte from a given address.
    pub fn read_byte(&mut self, address: u32) -> u8 {
        if self.is_locked() {
            return self.illegal_read(address, "read a byte while locked");
        }

        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_readable() => {
                self.rom.read_byte(address - ROM_OFFSET as u32)
            }
            RAM_OFFSET..RAM_END if self.ram.is_readable() => {
                self.ram.read_byte(address - RAM_OFFSET as u32)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.read_byte(address - VRAM_OFFSET as u32),
            DMA_R_DRIVE_NUM_ADDR => self.dma_r_drive_num_reg,
            DMA_R_BLOCK_ADDR => self.dma_r_block_num_reg,
            DMA_W_DRIVE_NUM_ADDR => self.dma_w_drive_num_reg,
            DMA_W_BLOCK_ADDR => self.dma_w_block_num_reg,
            ERR_REG_ADDR => self.consume_err_reg(),
            KB_REG_START..KB_REG_END => self.kb_reg.read_byte(address - KB_REG_START as u32),
            IE_REGISTER_ADDR => self.ie_register,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register,
            _ => self.illegal_read(address, "read a byte"),
        }
    }

    /// Write a byte to a given address.
    pub fn write_byte(&mut self, address: u32, value: u8) {
        if self.is_locked() {
            return self.illegal_write(address, "write a byte while locked");
        }

        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_writable() => {
                self.rom.write_byte(address - ROM_OFFSET as u32, value)
            }
            RAM_OFFSET..RAM_END if self.ram.is_writable() => {
                self.ram.write_byte(address - RAM_OFFSET as u32, value)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.write_byte(address - VRAM_OFFSET as u32, value),
            DMA_R_INIT_ADDR => self.dma_read(),
            DMA_R_DRIVE_NUM_ADDR => self.dma_r_drive_num_reg = value,
            DMA_R_BLOCK_ADDR => self.dma_r_block_num_reg = value,
            DMA_W_INIT_ADDR => self.dma_write(),
            DMA_W_DRIVE_NUM_ADDR => self.dma_w_drive_num_reg = value,
            DMA_W_BLOCK_ADDR => self.dma_w_block_num_reg = value,
            MAN_FRAME_UPDATE_ADDR => self.gpu.set_frame_update_flag(),
            MAN_FRAME_DISABLE_ADDR => self.gpu.man_frame_disable(),
            MAN_FRAME_ENABLE_ADDR => self.gpu.man_frame_enable(),
            IE_REGISTER_ADDR => self.ie_register = value,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register = value,
            _ => self.illegal_write(address, "write a byte"),
        };
    }

    /// Read a word starting at a given address.
    pub fn read_word(&mut self, address: u32) -> u16 {
        if self.is_locked() {
            return self.illegal_read(address, "read a word while locked");
        }

        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_readable() => {
                self.rom.read_word(address - ROM_OFFSET as u32)
            }
            RAM_OFFSET..RAM_END if self.ram.is_readable() => {
                self.ram.read_word(address - RAM_OFFSET as u32)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.read_word(address - VRAM_OFFSET as u32),
            ERR_REG_ADDR => self.consume_err_reg() as u16,
            IE_REGISTER_ADDR => self.ie_register as u16,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register as u16,
            _ => self.illegal_read(address, "read a word"),
        }
    }

    /// Write a word to a given address.
    pub fn write_word(&mut self, address: u32, value: u16) {
        if self.is_locked() {
            return self.illegal_write(address, "write a word while locked");
        }

        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_writable() => {
                self.rom.write_word(address - ROM_OFFSET as u32, value)
            }
            RAM_OFFSET..RAM_END if self.ram.is_writable() => {
                self.ram.write_word(address - RAM_OFFSET as u32, value)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.write_word(address - VRAM_OFFSET as u32, value),
            MAN_FRAME_UPDATE_ADDR => self.gpu.set_frame_update_flag(),
            MAN_FRAME_DISABLE_ADDR => self.gpu.man_frame_disable(),
            MAN_FRAME_ENABLE_ADDR => self.gpu.man_frame_enable(),
            IE_REGISTER_ADDR => self.ie_register = value as u8,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register = value as u8,
            _ => self.illegal_write(address, "write a word"),
        };
    }

    /// Read a double word starting at a given address.
    pub fn read_dword(&mut self, address: u32) -> u32 {
        if self.is_locked() {
            return self.illegal_read(address, "read a double word while locked");
        }

        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_readable() => {
                self.rom.read_dword(address - ROM_OFFSET as u32)
            }
            RAM_OFFSET..RAM_END if self.ram.is_readable() => {
                self.ram.read_dword(address - RAM_OFFSET as u32)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.read_dword(address - VRAM_OFFSET as u32),
            DMA_R_RAM_ADDR_START => self.dma_r_ram_start_reg,
            DMA_W_RAM_ADDR_START => self.dma_w_ram_start_reg,
            ERR_REG_ADDR => self.consume_err_reg() as u32,
            IE_REGISTER_ADDR => self.ie_register as u32,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register as u32,
            _ => self.illegal_read(address, "read a double word"),
        }
    }

    /// Write a double word to a given address.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        if self.is_locked() {
            return self.illegal_write(address, "write a double word while locked");
        }

        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_writable() => {
                self.rom.write_dword(address - ROM_OFFSET as u32, value)
            }
            RAM_OFFSET..RAM_END if self.ram.is_writable() => {
                self.ram.write_dword(address - RAM_OFFSET as u32, value)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.write_dword(address - VRAM_OFFSET as u32, value),
            DMA_R_RAM_ADDR_START => self.dma_r_ram_start_reg = value,
            DMA_W_RAM_ADDR_START => self.dma_w_ram_start_reg = value,
            MAN_FRAME_UPDATE_ADDR => self.gpu.set_frame_update_flag(),
            MAN_FRAME_DISABLE_ADDR => self.gpu.man_frame_disable(),
            MAN_FRAME_ENABLE_ADDR => self.gpu.man_frame_enable(),
            IE_REGISTER_ADDR => self.ie_register = value as u8,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register = value as u8,
            _ => self.illegal_write(address, "write a double word"),
        };
    }

    /// Write a double word to VRAM only.
    pub fn write_dword_vram(&mut self, address: u32, value: u32) {
        if self.is_locked() {
            return self.illegal_write(address, "VRAM write while locked");
        }

        match address.try_into().unwrap() {
            VRAM_OFFSET..VRAM_END => self.gpu.write_dword(address - VRAM_OFFSET as u32, value),
            _ => self.illegal_write(address, "VRAM write outside of VRAM"),
        }
    }

    /// What to do when an illegal write is performed.
    fn illegal_write(&mut self, address: u32, msg: &'static str) {
        self.set_error(MfsError::IllegalWrite);
        print_warning_message(msg, address, self.debug);
    }

    /// What to do when an illegal read is performed.
    fn illegal_read<T: ErrVal>(&mut self, address: u32, msg: &'static str) -> T {
        self.set_error(MfsError::IllegalRead);
        print_warning_message(msg, address, self.debug);
        <T>::ERR_VAL
    }
}
impl Default for Mmu {
    fn default() -> Self {
        Self {
            rom: Memory::new_empty(ROM_SIZE, true, false),
            ram: Memory::new_empty(RAM_SIZE, true, true),
            gpu: Gpu::default(),
            dma_r_drive_num_reg: 0x00,
            dma_r_block_num_reg: 0x00,
            dma_r_ram_start_reg: 0x0000_0000,
            dma_w_drive_num_reg: 0x00,
            dma_w_block_num_reg: 0x00,
            dma_w_ram_start_reg: 0x0000_0000,
            err_reg: 0x00,
            kb_reg: KbReg::default(),
            ie_register: 0x00,
            interrupt_register: 0x00,
            dma_read_cycles_remaining: 0,
            dma_write_cycles_remaining: 0,
            current_dma_block: [0x00; BLOCK_SIZE],
            debug: false,
        }
    }
}

trait ErrVal {
    /// The value that represents an "error" value for that type when reading from memory.
    const ERR_VAL: Self;
}
macro_rules! impl_err_val {
    ($($t:ty),+) => {
        $(impl ErrVal for $t {
            const ERR_VAL: $t = <$t>::MAX;
        })+
    }
}
impl_err_val!(u8, u16, u32);

/// Print a warning message if debugging is allowed.
pub fn print_warning_message(verb: &'static str, address: u32, debug: bool) {
    if debug {
        eprintln!(
            "MMU Warning: failed to {} at address {:#010X}.",
            verb, address
        );
    }
}

/// All the different errors recognised by the MFS-16.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MfsError {
    /// This bit is set when an illegal memory read is performed.
    IllegalRead,
    /// This bit is set when an illegal memory write is performed.
    IllegalWrite,
}
impl MfsError {
    /// Get the [MfsError] matching the given byte, panicking if an invalid number is given.
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::IllegalRead,
            1 => Self::IllegalWrite,
            _ => panic!("{byte} does not match a valid MfsError variant."),
        }
    }

    /// Get the byte matching the given [MfsError].
    pub fn into_byte(self) -> u8 {
        match self {
            Self::IllegalRead => 0,
            Self::IllegalWrite => 1,
        }
    }
}
impl Display for MfsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IllegalRead => "IllegalRead",
                Self::IllegalWrite => "IllegalWrite",
            }
        )
    }
}

/// All the different interrupts recognised by the MFS-16. Lower number = higher priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Interrupt {
    /// This interrupt is regularly activated after the amount of cycles in one frame have passed.
    Frame,
    /// This interrupt is activated if any keyboard keys are pressed.
    Keyboard,
    /// This interrupt is activated if any errors occur.
    Error,
}
impl Interrupt {
    /// Get the [Interrupt] matching the given byte, panicking if an invalid number is given.
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::Frame,
            1 => Self::Keyboard,
            2 => Self::Error,
            _ => panic!("{byte} does not match a valid Interrupt variant."),
        }
    }

    /// Get the byte matching the given [Interrupt].
    pub fn into_byte(self) -> u8 {
        match self {
            Self::Frame => 0,
            Self::Keyboard => 1,
            Self::Error => 2,
        }
    }
}
impl Display for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Frame => "Frame",
                Self::Keyboard => "Keyboard",
                Self::Error => "Error",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    macro_rules! impl_checks {
        ($chk_name:ident, $t:ty, $mmu_w_fn:ident, $mmu_r_fn:ident) => {
            fn $chk_name(mmu: &mut Mmu, addr: usize, val: $t) {
                mmu.$mmu_w_fn(addr as u32, val);
                assert_eq!(mmu.$mmu_r_fn(addr as u32), val);
                assert_eq!(mmu.err_reg, 0);
            }
        };
    }

    impl_checks!(check_write_byte, u8, write_byte, read_byte);
    // impl_checks!(check_write_word, u16, write_word, read_word);
    impl_checks!(check_write_dword, u32, write_dword, read_dword);

    fn assert_dma_off(mmu: &mut Mmu) {
        assert_eq!(mmu.dma_write_cycles_remaining, 0);
        assert_eq!(mmu.dma_read_cycles_remaining, 0);
        assert!(!mmu.is_locked());
    }

    fn assert_mmu_locked(mmu: &mut Mmu) {
        mmu.write_byte(RAM_OFFSET as u32, 0xAB);
        assert!(mmu.consume_err_reg() != 0);
        mmu.write_word(RAM_OFFSET as u32, 0xABCD);
        assert!(mmu.consume_err_reg() != 0);
        mmu.write_dword(RAM_OFFSET as u32, 0xABCD_EF01);
        assert!(mmu.consume_err_reg() != 0);

        assert_eq!(mmu.read_byte(RAM_OFFSET as u32), <u8>::ERR_VAL);
        assert!(mmu.consume_err_reg() != 0);
        assert_eq!(mmu.read_word(RAM_OFFSET as u32), <u16>::ERR_VAL);
        assert!(mmu.consume_err_reg() != 0);
        assert_eq!(mmu.read_dword(RAM_OFFSET as u32), <u32>::ERR_VAL);
        assert!(mmu.consume_err_reg() != 0);

        assert!(mmu.is_locked());
    }

    fn assert_mmu_unlocked(mmu: &mut Mmu) {
        const EXPECTED_BYTE: u8 = 0xAB;
        const EXPECTED_WORD: u16 = 0xABCD;
        const EXPECTED_DWORD: u32 = 0xABCD_EF01;

        assert_eq!(mmu.read_byte(RAM_OFFSET as u32), 0);
        assert!(mmu.consume_err_reg() == 0);
        assert_eq!(mmu.read_word(RAM_OFFSET as u32), 0);
        assert!(mmu.consume_err_reg() == 0);
        assert_eq!(mmu.read_dword(RAM_OFFSET as u32), 0);
        assert!(mmu.consume_err_reg() == 0);

        mmu.write_byte(RAM_OFFSET as u32, EXPECTED_BYTE);
        assert!(mmu.consume_err_reg() == 0);
        assert_eq!(mmu.read_byte(RAM_OFFSET as u32), EXPECTED_BYTE);
        assert!(mmu.consume_err_reg() == 0);

        mmu.write_word(RAM_OFFSET as u32, EXPECTED_WORD);
        assert!(mmu.consume_err_reg() == 0);
        assert_eq!(mmu.read_word(RAM_OFFSET as u32), EXPECTED_WORD);
        assert!(mmu.consume_err_reg() == 0);

        mmu.write_dword(RAM_OFFSET as u32, EXPECTED_DWORD);
        assert!(mmu.consume_err_reg() == 0);
        assert_eq!(mmu.read_dword(RAM_OFFSET as u32), EXPECTED_DWORD);
        assert!(mmu.consume_err_reg() == 0);

        assert!(!mmu.is_locked());
    }

    #[test]
    fn test_dma_regs() {
        let mut mmu = Mmu::default();

        assert_dma_off(&mut mmu);

        check_write_byte(&mut mmu, DMA_R_DRIVE_NUM_ADDR, 0x01);
        check_write_byte(&mut mmu, DMA_R_BLOCK_ADDR, 0x23);
        check_write_dword(&mut mmu, DMA_R_RAM_ADDR_START, 0x0080_1234);

        assert_dma_off(&mut mmu);

        check_write_byte(&mut mmu, DMA_W_DRIVE_NUM_ADDR, 0x45);
        check_write_byte(&mut mmu, DMA_W_BLOCK_ADDR, 0x67);
        check_write_dword(&mut mmu, DMA_W_RAM_ADDR_START, 0x0080_5678);

        assert_dma_off(&mut mmu);

        // Read-only
        assert_eq!(mmu.read_byte(DMA_W_INIT_ADDR as u32), <u8>::ERR_VAL);
        assert!(mmu.consume_err_reg() != 0);
        assert_eq!(mmu.read_byte(DMA_R_INIT_ADDR as u32), <u8>::ERR_VAL);
        assert!(mmu.consume_err_reg() != 0);
    }

    #[test]
    fn test_dma() {
        let mut mmu = Mmu::default();
        let mut empty_drives: Vec<Drive> = Vec::new();

        assert_dma_off(&mut mmu);

        // Test DMA write
        assert_eq!(mmu.dma_write_cycles_remaining, 0);
        mmu.write_byte(DMA_W_INIT_ADDR as u32, 0x00);

        assert_eq!(mmu.dma_read_cycles_remaining, 0);
        assert_eq!(mmu.dma_write_cycles_remaining, DMA_TRANSFER_CYCLES + 1);
        assert!(mmu.is_locked());

        for _ in 0..DMA_TRANSFER_CYCLES {
            mmu.cycle(&mut empty_drives);
            assert_mmu_locked(&mut mmu);
        }
        mmu.cycle(&mut empty_drives);
        assert_mmu_unlocked(&mut mmu);
        assert_eq!(mmu.dma_write_cycles_remaining, 0);

        // Reset
        mmu.write_dword(RAM_OFFSET as u32, 0);

        // Test DMA read
        assert_eq!(mmu.dma_read_cycles_remaining, 0);
        mmu.write_byte(DMA_R_INIT_ADDR as u32, 0x00);

        assert_eq!(mmu.dma_write_cycles_remaining, 0);
        assert_eq!(mmu.dma_read_cycles_remaining, DMA_TRANSFER_CYCLES + 1);
        assert!(mmu.is_locked());
        for i in 0..DMA_TRANSFER_CYCLES {
            mmu.cycle(&mut empty_drives);
            assert_mmu_locked(&mut mmu);
            println!("MMU was locked properly for CPU cycle {}.", i + 1);
        }
        mmu.cycle(&mut empty_drives);
        assert_mmu_unlocked(&mut mmu);
        assert_eq!(mmu.dma_read_cycles_remaining, 0);
    }

    #[test]
    fn test_set_interrupt() {
        let mut mmu = Mmu::default();

        assert_eq!(mmu.interrupt_register, 0b0000_0000);
        mmu.set_interrupt(Interrupt::Frame);
        assert_eq!(mmu.interrupt_register, 0b0000_0001);
        mmu.set_interrupt(Interrupt::Keyboard);
        assert_eq!(mmu.interrupt_register, 0b0000_0011);
    }

    #[test]
    fn test_mmu() {
        let mut mmu = Mmu::default();

        // Test ROM, RAM, and VRAM
        for i in 0..(VRAM_END as u32) {
            mmu.write_byte(i, i as u8);
        }

        for i in 0..(VRAM_END as u32) {
            if ((i as usize) < ROM_END) && ((i as usize) >= ROM_OFFSET) {
                assert_eq!(mmu.read_byte(i), 0);
            } else if ((i as usize) < RAM_END) && ((i as usize) >= RAM_OFFSET) {
                assert_eq!(mmu.read_byte(i), i as u8);
            } else if ((i as usize) < VRAM_END) && ((i as usize) >= VRAM_OFFSET) {
                assert_eq!(mmu.read_byte(i), i as u8);
            }
        }
    }
}
