use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::RuntimeError;
use crate::vm::env::MonaEnv;

pub struct ByteCodeLoadBool {
    pub value: bool
}

impl ByteCode for ByteCodeLoadBool {
    fn to_string(&self, ctx: &CodeObject) -> String {
        format!("load_bool {}", self.value)
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::LoadBool as u64;
        let v = if self.value { 1 } else { 0 } as u64;

        (op << (64 - 8)) | v
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        if self.value {
            env.push_stack(env.bool_true.clone());
        } else {
            env.push_stack(env.bool_false.clone());
        }

        Ok(())
    }
}