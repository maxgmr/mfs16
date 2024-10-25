//! All the Arithmetic Logic Unit (ALU) instructions.
use std::{
    fmt::{Binary, Display},
    ops::{Add as OpsAdd, BitAnd, BitOr, BitXor, Not as OpsNot, Rem, Shl, Shr, Sub as OpsSub},
};

use crate::{
    cpu::{
        flag::{Msb, Oneable, Zeroable},
        Cpu,
    },
    helpers::{n_least_significant, n_most_significant, test_bit},
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
    Inc,
    Dec,
    Pss,
    And,
    Or,
    Xor,
    Not,
    Asr,
    Asl,
    Lsr,
    Rtr,
    Rtl,
    Rcr,
    Rcl,
}

/// ALU function. Take two integer operands and the operation as input. Produce the integer result
/// and set CPU flags accordingly.
///
/// Argument 'b' is ignored for single-operand operations.
pub fn alu<T>(cpu: &mut Cpu, operation: AluOp, a: T, b: T) -> T
where
    T: Zeroable
        + Oneable
        + Msb
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + OpsNot<Output = T>
        + OpsAdd<Output = T>
        + WrappingAdd
        + OpsSub<Output = T>
        + WrappingSub
        + NMinus1Mask
        + Display
        + Binary
        + AsLargerType
        + HasMax
        + Shl<Output = T>
        + Shr<Output = T>
        + NumBits
        + Rem<Output = T>,
    <T as AsLargerType>::Output: std::ops::Add + PartialOrd,
    <<T as AsLargerType>::Output as std::ops::Add>::Output: Into<<T as AsLargerType>::Output>,
{
    match operation {
        Add => alu_add(cpu, a, b, false),
        Adc => alu_add(cpu, a, b, true),
        Sub => alu_sub(cpu, a, b, false),
        Sbb => alu_sub(cpu, a, b, true),
        Tcp => alu_tcp(cpu, a),
        Inc => alu_inc_dec(cpu, a, true),
        Dec => alu_inc_dec(cpu, a, false),
        Pss => alu_pss(cpu, a),
        And => alu_and(cpu, a, b),
        Or => alu_or(cpu, a, b),
        Xor => alu_xor(cpu, a, b),
        Not => alu_not(cpu, a),
        Asr => alu_shr(cpu, a, b, true),
        Asl => alu_shl(cpu, a, b),
        Lsr => alu_shr(cpu, a, b, false),
        Rtr => alu_rotate(cpu, a, b, false),
        Rtl => alu_rotate(cpu, a, b, true),
        Rcr => alu_rotate_carry(cpu, a, b, false),
        Rcl => alu_rotate_carry(cpu, a, b, true),
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
        + BitAnd<Output = T>
        + WrappingAdd
        + NMinus1Mask
        + AsLargerType
        + HasMax,
    <T as AsLargerType>::Output: std::ops::Add + PartialOrd,
    <<T as AsLargerType>::Output as std::ops::Add>::Output: Into<<T as AsLargerType>::Output>,
{
    let c = get_carry::<T>(cpu, use_carry);

    let result = a.trait_wrapping_add(b).trait_wrapping_add(c);

    let lower_n_minus_1_bits = T::n_minus_1_mask();
    let n_carry = ((a.as_larger_type() + b.as_larger_type()).into() + c.as_larger_type()).into()
        > T::get_max().as_larger_type();
    let n_minus_1_carry = (a & lower_n_minus_1_bits)
        .trait_wrapping_add(b & lower_n_minus_1_bits)
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
        + BitAnd<Output = T>
        + WrappingSub
        + WrappingAdd
        + NMinus1Mask,
{
    let c = get_carry::<T>(cpu, use_carry);

    let result = a.trait_wrapping_sub(b).trait_wrapping_sub(c);

    let lower_n_minus_1_bits = T::n_minus_1_mask();
    let n_carry = a < b.trait_wrapping_add(c);
    let n_minus_1_carry =
        (a & lower_n_minus_1_bits) < (b & lower_n_minus_1_bits).trait_wrapping_add(c);
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
        + BitAnd<Output = T>
        + WrappingSub
        + WrappingAdd
        + NMinus1Mask,
{
    let result = alu_sub(cpu, <T>::zero(), a, false);
    cpu.flags.change_flag(Carry, a != <T>::zero());
    result
}

fn alu_inc_dec<T>(cpu: &mut Cpu, a: T, is_inc: bool) -> T
where
    T: Zeroable
        + Oneable
        + Msb
        + Copy
        + PartialEq
        + PartialOrd
        + Ord
        + BitAnd<Output = T>
        + WrappingAdd
        + WrappingSub
        + NMinus1Mask
        + AsLargerType
        + HasMax,
    <T as AsLargerType>::Output: std::ops::Add + PartialOrd,
    <<T as AsLargerType>::Output as std::ops::Add>::Output: Into<<T as AsLargerType>::Output>,
{
    let original_carry = cpu.flag(Carry);
    let original_overflow = cpu.flag(Overflow);
    let original_negative = cpu.flag(Negative);

    let result = if is_inc {
        alu_add(cpu, a, <T>::one(), false)
    } else {
        alu_sub(cpu, a, <T>::one(), false)
    };

    cpu.change_flag(Carry, original_carry);
    cpu.change_flag(Overflow, original_overflow);
    cpu.change_flag(Negative, original_negative);

    result
}

fn alu_pss<T>(cpu: &mut Cpu, a: T) -> T
where
    T: Zeroable + Oneable + Msb + PartialEq + Eq + BitAnd<Output = T> + Copy,
{
    let original_carry = cpu.flag(Carry);
    let original_overflow = cpu.flag(Overflow);

    set_add_sub_flags(cpu, false, false, a);

    cpu.change_flag(Carry, original_carry);
    cpu.change_flag(Overflow, original_overflow);

    a
}

fn alu_and<T>(cpu: &mut Cpu, a: T, b: T) -> T
where
    T: Zeroable + Oneable + Msb + PartialEq + Eq + BitAnd<Output = T> + Copy,
{
    let result: T = a & b;
    set_bitwise_flags(cpu, result);
    result
}

fn alu_or<T>(cpu: &mut Cpu, a: T, b: T) -> T
where
    T: Zeroable + Oneable + Msb + PartialEq + Eq + BitAnd<Output = T> + BitOr<Output = T> + Copy,
{
    let result: T = a | b;
    set_bitwise_flags(cpu, result);
    result
}

fn alu_xor<T>(cpu: &mut Cpu, a: T, b: T) -> T
where
    T: Zeroable + Oneable + Msb + PartialEq + Eq + BitAnd<Output = T> + BitXor<Output = T> + Copy,
{
    let result: T = a ^ b;
    set_bitwise_flags(cpu, result);
    result
}

fn alu_not<T>(cpu: &mut Cpu, a: T) -> T
where
    T: OpsNot<Output = T> + Copy + Zeroable + Oneable + Msb + Eq + BitAnd<Output = T>,
{
    let result: T = !a;
    set_bitwise_flags(cpu, result);
    result
}

fn alu_shr<T>(cpu: &mut Cpu, a: T, b: T, preserve_msb: bool) -> T
where
    T: NumBits
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + PartialEq
        + PartialOrd
        + Zeroable
        + Oneable
        + Shl<Output = T>
        + Shr<Output = T>
        + Msb
        + HasMax
        + Copy
        + OpsSub<Output = T>,
{
    // Do nothing if shift by 0
    if b == <T>::zero() {
        return a;
    }

    if preserve_msb {
        // For arithmetic shift, Overflow flag always reset
        cpu.reset_flag(Overflow);
    } else {
        // For logical shift, Overflow flag set to MSB of original operand
        cpu.change_flag(Overflow, a.msb());
    }

    let msb = if preserve_msb && a.msb() {
        <T>::one()
    } else {
        <T>::zero()
    };

    if b > <T>::num_bits() {
        if msb == <T>::zero() {
            cpu.reset_flag(Carry);
            return <T>::zero();
        } else {
            cpu.set_flag(Carry);
            return <T>::get_max();
        }
    }

    if b == <T>::num_bits() {
        cpu.change_flag(Carry, a.msb());

        if msb == <T>::zero() {
            return <T>::zero();
        } else {
            return <T>::get_max();
        }
    }

    let result: T = (a >> b)
        | (if msb == <T>::zero() {
            <T>::zero()
        } else {
            <T>::get_max() << (<T>::num_bits() - b)
        });

    // Set carry flag iff last value shifted out was equal to 1
    cpu.change_flag(Carry, ((a >> (b - <T>::one())) & <T>::one()) == <T>::one());

    result
}

fn alu_shl<T>(cpu: &mut Cpu, a: T, b: T) -> T
where
    T: Zeroable
        + Oneable
        + NumBits
        + PartialEq
        + PartialOrd
        + Shl<Output = T>
        + Shr<Output = T>
        + OpsSub<Output = T>
        + NumBits
        + Copy
        + Msb,
{
    // Do nothing if shift by 0
    if b == <T>::zero() {
        return a;
    }

    if b > <T>::num_bits() {
        cpu.reset_flag(Carry);
        return <T>::zero();
    }

    let result: T = a << b;

    // Set carry flag iff last value shifted out was equal to 1
    cpu.change_flag(
        Carry,
        ((a << (b - <T>::one())) >> (<T>::num_bits() - <T>::one())) != <T>::zero(),
    );

    // Set overflow flag iff the result's MSB is different than the original MSB.
    cpu.change_flag(Overflow, result.msb() != a.msb());

    result
}

fn alu_rotate<T>(cpu: &mut Cpu, a: T, b: T, is_left: bool) -> T
where
    T: Rem<Output = T>
        + Oneable
        + Zeroable
        + PartialEq
        + NumBits
        + Copy
        + Msb
        + OpsSub<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Shl<Output = T>
        + Shr<Output = T>,
{
    let rotate_bits = b % <T>::num_bits();

    // Do nothing if rotate ends at same place
    if rotate_bits == <T>::zero() {
        return a;
    }

    let result = if is_left {
        (a << rotate_bits) | (a >> (<T>::num_bits() - rotate_bits))
    } else {
        (a >> rotate_bits) | (a << (<T>::num_bits() - rotate_bits))
    };

    // Set carry = last bit carried over to other side
    if is_left {
        cpu.change_flag(Carry, result & <T>::one() == <T>::one());
    } else {
        cpu.change_flag(Carry, result.msb());
    }

    // Set overflow iff MSB of result does not match MSB of operand
    if a.msb() != result.msb() {
        cpu.set_flag(Overflow);
    } else {
        cpu.reset_flag(Overflow);
    }

    result
}

fn alu_rotate_carry<T>(cpu: &mut Cpu, a: T, b: T, is_left: bool) -> T
where
    T: Rem<Output = T>
        + Zeroable
        + Oneable
        + PartialEq
        + PartialOrd
        + NumBits
        + HasMax
        + Msb
        + OpsNot<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + OpsAdd<Output = T>
        + OpsSub<Output = T>
        + Shl<Output = T>
        + Shr<Output = T>
        + Copy,
{
    let rotate_bits = b % (<T>::num_bits() + <T>::one());

    // Do nothing if rotate ends at same place
    if rotate_bits == <T>::zero() {
        return a;
    }

    let old_carry: T = if cpu.flag(Carry) {
        <T>::one()
    } else {
        <T>::zero()
    };

    let (new_carry, result) = if is_left {
        let first_bits: T = n_most_significant(a, rotate_bits - <T>::one());
        let new_carry: bool = test_bit(a, <T>::num_bits() - rotate_bits);
        let remaining_bits: T = n_least_significant(a, <T>::num_bits() - rotate_bits);

        (
            new_carry,
            first_bits
                | (old_carry << (rotate_bits - <T>::one()))
                | (remaining_bits << rotate_bits),
        )
    } else {
        let last_bits: T = n_least_significant(a, rotate_bits - <T>::one());
        let new_carry: bool = test_bit(a, rotate_bits - <T>::one());
        let remaining_bits: T = n_most_significant(a, <T>::num_bits() - rotate_bits);

        (
            new_carry,
            remaining_bits
                | (old_carry << (<T>::num_bits() - rotate_bits))
                | if rotate_bits > <T>::one() {
                    last_bits << ((<T>::num_bits() + <T>::one()) - rotate_bits)
                } else {
                    <T>::zero()
                },
        )
    };

    cpu.change_flag(Carry, new_carry);
    if a.msb() != result.msb() {
        cpu.set_flag(Overflow);
    } else {
        cpu.reset_flag(Overflow);
    }

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
    T: Zeroable + Oneable + Msb + PartialEq + Eq + BitAnd<Output = T> + Copy,
{
    cpu.flags.change_zero(result);
    cpu.flags.change_flag(Carry, n_carry);
    cpu.flags.change_flag(Overflow, n_minus_1_carry ^ n_carry);
    cpu.flags.change_parity(result);
    cpu.flags.change_negative(result);
}

fn set_bitwise_flags<T>(cpu: &mut Cpu, result: T)
where
    T: Zeroable + Copy + Eq + BitAnd<Output = T> + Oneable + Msb,
{
    cpu.flags.change_zero(result);
    cpu.flags.reset_flag(Carry);
    cpu.flags.reset_flag(Overflow);
    cpu.flags.change_parity(result);
    cpu.flags.change_negative(result);
}

// ------- TRAITS -------

/// Implementors of this trait have a wrapping_add method.
pub trait WrappingAdd {
    /// Does the same thing as wrapping_add() for primitives.
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
    /// Does the same thing as wrapping_sub() for primitives.
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
    /// Get a bitmask of n-1 bits for the given type, where n is the number of bits the given data
    /// type contains.
    fn n_minus_1_mask() -> Self;
}
macro_rules! impl_n_minus_1_mask {
    ($($t:ty),+) => {
        $(impl NMinus1Mask for $t {
            fn n_minus_1_mask() -> Self {
                !(<Self>::MAX & (1 << (<Self>::BITS - 1)))
            }
        })*
    }
}
impl_n_minus_1_mask!(u8, u16, u32, u64, u128);

/// Implementors of this trait are able to cast themselves as the larger version of that type.
pub trait AsLargerType {
    /// The larger version of the type.
    type Output;

    /// Get the value cast to the larger version of that type.
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
    /// Get the maximum value of this data type.
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

/// Implementors of this trait are able to return the number of bits they take up as a constant.
pub trait NumBits {
    /// Get the number of bits this data type contains.
    fn num_bits() -> Self;
}
macro_rules! impl_num_bits {
    ($($t:ty),+) => {
        $(impl NumBits for $t {
            fn num_bits() -> Self {
                <Self>::BITS as Self
            }
        })*
    }
}
impl_num_bits!(u8, u16, u32, u64, u128);

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

    #[test]
    fn test_num_bits() {
        assert_eq!(u8::num_bits(), 8_u8);
        assert_eq!(u16::num_bits(), 16_u16);
        assert_eq!(u32::num_bits(), 32_u32);
        assert_eq!(u64::num_bits(), 64_u64);
        assert_eq!(u128::num_bits(), 128_u128);
    }
}