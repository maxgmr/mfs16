//! Helper functions for CPU instructions.
use crate::{
    cpu::Cpu,
    helpers::{combine_u16_be, split_dword},
    mmu::Mmu,
    Addr,
};

/// Generate a LFSR-based pseudorandom number.
pub fn lfsr_rand(cpu: &Cpu) -> u32 {
    let mut state = (cpu.total_cycles as u32)
        .rotate_left(cpu.pc.address())
        .wrapping_add(cpu.pc.address());
    for _ in 0..32 {
        let mut new_bit = state ^ (state >> 2) ^ (state >> 3) ^ (state >> 5);
        new_bit &= 1;
        state = (state >> 1) | (new_bit << 31);
    }
    state
}

/// Get 32-bit double word from the last two words read.
pub fn get_dword_from_last(cpu: &Cpu) -> u32 {
    combine_u16_be(cpu.last_word, cpu.second_last_word)
}

/// Write a little-endian 32-bit value to the address made from the last two words read.
pub fn write_dword_to_last(cpu: &Cpu, mmu: &mut Mmu, val: u32) {
    let mut addr = Addr::new_default_range(get_dword_from_last(cpu));
    let (msw, lsw) = split_dword(val);
    mmu.write_word(addr.address(), lsw);
    addr.wrapping_inc();
    addr.wrapping_inc();
    mmu.write_word(addr.address(), msw);
}
