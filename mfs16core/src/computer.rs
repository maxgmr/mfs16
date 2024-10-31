use crate::{cpu::Cpu, gpu::Gpu, ram::Ram, Addr};

/// System clock frequency: 33_554_432 Hz (33.55 MHz)
pub const CLOCK_FREQ: u32 = 2_u32.pow(25);

/// RAM Size: 16 MiB (~16.78 MB)
/// Address range: [0x00_0000, 0xFF_FFFF]
pub const RAM_SIZE: usize = 0x100_0000;

/// Display height
pub const DISPLAY_HEIGHT: usize = 240;
/// Display width
pub const DISPLAY_WIDTH: usize = 320;

/// The MFS-16 virtual computer.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Computer {
    /// The CPU of the computer.
    pub cpu: Cpu,
    /// The GPU of the computer.
    pub gpu: Gpu,
    /// The RAM of the computer.
    pub ram: Ram,
    /// The cycle counter.
    pub cycles: u128,
    /// Will print debug messages to stdout when true.
    pub debug: bool,
}
impl Computer {
    /// The system clock frequency in Hz.
    pub const CLOCK_FREQ: u32 = CLOCK_FREQ;

    /// The system RAM size in bytes.
    pub const RAM_SIZE: usize = RAM_SIZE;

    /// The display height in pixels.
    pub const DISPLAY_HEIGHT: usize = DISPLAY_HEIGHT;

    /// The display width in pixels.
    pub const DISPLAY_WIDTH: usize = DISPLAY_WIDTH;

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
        self.cpu.cycle(&mut self.gpu, &mut self.ram);
        self.cycles += 1;
    }

    /// Load a slice of bytes directly into RAM starting at the given address, overwriting any
    /// existing data in that range.
    pub fn direct_write(&mut self, start: Addr, bytes: &[u8]) {
        self.ram.direct_write(start.into(), bytes);
    }
}
