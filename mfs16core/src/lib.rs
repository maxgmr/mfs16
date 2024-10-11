//! Core library for backend mfs16 functionality.
#![warn(missing_docs)]

mod computer;
mod cpu;
mod helpers;
mod ram;

// Re-exports
pub use computer::{Computer, CLOCK_FREQ, RAM_SIZE};
