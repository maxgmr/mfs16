use camino::Utf8Path;
use color_eyre::eyre;
use mfs16core::Computer;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};
use std::{fs::OpenOptions, io::Write};

/// The number of bytes at and after program counter to store in the history.
pub const PC_BYTES_SIZE: usize = 16;

/// Responsible for debugging functionality.
#[derive(Debug, Clone)]
pub struct Debugger {
    /// The criteria for breaking.
    pub criteria: BreakCriteria,
    /// The different states of the computer over the last [HISTORY_SIZE] cycles.
    pub history: VecDeque<ComputerState>,
    /// The ranges of memory to log.
    mem_ranges: Vec<MemRange>,
    /// If true, only collect data on the CPU state.
    cpu_only: bool,
    /// The number of cycles to store in the history.
    history_size: usize,
}
impl Debugger {
    /// Create a new [Debugger] with the given [BreakCriteria] and [MemRange]s.
    pub fn new(
        criteria: BreakCriteria,
        mem_ranges: Vec<MemRange>,
        cpu_only: bool,
        history_size: usize,
    ) -> Self {
        Self {
            criteria,
            history: VecDeque::with_capacity(history_size),
            mem_ranges,
            cpu_only,
            history_size,
        }
    }

    /// Add the given [Computer]'s current state to history.
    pub fn add_state(&mut self, computer: &Computer) {
        if self.history.len() >= self.history_size {
            self.history.pop_front();
        }
        self.history.push_back(ComputerState::from_computer(
            computer,
            &self.mem_ranges,
            self.cpu_only,
        ));
    }

    /// Write the debugger results to the given [Utf8Path].
    pub fn write_to_file<P: AsRef<Utf8Path>>(&self, file_path: P) -> eyre::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path.as_ref())?;
        file.write_all(format!("{self}").as_bytes())?;
        Ok(())
    }
}
impl Display for Debugger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.history
                .iter()
                .map(|cs| format!("{}", cs))
                .collect::<Vec<String>>()
                .join(if self.cpu_only { "\n" } else { "\n\n\n" })
        )
    }
}

/// Responsible for debugger breakpoint criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakCriteria {
    // The list of program counter addresses which will satisfy the criteria.
    pub pc_list: Option<Vec<u32>>,
    // TODO add more options
}
impl BreakCriteria {
    /// Check to see whether the given [Computer]'s state satisfies the break criteria.
    pub fn is_satisfied(&self, computer: &Computer) -> bool {
        // Check whether computer program counter matches any PC options in the list
        if let Some(pc_list) = &self.pc_list {
            for pc in pc_list {
                if pc == &computer.cpu.pc.address() {
                    return true;
                }
            }
        }
        false
    }
}

/// Denotes a range start..end of memory.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MemRange {
    /// Inclusive start address of range.
    pub start: u32,
    /// Exclusive end address of range.
    pub end: u32,
}
impl MemRange {
    /// Get this range of memory from the given [Computer].
    fn grab(self, computer: &Computer) -> (Self, Vec<u8>) {
        let mut result = Vec::with_capacity((self.end - self.start) as usize);
        for address in self.start..self.end {
            result.push(computer.mmu.read_byte(address));
        }
        (self, result)
    }
}

/// A printable/stringable [Computer] state.
#[derive(Debug, Clone)]
pub struct ComputerState {
    /// The total number of completed cycles.
    num_cycles: Option<u128>,
    /// The CPU state.
    cpu_state: mfs16core::Cpu,
    /// The bytes at and after the PC.
    pc_bytes: Option<[u8; PC_BYTES_SIZE]>,
    /// User-defined ranges of memory.
    memory_ranges: Option<Vec<(MemRange, Vec<u8>)>>,
    /// Whether only the CPU string should be printed or not.
    cpu_only: bool,
}
impl ComputerState {
    /// Create a new [ComputerState] from a given [Computer] and [MemRange]s.
    fn from_computer(computer: &Computer, mem_ranges: &[MemRange], cpu_only: bool) -> Self {
        if cpu_only {
            Self {
                num_cycles: None,
                cpu_state: computer.cpu.clone(),
                pc_bytes: None,
                memory_ranges: None,
                cpu_only,
            }
        } else {
            Self {
                num_cycles: Some(computer.cycles),
                cpu_state: computer.cpu.clone(),
                pc_bytes: Some(Self::read_pc_bytes(computer)),
                memory_ranges: Some(mem_ranges.iter().map(|mr| mr.grab(computer)).collect()),
                cpu_only,
            }
        }
    }

    /// Read [PC_BYTES_SIZE] bytes from a given [Computer], starting at the [Computer]'s program
    /// counter.
    fn read_pc_bytes(computer: &Computer) -> [u8; PC_BYTES_SIZE] {
        let mut result = [0_u8; PC_BYTES_SIZE];
        for (index, item) in result.iter_mut().enumerate().take(PC_BYTES_SIZE) {
            *item = computer
                .mmu
                .read_byte(computer.cpu.pc.address() + (index as u32));
        }
        result
    }
}
impl Display for ComputerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cpu_only {
            return write!(f, "{}", self.cpu_state);
        }

        let formatted_pc_bytes = if let Some(pc_bytes) = self.pc_bytes {
            pc_bytes
                .into_iter()
                .map(|byte| format!("{:#04X}", byte))
                .collect::<Vec<String>>()
                .join(",")
        } else {
            String::new()
        };

        let formatted_memory_ranges = if let Some(memory_ranges) = &self.memory_ranges {
            memory_ranges
                .iter()
                .map(|(mr, bytes)| {
                    format!(
                        "\t-------{:#010X}..{:#010X}-------
\t\t{}",
                        mr.start,
                        mr.end,
                        bytes
                            .iter()
                            .map(|b| format!("{:#04X}", b))
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n")
        } else {
            String::new()
        };
        write!(
            f,
            "=======CYCLE {} START=======

\tBYTES: [<PC>,{}]

\tCPU:   [{}]

{}",
            self.num_cycles.unwrap_or(0),
            formatted_pc_bytes,
            self.cpu_state,
            formatted_memory_ranges,
        )
    }
}
