/// Random-access memory for direct interfacing with the CPU.
use crate::RAM_SIZE;

pub struct Ram {
    memory: [u8; RAM_SIZE],
}
impl Ram {}
