use smallvec::SmallVec;
use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::builtin_function::ParamVecType;
use crate::object::mona_object::MonaObjectEnum;
use crate::vm::env::MonaEnv;

pub struct ByteCodeCall {
    pub param_count: u8
}

impl ByteCode for ByteCodeCall {
    fn to_string(&self, ctx: &CodeObject) -> String {
        format!("call {}", self.param_count)
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::Call as u64;

        (op << (64 - 8)) | ((self.param_count as u64) << 48)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let function_object1 = env.pop_stack();
        let function_object = function_object1.borrow();
        let func = match &function_object.data {
            MonaObjectEnum::BuiltinFunction(x) => x,
            _ => return Err(RuntimeError::new(
                RuntimeErrorEnum::TypeError,
                &format!("expecting `function`, found `{}`", function_object.get_type())
            ))
        };

        let mut params: ParamVecType = SmallVec::new();
        for _ in 0..self.param_count {
            params.push(env.pop_stack());
        }

        let ret = func.call(params, env)?;

        if let Some(x) = ret {
            env.push_stack(x);
        }

        Ok(())
    }
}