//! Helper functions for CPU instructions.

use crate::{cpu::Cpu, helpers::combine_u16_be, Pc, Reg32};

/// Get 32-bit double word from the last two words read.
pub fn get_dword_from_last(cpu: &Cpu) -> u32 {
    combine_u16_be(cpu.last_word, cpu.second_last_word)
}

/// Increment the given big register twice.
pub fn dbl_inc_br(cpu: &mut Cpu, breg: Reg32) {
    inc_dec_br_helper(cpu, breg, true);
}

/// Decrement the given big register twice.
pub fn dbl_dec_br(cpu: &mut Cpu, breg: Reg32) {
    inc_dec_br_helper(cpu, breg, false);
}

fn inc_dec_br_helper(cpu: &mut Cpu, breg: Reg32, is_inc: bool) {
    let mut br_val = Pc::new(cpu.breg(breg));
    if is_inc {
        br_val.wrapping_inc();
        br_val.wrapping_inc();
    } else {
        br_val.wrapping_dec();
        br_val.wrapping_dec();
    }
    cpu.set_breg(breg, br_val.into());
}
