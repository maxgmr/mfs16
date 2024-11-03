use crate::{cpu::Cpu, mmu::Mmu, Addr};

/// System clock frequency: 33_554_432 Hz (33.55 MHz)
pub const CLOCK_FREQ: u32 = 2_u32.pow(25);

/// ROM Size: 8 MiB (~8.39 MB)
pub const ROM_SIZE: usize = 0x80_0000;
/// Address range: [0x00_0000, 0x7F_FFFF]
pub const ROM_OFFSET: usize = 0x00_0000;
/// RAM Size: 8 MiB (~8.39 MB)
pub const RAM_SIZE: usize = 0x80_0000;
/// Address range: [0x80_0000, 0xFF_FFFF]
pub const RAM_OFFSET: usize = ROM_SIZE;

/// Display height
pub const DISPLAY_HEIGHT: usize = 240;
/// Display width
pub const DISPLAY_WIDTH: usize = 320;
/// Video RAM size in bytes
pub const VRAM_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT / 2;

/// The MFS-16 virtual computer.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Computer {
    /// The central processing unit of the computer.
    pub cpu: Cpu,
    /// The memory management unit of the computer.
    pub mmu: Mmu,
    /// The cycle counter.
    pub cycles: u128,
    /// Will print debug messages to stdout when true.
    pub debug: bool,
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

    /// The starting RAM address in bytes.
    pub const RAM_OFFSET: usize = RAM_OFFSET;

    /// The display height in pixels.
    pub const DISPLAY_HEIGHT: usize = DISPLAY_HEIGHT;

    /// The display width in pixels.
    pub const DISPLAY_WIDTH: usize = DISPLAY_WIDTH;

    /// The VRAM size in bytes.
    pub const VRAM_SIZE: usize = VRAM_SIZE;

    /// Create a new [Computer] with empty [Ram].
    pub fn new(debug: bool) -> Self {
        let mut cpu = Cpu::default();
        cpu.debug = debug;
        Self {
            cpu,
            debug,
            ..Self::default()
        }
    }

    /// Perform one clock cycle.
    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.mmu);
        self.cycles += 1;
    }

    /// Load a slice of bytes directly into ROM starting at the given address, overwriting any
    /// existing data in that range.
    pub fn direct_write(&mut self, start: Addr, bytes: &[u8]) {
        self.mmu.rom.direct_write(start.into(), bytes);
    }
}
