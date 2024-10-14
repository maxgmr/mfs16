//! Helper functions for CPU instructions.
use std::ops::BitAnd;

use crate::{
    cpu::{
        flag::{Msb, Oneable, Zeroable},
        Cpu,
    },
    helpers::{combine_u16_be, split_dword},
    ram::Ram,
    Flag::*,
    Pc, Reg16, Reg32, Reg8,
};

/// 16-bit addition with optional carry flag usage. Sets flags accordingly.
pub fn alu_add16(cpu: &mut Cpu, ra: Reg16, rb: Reg16, use_carry: bool) -> u16 {
    const LOWER_15_BITS: u16 = 0x7FFF;
    let a = cpu.reg(ra);
    let b = cpu.reg(rb);
    let c = if use_carry && cpu.flag(Carry) { 1 } else { 0 };

    let result = a.wrapping_add(b).wrapping_add(c);

    let bit_14_carry = (a & LOWER_15_BITS)
        .wrapping_add(b & LOWER_15_BITS)
        .wrapping_add(c)
        > LOWER_15_BITS;
    let bit_15_carry = ((a as u32) + (b as u32) + (c as u32)) > <u16>::MAX.into();
    cpu.flags.change_flag(Carry, bit_15_carry);
    set_add_sub_flags(cpu, a, b, bit_14_carry, bit_15_carry, result);

    result
}

/// 32-bit addition with optional carry flag usage. Sets flags accordingly.
pub fn alu_add32(cpu: &mut Cpu, bra: Reg32, brb: Reg32, use_carry: bool) -> u32 {
    const LOWER_31_BITS: u32 = 0x7FFF_FFFF;
    let a = cpu.breg(bra);
    let b = cpu.breg(brb);
    let c = if use_carry && cpu.flag(Carry) { 1 } else { 0 };

    let result = a.wrapping_add(b).wrapping_add(c);

    let bit_30_carry = (a & LOWER_31_BITS)
        .wrapping_add(b & LOWER_31_BITS)
        .wrapping_add(c)
        > LOWER_31_BITS;
    let bit_31_carry = ((a as u64) + (b as u64) + (c as u64)) > <u32>::MAX.into();
    cpu.flags.change_flag(Carry, bit_31_carry);
    set_add_sub_flags(cpu, a, b, bit_30_carry, bit_31_carry, result);

    result
}

/// 8-bit addition with optional carry flag usage. Sets flags accordingly.
pub fn alu_add8(cpu: &mut Cpu, vra: Reg8, vrb: Reg8, use_carry: bool) -> u8 {
    const LOWER_7_BITS: u8 = 0x7F;
    let a = cpu.vreg(vra);
    let b = cpu.vreg(vrb);
    let c = if use_carry && cpu.flag(Carry) { 1 } else { 0 };

    let result = a.wrapping_add(b).wrapping_add(c);

    let bit_6_carry = (a & LOWER_7_BITS)
        .wrapping_add(b & LOWER_7_BITS)
        .wrapping_add(c)
        > LOWER_7_BITS;
    let bit_7_carry = ((a as u16) + (b as u16) + (c as u16)) > <u8>::MAX.into();
    cpu.flags.change_flag(Carry, bit_7_carry);
    set_add_sub_flags(cpu, a, b, bit_6_carry, bit_7_carry, result);

    result
}

/// 16-bit subtraction with optional carry flag usage. Sets flags accordingly.
pub fn alu_sub16(cpu: &mut Cpu, ra: Reg16, rb: Reg16, use_carry: bool) -> u16 {
    let a = cpu.reg(ra);
    let b = cpu.reg(rb);
    let c = if use_carry && cpu.flag(Carry) { 1 } else { 0 };

    let result = a.wrapping_sub(b).wrapping_sub(c);

    cpu.flags
        .change_flag(Carry, (a as u32) < ((b as u32) + (c as u32)));

    result
}

/// 32-bit subtraction with optional carry flag usage. Sets flags accordingly.
pub fn alu_sub32(cpu: &mut Cpu, bra: Reg32, brb: Reg32, use_carry: bool) -> u32 {
    let a = cpu.breg(bra);
    let b = cpu.breg(brb);
    let c = if use_carry && cpu.flag(Carry) { 1 } else { 0 };

    let result = a.wrapping_sub(b).wrapping_sub(c);

    cpu.flags
        .change_flag(Carry, (a as u64) < ((b as u64) + (c as u64)));

    result
}

/// 8-bit subtraction with optional carry flag usage. Sets flags accordingly.
pub fn alu_sub8(cpu: &mut Cpu, vra: Reg8, vrb: Reg8, use_carry: bool) -> u8 {
    let a = cpu.vreg(vra);
    let b = cpu.vreg(vrb);
    let c = if use_carry && cpu.flag(Carry) { 1 } else { 0 };

    let result = a.wrapping_sub(b).wrapping_sub(c);

    cpu.flags
        .change_flag(Carry, (a as u16) < ((b as u16) + (c as u16)));

    result
}

fn set_add_sub_flags<T>(cpu: &mut Cpu, a: T, b: T, n_minus_1_carry: bool, n_carry: bool, result: T)
where
    T: Zeroable + Oneable + Msb + PartialEq + Eq + BitAnd + Copy,
    <T as BitAnd>::Output: PartialEq<T>,
{
    cpu.flags.change_zero(result);
    cpu.flags.change_flag(Carry, n_carry);
    cpu.flags.change_flag(Overflow, n_minus_1_carry ^ n_carry);
    cpu.flags.change_parity(result);
    cpu.flags.change_negative(result);
}
//
// fn check_overflow<T: Msb>(a: T, b: T, r: T) -> bool {
//     (a.msb() == b.msb()) && (r.msb() != a.msb())
// }

/// Get 32-bit double word from the last two words read.
pub fn get_dword_from_last(cpu: &Cpu) -> u32 {
    combine_u16_be(cpu.last_word, cpu.second_last_word)
}

/// Write a little-endian 32-bit value to the address made from the last two words read.
pub fn write_dword_to_last(cpu: &Cpu, ram: &mut Ram, val: u32) {
    let mut addr = Pc::new(get_dword_from_last(cpu));
    let (msw, lsw) = split_dword(val);
    ram.write_word(addr.into(), lsw);
    addr.wrapping_inc();
    addr.wrapping_inc();
    ram.write_word(addr.into(), msw);
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
