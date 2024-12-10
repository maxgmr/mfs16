//! Helper functions for CPU instructions.
use crate::{
    cpu::Cpu,
    helpers::{combine_u16_be, split_dword},
    mmu::Mmu,
    Addr,
};

/// Generate a Xorshift-based pseudorandom number.
pub fn xorshift_rand(cpu: &Cpu) -> u32 {
    let mut state = (cpu.total_cycles as u32)
        + (cpu.reg(crate::Reg16::A) as u32)
        + (cpu.reg(crate::Reg16::B) as u32)
        + (cpu.reg(crate::Reg16::C) as u32)
        + (cpu.reg(crate::Reg16::D) as u32)
        + (cpu.reg(crate::Reg16::E) as u32)
        + (cpu.reg(crate::Reg16::H) as u32)
        + (cpu.reg(crate::Reg16::L) as u32);
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
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
