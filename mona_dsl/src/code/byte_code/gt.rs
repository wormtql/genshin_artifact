use std::cell::RefCell;
use std::rc::Rc;
use crate::code::byte_code::ByteCode;
use crate::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::vm::env::MonaEnv;

pub struct ByteCodeGt;

impl ByteCode for ByteCodeGt {
    fn to_string(&self, ctx: &CodeObject) -> String {
        String::from("gt")
    }

    fn get_bits(&self) -> u64 {
        todo!()
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
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

        let result = value1 > value2;
        let obj = MonaObject::new_bool(result);

        env.push_stack(Rc::new(RefCell::new(obj)));

        Ok(())
    }
}