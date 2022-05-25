use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::RuntimeError;
use crate::vm::env::MonaEnv;

pub struct ByteCodeAccess;

impl ByteCode for ByteCodeAccess {
    fn to_string(&self, ctx: &CodeObject) -> String {
        String::from("access")
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::Access as u64;
        op << (64 - 8)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let expression = env.pop_stack();
        let key = env.pop_stack();

        let result = expression.borrow().access(key)?;
        env.push_stack(result);

        Ok(())
    }
}