use crate::{
    cpu::Cpu,
    drive::Drive,
    keyboard::KbCode,
    mmu::{Interrupt, Mmu},
    Addr,
};

/// System clock frequency: 33 554 432 Hz (~33.55 MHz)
pub const CLOCK_FREQ: u32 = 2_u32.pow(25);

/// ROM Size: 8 MiB (~8.39 MB)
pub const ROM_SIZE: usize = 0x80_0000;
/// Address range: [0x00_0000, 0x7F_FFFF]
pub const ROM_OFFSET: usize = 0x00_0000;
/// RAM Size: 8 MiB (~8.39 MB)
pub const RAM_SIZE: usize = 0x80_0000;
/// Address range: [0x80_0000, 0xFF_FFFF]
pub const RAM_OFFSET: usize = ROM_SIZE;
/// VRAM offset
pub const VRAM_OFFSET: usize = RAM_SIZE + ROM_SIZE;
/// Video RAM size in bytes
pub const VRAM_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT / 2;

/// Starting memory location of interrupt handlers
pub const INTERRUPT_HANDLERS_OFFSET: usize = 0x0000_0100;

/// Display height
pub const DISPLAY_HEIGHT: usize = 480;
/// Display width
pub const DISPLAY_WIDTH: usize = 640;

/// Drive block size
pub const BLOCK_SIZE: usize = 512;

/// The speed of DMA transfer
pub const DMA_BYTES_PER_CYCLE: usize = 4;

/// The MFS-16 virtual computer.
#[derive(Default, Debug)]
pub struct Computer {
    /// The central processing unit of the computer.
    pub cpu: Cpu,
    /// The memory management unit of the computer.
    pub mmu: Mmu,
    /// All the drives connected to the computer.
    drives: Vec<Drive>,
    /// The cycle counter.
    pub cycles: u128,
    /// Will print debug messages to stdout when true.
    pub debug: bool,
    /// Will print keyboard debug messages to stdout when true.
    pub kb_debug: bool,
}
impl Computer {
    /// The system clock frequency in Hz.
    pub const CLOCK_FREQ: u32 = CLOCK_FREQ;

    /// The system ROM size in bytes.
    pub const ROM_SIZE: usize = ROM_SIZE;

    /// The starting ROM address in bytes.
    pub const ROM_OFFSET: usize = ROM_OFFSET;

    /// The system RAM size in bytes.
    pub const RAM_SIZE: usize = RAM_SIZE;

    /// The VRAM offset.
    pub const VRAM_OFFSET: usize = VRAM_OFFSET;

    /// The VRAM size in bytes.
    pub const VRAM_SIZE: usize = VRAM_SIZE;

    /// The starting RAM address in bytes.
    pub const RAM_OFFSET: usize = RAM_OFFSET;

    /// The display height in pixels.
    pub const DISPLAY_HEIGHT: usize = DISPLAY_HEIGHT;

    /// The display width in pixels.
    pub const DISPLAY_WIDTH: usize = DISPLAY_WIDTH;

    /// The drive block size.
    pub const BLOCK_SIZE: usize = BLOCK_SIZE;

    /// The DMA transfer speed.
    pub const DMA_BYTES_PER_CYCLE: usize = DMA_BYTES_PER_CYCLE;

    /// Create a new [Computer] with empty memory.
    pub fn new(debug: bool) -> Self {
        Self {
            cpu: Cpu {
                debug,
                ..Default::default()
            },
            debug,
            ..Default::default()
        }
    }

    /// Insert a [Drive] into the computer, failing if a drive with that number already exists.
    pub fn insert_drive(&mut self, drive: Drive) -> Result<(), String> {
        if self.find_drive(drive.drive_number()).is_none() {
            return Err(format!(
                "Failed to insert drive: Drive with number {} already exists!",
                drive.drive_number()
            ));
        }

        self.drives.push(drive);
        Ok(())
    }

    /// Perform one clock cycle.
    pub fn cycle(&mut self) {
        self.mmu.cycle(&mut self.drives);
        self.cpu.cycle(&mut self.mmu);
        self.cycles += 1;
    }

    /// Load a slice of bytes directly into ROM starting at the given address, overwriting any
    /// existing data in that range.
    pub fn direct_write(&mut self, start: Addr, bytes: &[u8]) {
        self.mmu.rom.direct_write(start.into(), bytes);
    }

    /// Handle a pressed keyboard key.
    pub fn key_down<C: Into<u16> + Copy>(&mut self, code: C) {
        if !self.mmu.kb_reg.key(code) {
            self.mmu.set_interrupt(Interrupt::Keyboard);
        }
        self.mmu.kb_reg.key_down(code);
        if self.kb_debug {
            if let Some(kbc) = KbCode::try_from_u16(code.into()) {
                println!("`{}` pressed", kbc);
            }
        }
    }

    /// Handle a released keyboard key.
    pub fn key_up<C: Into<u16> + Copy>(&mut self, code: C) {
        self.mmu.kb_reg.key_up(code);
        if self.kb_debug {
            if let Some(kbc) = KbCode::try_from_u16(code.into()) {
                println!("`{}` released", kbc);
            }
        }
    }

    /// Find the [Drive] with the given drive number.
    pub fn find_drive(&self, drive_number: u8) -> Option<&Drive> {
        self.drives
            .iter()
            .find(|inserted_drive| inserted_drive.drive_number() == drive_number)
    }
}
