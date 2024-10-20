//! All the Arithmetic Logic Unit (ALU) instructions.
use std::ops::BitAnd;

use crate::{
    cpu::{
        flag::{Msb, Oneable, Zeroable},
        Cpu,
    },
    Flag::*,
};

use AluOp::*;

/// All the operations the ALU can perform.
pub enum AluOp {
    Add,
    Adc,
    Sub,
    Sbb,
    Tcp,
}

/// ALU function. Take two integer operands and the operation as input. Produce the integer result
/// and set CPU flags accordingly.
pub fn alu<T>(cpu: &mut Cpu, operation: AluOp, a: T, b: T) -> T
where
    T: Zeroable
        + Oneable
        + Msb
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + BitAnd
        + WrappingAdd
        + WrappingSub
        + NMinus1Mask
        + AsLargerType
        + HasMax,
    <T as BitAnd>::Output: PartialEq<T> + Into<T>,
    <T as AsLargerType>::Output: std::ops::Add + PartialOrd,
    <<T as AsLargerType>::Output as std::ops::Add>::Output: Into<<T as AsLargerType>::Output>,
{
    match operation {
        Add => alu_add(cpu, a, b, false),
        Adc => alu_add(cpu, a, b, true),
        Sub => alu_sub(cpu, a, b, false),
        Sbb => alu_sub(cpu, a, b, true),
        Tcp => alu_tcp(cpu, a),
    }
}

fn alu_add<T>(cpu: &mut Cpu, a: T, b: T, use_carry: bool) -> T
where
    T: Zeroable
        + Oneable
        + Msb
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + BitAnd
        + WrappingAdd
        + NMinus1Mask
        + AsLargerType
        + HasMax,
    <T as BitAnd>::Output: PartialEq<T> + Into<T>,
    <T as AsLargerType>::Output: std::ops::Add + PartialOrd,
    <<T as AsLargerType>::Output as std::ops::Add>::Output: Into<<T as AsLargerType>::Output>,
{
    let c = get_carry::<T>(cpu, use_carry);

    let result = a.trait_wrapping_add(b).trait_wrapping_add(c);

    let lower_n_minus_1_bits = T::n_minus_1_mask();
    let n_carry = ((a.as_larger_type() + b.as_larger_type()).into() + c.as_larger_type()).into()
        > T::get_max().as_larger_type();
    let n_minus_1_carry = (a & lower_n_minus_1_bits)
        .into()
        .trait_wrapping_add((b & lower_n_minus_1_bits).into())
        .trait_wrapping_add(c)
        > lower_n_minus_1_bits;
    set_add_sub_flags(cpu, n_minus_1_carry, n_carry, result);

    result
}

fn alu_sub<T>(cpu: &mut Cpu, a: T, b: T, use_carry: bool) -> T
where
    T: Zeroable
        + Oneable
        + Msb
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + BitAnd
        + WrappingSub
        + WrappingAdd
        + NMinus1Mask,
    <T as BitAnd>::Output: PartialEq<T> + Into<T>,
{
    let c = get_carry::<T>(cpu, use_carry);

    let result = a.trait_wrapping_sub(b).trait_wrapping_sub(c);

    let lower_n_minus_1_bits = T::n_minus_1_mask();
    let n_carry = a < b.trait_wrapping_add(c);
    let n_minus_1_carry =
        (a & lower_n_minus_1_bits).into() < (b & lower_n_minus_1_bits).into().trait_wrapping_add(c);
    set_add_sub_flags(cpu, n_minus_1_carry, n_carry, result);

    result
}

fn alu_tcp<T>(cpu: &mut Cpu, a: T) -> T
where
    T: Zeroable
        + Oneable
        + Msb
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + BitAnd
        + WrappingSub
        + WrappingAdd
        + NMinus1Mask,
    <T as BitAnd>::Output: PartialEq<T> + Into<T>,
{
    let result = alu_sub(cpu, <T>::zero(), a, false);
    cpu.flags.change_flag(Carry, a != <T>::zero());
    result
}

fn get_carry<T>(cpu: &Cpu, use_carry: bool) -> T
where
    T: Oneable + Zeroable,
{
    if use_carry && cpu.flag(Carry) {
        T::one()
    } else {
        T::zero()
    }
}

fn set_add_sub_flags<T>(cpu: &mut Cpu, n_minus_1_carry: bool, n_carry: bool, result: T)
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

// ------- TRAITS -------

/// Implementors of this trait have a wrapping_add method.
pub trait WrappingAdd {
    fn trait_wrapping_add(&self, rhs: Self) -> Self;
}
macro_rules! impl_wrapping_add {
    ($($t:ty),+) => {
        $(impl WrappingAdd for $t {
            fn trait_wrapping_add(&self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        })*
    }
}
impl_wrapping_add!(u8, u16, u32, u64, u128);

/// Implementors of this trait have a wrapping_sub method.
pub trait WrappingSub {
    fn trait_wrapping_sub(&self, rhs: Self) -> Self;
}
macro_rules! impl_wrapping_sub {
    ($($t:ty),+) => {
        $(impl WrappingSub for $t {
            fn trait_wrapping_sub(&self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
        })*
    }
}
impl_wrapping_sub!(u8, u16, u32, u64, u128);

/// Implementors of this trait can get a bitmask of n-1 bits, where n is the number of bits the
/// given data type contains.
pub trait NMinus1Mask {
    fn n_minus_1_mask() -> Self;
}
macro_rules! impl_n_minus_1_mask {
    ($($t:ty),+) => {
        $(impl NMinus1Mask for $t {
            fn n_minus_1_mask() -> Self {
                !(<Self>::MAX & (0b1 << (<Self>::BITS - 1)))
            }
        })*
    }
}
impl_n_minus_1_mask!(u8, u16, u32, u64, u128);

/// Implementors of this trait are able to cast themselves as the larger version of that type.
pub trait AsLargerType {
    type Output;

    fn as_larger_type(&self) -> Self::Output;
}
macro_rules! impl_as_larger_type {
    ($(($t:ty, $t_large:ty)),+) => {
        $(impl AsLargerType for $t {
            type Output = $t_large;
            fn as_larger_type(&self) -> Self::Output {
                *self as Self::Output
            }
        })*
    }
}
impl_as_larger_type!((u8, u16), (u16, u32), (u32, u64), (u64, u128));

/// Implementors of this trait are able to return the maximum value of that type as a constant.
pub trait HasMax {
    fn get_max() -> Self;
}
macro_rules! impl_has_max {
    ($($t:ty),+) => {
        $(impl HasMax for $t {
            fn get_max() -> Self {
                <Self>::MAX
            }
        })*
    }
}
impl_has_max!(u8, u16, u32, u64, u128);

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_n_minus_1_mask() {
        assert_eq!(u8::n_minus_1_mask(), 0x7F);
        assert_eq!(u16::n_minus_1_mask(), 0x7FFF);
        assert_eq!(u32::n_minus_1_mask(), 0x7FFF_FFFF);
        assert_eq!(u64::n_minus_1_mask(), 0x7FFF_FFFF_FFFF_FFFF);
        assert_eq!(
            u128::n_minus_1_mask(),
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF
        );
    }
}
