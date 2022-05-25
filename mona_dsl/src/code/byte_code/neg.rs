use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::RuntimeError;
use crate::vm::env::MonaEnv;

pub struct ByteCodeNeg;

impl ByteCode for ByteCodeNeg {
    fn to_string(&self, ctx: &CodeObject) -> String {
        String::from("neg")
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::Neg as u64;

        op << (64 - 8)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        todo!()
    }
}