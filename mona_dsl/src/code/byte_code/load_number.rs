use std::cell::RefCell;
use std::rc::Rc;
use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::{CodeError, CodeErrorType};
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::mona_object::MonaObject;
use crate::vm::env::MonaEnv;

pub struct ByteCodeLoadNumber {
    pub index: u32,
}

impl ByteCode for ByteCodeLoadNumber {
    fn to_string(&self, ctx: &CodeObject) -> String {
        format!("load_number {}({})", self.index, ctx.numbers[self.index as usize])
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::LoadNumber as u64;

        (op << (64 - 8)) | (self.index as u64)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        let op = ByteCodeOp::LoadNumber as u64;
        let op2 = op << (64 - 8);

        if (op2 & bits) != op2 {
            Err(CodeError {
                t: CodeErrorType::ParseError,
                desc: format!("Error parsing op code `0x{:016x}` to `LoadNumber`", bits)
            })
        } else {
            let index = (bits & 0xffffffff) as u32;
            Ok(ByteCodeLoadNumber {
                index
            })
        }
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let value = match env.code_object.numbers.get(self.index as usize) {
            Some(x) => *x,
            None => return Err(RuntimeError::new(RuntimeErrorEnum::CodeError, &format!("cannot find number index {}", self.index)))
        };

        let obj = MonaObject::new_number(value);
        env.push_stack(Rc::new(RefCell::new(obj)));

        Ok(())
    }
}
