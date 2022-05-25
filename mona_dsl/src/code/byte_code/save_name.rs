use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::{CodeError, CodeErrorType};
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::vm::env::MonaEnv;

pub struct ByteCodeSaveName {
    pub name_index: u32,
}

impl ByteCode for ByteCodeSaveName {
    fn to_string(&self, ctx: &CodeObject) -> String {
        format!("save {}", ctx.names[self.name_index as usize])
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::SaveName as u64;

        (op << (64 - 8)) | (self.name_index as u64)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> {
        let op = ByteCodeOp::SaveName as u64;
        let op2 = op << (64 - 8);

        if op2 & bits != op2 {
            Err(CodeError {
                t: CodeErrorType::ParseError,
                desc: format!("Error parsing op code `0x{:016x}` to `SaveName`", bits)
            })
        } else {
            let index = (bits & 0xffffffff) as u32;

            Ok(ByteCodeSaveName {
                name_index: index
            })
        }
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let value = env.pop_stack();
        let name = match env.code_object.names.get(self.name_index as usize) {
            Some(x) => x,
            None => {
                return Err(RuntimeError::new(RuntimeErrorEnum::CodeError, &format!("cannot find name index {}", self.name_index)));
            }
        };

        env.namespace.map.insert(name.clone(), value);

        Ok(())
    }
}