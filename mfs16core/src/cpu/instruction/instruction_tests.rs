use strum::IntoEnumIterator;

use super::*;

#[test]
fn test_into_from_opcode() {
    for instr in Instruction::iter() {
        assert_eq!(instr, Instruction::from_opcode(instr.into_opcode()));
    }
}
