use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::vm::env::MonaEnv;

pub struct ByteCodeLoadName {
    pub index: u32,
}

impl ByteCode for ByteCodeLoadName {
    fn to_string(&self, ctx: &CodeObject) -> String {
        format!("load_name {}({})", self.index, ctx.names[self.index as usize])
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::LoadName as u64;
        (op << (64 - 8)) | (self.index as u64)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let name = &env.code_object.names[self.index as usize];
        // println!("{}", name);

        let value = match env.namespace.map.get(name) {
            Some(x) => x.clone(),
            None => {
                match env.global_namespace.map.get(name) {
                    Some(x) => x.clone(),
                    None => return Err(RuntimeError::new(RuntimeErrorEnum::NameNotFound, &format!("cannot load name `{}`", name)))
                }
            }
        };

        env.push_stack(value);

        Ok(())
    }
}