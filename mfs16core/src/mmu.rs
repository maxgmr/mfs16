//! Memory management unit. Responsible for memory reads and writes across all components of the
//! system.
use std::{default::Default, fmt::Display};

use crate::{
    gpu::Gpu,
    helpers::{combine_u16_le, combine_u8_le},
    keyboard::{KbReg, KB_REG_SIZE},
    memory::Memory,
    RAM_OFFSET, RAM_SIZE, ROM_OFFSET, ROM_SIZE, VRAM_OFFSET, VRAM_SIZE,
};

/// This byte is returned when the memory can't be read for any reason.
pub const NOT_READABLE_BYTE: u8 = 0xFF;

const ROM_END: usize = ROM_OFFSET + ROM_SIZE;
const RAM_END: usize = RAM_OFFSET + RAM_SIZE;
const VRAM_END: usize = VRAM_OFFSET + VRAM_SIZE;

/// Address of the error register.
pub const ERR_REG_ADDR: usize = 0xFFFF_FFBA;

/// Write to this address to send a manual frame update.
pub const MAN_FRAME_UPDATE_ADDR: usize = 0xFFFF_FFBB;
/// Write to this address to turn off manual frame updates.
pub const MAN_FRAME_DISABLE_ADDR: usize = 0xFFFF_FFBC;
/// Write to this address to turn on manual frame updates.
pub const MAN_FRAME_ENABLE_ADDR: usize = 0xFFFF_FFBD;

/// Start address of the keyboard register.
pub const KB_REG_START: usize = 0xFFFF_FFBE;

const KB_REG_END: usize = KB_REG_START + KB_REG_SIZE;

/// Address of the interrupt enable register.
pub const IE_REGISTER_ADDR: usize = 0xFFFF_FFFE;
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

    /// Read a byte from a given address.
    pub fn read_byte(&mut self, address: u32) -> u8 {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_readable() => {
                self.rom.read_byte(address - ROM_OFFSET as u32)
            }
            RAM_OFFSET..RAM_END if self.ram.is_readable() => {
                self.ram.read_byte(address - RAM_OFFSET as u32)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.read_byte(address - VRAM_OFFSET as u32),
            ERR_REG_ADDR => self.consume_err_reg(),
            KB_REG_START..KB_REG_END => self.kb_reg.read_byte(address - KB_REG_START as u32),
            IE_REGISTER_ADDR => self.ie_register,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register,
            _ => {
                self.set_error(MfsError::IllegalRead);
                print_warning_message("read a byte", address, self.debug);
                NOT_READABLE_BYTE
            }
        }
    }

    /// Write a byte to a given address.
    pub fn write_byte(&mut self, address: u32, value: u8) {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_writable() => {
                self.rom.write_byte(address - ROM_OFFSET as u32, value)
            }
            RAM_OFFSET..RAM_END if self.ram.is_writable() => {
                self.ram.write_byte(address - RAM_OFFSET as u32, value)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.write_byte(address - VRAM_OFFSET as u32, value),
            MAN_FRAME_UPDATE_ADDR => self.gpu.set_frame_update_flag(),
            MAN_FRAME_DISABLE_ADDR => self.gpu.man_frame_disable(),
            MAN_FRAME_ENABLE_ADDR => self.gpu.man_frame_enable(),
            IE_REGISTER_ADDR => self.ie_register = value,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register = value,
            _ => {
                self.set_error(MfsError::IllegalWrite);
                print_warning_message("write a byte", address, self.debug);
            }
        };
    }

    /// Read a word starting at a given address.
    pub fn read_word(&mut self, address: u32) -> u16 {
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
            _ => combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
        }
    }

    /// Write a word to a given address.
    pub fn write_word(&mut self, address: u32, value: u16) {
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
            _ => {
                self.set_error(MfsError::IllegalWrite);
                print_warning_message("write a word", address, self.debug);
            }
        };
    }

    /// Read a double word starting at a given address.
    pub fn read_dword(&mut self, address: u32) -> u32 {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_readable() => {
                self.rom.read_dword(address - ROM_OFFSET as u32)
            }
            RAM_OFFSET..RAM_END if self.ram.is_readable() => {
                self.ram.read_dword(address - RAM_OFFSET as u32)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.read_dword(address - VRAM_OFFSET as u32),
            ERR_REG_ADDR => self.consume_err_reg() as u32,
            IE_REGISTER_ADDR => self.ie_register as u32,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register as u32,
            _ => combine_u16_le(
                combine_u8_le(self.read_byte(address), self.read_byte(address + 1)),
                combine_u8_le(self.read_byte(address + 2), self.read_byte(address + 3)),
            ),
        }
    }

    /// Write a double word to a given address.
    pub fn write_dword(&mut self, address: u32, value: u32) {
        match address.try_into().unwrap() {
            ROM_OFFSET..ROM_END if self.rom.is_writable() => {
                self.rom.write_dword(address - ROM_OFFSET as u32, value)
            }
            RAM_OFFSET..RAM_END if self.ram.is_writable() => {
                self.ram.write_dword(address - RAM_OFFSET as u32, value)
            }
            VRAM_OFFSET..VRAM_END => self.gpu.write_dword(address - VRAM_OFFSET as u32, value),
            MAN_FRAME_UPDATE_ADDR => self.gpu.set_frame_update_flag(),
            MAN_FRAME_DISABLE_ADDR => self.gpu.man_frame_disable(),
            MAN_FRAME_ENABLE_ADDR => self.gpu.man_frame_enable(),
            IE_REGISTER_ADDR => self.ie_register = value as u8,
            INTERRUPT_REGISTER_ADDR => self.interrupt_register = value as u8,
            _ => {
                self.set_error(MfsError::IllegalWrite);
                print_warning_message("write a double word", address, self.debug);
            }
        };
    }

    /// Write a double word to VRAM only.
    pub fn write_dword_vram(&mut self, address: u32, value: u32) {
        match address.try_into().unwrap() {
            VRAM_OFFSET..VRAM_END => self.gpu.write_dword(address - VRAM_OFFSET as u32, value),
            _ => {
                self.set_error(MfsError::IllegalWrite);
                print_warning_message("VRAM write outside of VRAM", address, self.debug);
            }
        }
    }
}
impl Default for Mmu {
    fn default() -> Self {
        Self {
            rom: Memory::new_empty(ROM_SIZE, true, false),
            ram: Memory::new_empty(RAM_SIZE, true, true),
            gpu: Gpu::default(),
            err_reg: 0x00,
            kb_reg: KbReg::default(),
            ie_register: 0x00,
            interrupt_register: 0x00,
            debug: false,
        }
    }
}

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
