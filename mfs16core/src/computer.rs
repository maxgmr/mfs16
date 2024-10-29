use crate::{cpu::Cpu, ram::Ram, Addr};

/// System clock frequency: 33_554_432 Hz (33.55 MHz)
pub const CLOCK_FREQ: u32 = 2_u32.pow(25);

/// RAM Size: 16 MiB (~16.78 MB)
/// Address range: [0x00_0000, 0xFF_FFFF]
pub const RAM_SIZE: usize = 0x100_0000;

/// The MFS-16 virtual computer.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Computer {
    /// The CPU of the computer.
    pub cpu: Cpu,
    /// The RAM of the computer.
    pub ram: Ram,
    /// The cycle counter.
    pub cycles: u128,
}
impl Computer {
    /// Perform one clock cycle.
    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.ram);
        self.cycles += 1;
    }

    /// Load a slice of bytes directly into RAM starting at the given address, overwriting any
    /// existing data in that range.
    pub fn direct_write(&mut self, start: Addr, bytes: &[u8]) {
        self.ram.direct_write(start.into(), bytes);
    }
}
