//! Helper functions for CPU instructions.
use crate::{
    cpu::Cpu,
    helpers::{combine_u16_be, split_dword},
    mmu::Mmu,
    Addr, Reg32,
};

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

/// Increment the given address twice.
pub fn dbl_inc_addr(cpu: &mut Cpu, breg: Reg32) {
    inc_dec_br_helper(cpu, breg, true);
}

/// Decrement the given address twice.
pub fn dbl_dec_addr(cpu: &mut Cpu, breg: Reg32) {
    inc_dec_br_helper(cpu, breg, false);
}

fn inc_dec_br_helper(cpu: &mut Cpu, breg: Reg32, is_inc: bool) {
    let mut br_val = Addr::new_default_range(cpu.breg(breg));
    if is_inc {
        br_val.wrapping_inc();
        br_val.wrapping_inc();
    } else {
        br_val.wrapping_dec();
        br_val.wrapping_dec();
    }
    cpu.set_breg(breg, br_val.into());
}
