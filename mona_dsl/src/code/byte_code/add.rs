use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::code::byte_code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::{CodeError, CodeErrorType};
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::vm::env::MonaEnv;

pub struct ByteCodeAdd;

impl ByteCode for ByteCodeAdd {
    fn to_string(&self, _ctx: &CodeObject) -> String {
        String::from("add")
    }

    fn get_bits(&self) -> u64 {
        let op = ByteCodeOp::Add as u64;
        op << (64 - 8)
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> {
        let op = ByteCodeOp::Add as u64;
        if bits != (op << (64 - 8)) {
            Err(CodeError {
                t: CodeErrorType::ParseError,
                desc: format!("Error parsing op code `0x{:016x}` to `Add`", bits)
            })
        } else {
            Ok(ByteCodeAdd)
        }
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let op2 = env.pop_stack();
        let op1 = env.pop_stack();

        let value1 = match &op1.borrow().data {
            MonaObjectEnum::Number(x) => x.value,
            _ => return Err(RuntimeError::new(RuntimeErrorEnum::TypeError, &format!("expected number, found {}", op1.borrow().get_type())))
        };
        let value2 = match &op2.borrow().data {
            MonaObjectEnum::Number(x) => x.value,
            _ => return Err(RuntimeError::new(RuntimeErrorEnum::TypeError, &format!("expected number, found {}", op1.borrow().get_type())))
        };

        let result = value1 + value2;
        let obj = MonaObject::new_number(result);

        env.push_stack(Rc::new(RefCell::new(obj)));

        Ok(())
    }
}
