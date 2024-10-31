//! All the floating-point arithmetic instructions.
//! Represents a physical FPU (floating-point unit).
//!
//! The FPU can perform operations on 32-bit IEEE 754 floating-point representations.
//!
//! As of right now, the FPU is just a wrapper around the Rust standard library floating-point
//! operations.
use super::Cpu;
use crate::Flag::*;

use FpuOp::*;

/// All the operations the FPU can perform.
pub enum FpuOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// FPU function. Take two operands and the operation as input. Return the result.
///
/// Argument 'b' is ignored for single-operand operations.
///
/// Set Zero flag iff the result is zero.
/// Set Carry flag iff the result is infinite.
/// Set Overflow flag iff the result is NaN.
/// Set Parity flag iff the result is subnormal.
/// Set Negative flag iff the result is negative.
pub fn fpu<T>(cpu: &mut Cpu, operation: FpuOp, a: T, b: T) -> T
where
    T: BitsAsFloat<FloatType = f32>,
{
    match operation {
        Add => fpu_operation(cpu, a, b, |x, y| x + y),
        Sub => fpu_operation(cpu, a, b, |x, y| x - y),
        Mul => fpu_operation(cpu, a, b, |x, y| x * y),
        Div => fpu_operation(cpu, a, b, |x, y| x / y),
    }
}

fn fpu_operation<T, F>(cpu: &mut Cpu, a: T, b: T, mut op: F) -> T
where
    T: BitsAsFloat<FloatType = f32>,
    F: FnMut(f32, f32) -> f32,
{
    let float_result = op(a.bits_to_float(), b.bits_to_float());
    handle_fpu_flags(cpu, float_result);
    T::from_float_bits(float_result)
}

fn handle_fpu_flags(cpu: &mut Cpu, result: f32) {
    cpu.change_flag(Zero, result == 0.0);
    cpu.change_flag(Carry, result.is_infinite());
    cpu.change_flag(Overflow, result.is_nan());
    cpu.change_flag(Parity, result.is_subnormal());
    cpu.change_flag(Negative, result.is_sign_negative());
}

/// Implementors of this trait can be bitwise interpreted as a floating-point data type.
pub trait BitsAsFloat {
    type FloatType;

    /// Convert the bits of this type to its corresponding floating-point type.
    fn bits_to_float(self) -> Self::FloatType;

    /// Convert the bits of a floating-point type to this type.
    fn from_float_bits(float: Self::FloatType) -> Self;
}
impl BitsAsFloat for u32 {
    type FloatType = f32;

    fn bits_to_float(self) -> Self::FloatType {
        <Self::FloatType>::from_bits(self)
    }

    fn from_float_bits(float: Self::FloatType) -> Self {
        <Self>::from_le_bytes(float.to_le_bytes())
    }
}
