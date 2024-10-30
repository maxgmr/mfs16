// //! All the floating-point arithmetic instructions.
// //! Represents a physical FPU (floating-point unit).
// //!
// //! The FPU can perform operations on 32-bit IEEE 754 floating-point representations.
// use crate::helpers::{get_bit, test_bit};
//
// /// All the operations the FPU can perform.
// pub enum FpuOp {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }
//
// /// Implementors of this trait can access various methods that are useful for floating-point
// /// arithmetic.
// trait Ieee754 {
//     /// The bit width of the exponent component.
//     const EXPONENT_WIDTH: usize;
//
//     /// The bit width of the significand component.
//     const SIGNIFICAND_WIDTH: usize;
//
//     /// Get the sign bit of this number as a 1 or a 0.
//     fn sign(&self) -> Self;
//
//     /// Get the sign bit of this number as a boolean.
//     fn is_negative(&self) -> bool;
//
//     /// Get the exponent of this number.
//     fn exponent(&self) -> Self;
//
//     /// Get the significand of this number.
//     fn significand(&self) -> Self;
//
//     /// Get the significand of this number with the implicit leading bit.
//     fn actual_significand(&self) -> Self;
// }
// impl Ieee754 for u32 {
//     const EXPONENT_WIDTH: usize = 8;
//     const SIGNIFICAND_WIDTH: usize = 23;
//
//     fn sign(&self) -> Self {
//         get_bit(*self, <Self>::BITS - 1)
//     }
//
//     fn is_negative(&self) -> bool {
//         test_bit(*self, <Self>::BITS - 1)
//     }
//
//     fn exponent(&self) -> Self {
//         (*self << 1) >> (<Self>::EXPONENT_WIDTH + 1)
//     }
//
//     fn significand(&self) -> Self {
//         (*self << (<Self>::EXPONENT_WIDTH + 1)) >> (<Self>::EXPONENT_WIDTH + 1)
//     }
//
//     fn actual_significand(&self) -> Self {
//         (1 << <Self>::SIGNIFICAND_WIDTH) | self.significand()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;
//
//     use super::*;
//
//     #[test]
//     fn test_sign_u32() {
//         assert_eq!(0x8000_0000.sign(), 1);
//         assert!(0x8000_0000.is_negative());
//         assert_eq!(0x7FFF_FFFF.sign(), 0);
//         assert!(!0x7FFF_FFFF.is_negative());
//     }
//
//     #[test]
//     fn test_from_bits() {
//         assert_eq!(
//             f32::from_bits(0b0011_1110_0010_0000_0000_0000_0000_0000),
//             0.15625_f32
//         );
//     }
// }
