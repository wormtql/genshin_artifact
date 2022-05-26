use std::cell::RefCell;
use std::rc::Rc;
use crate::code::byte_code::{ByteCode, ByteCodeOp};
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
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
        let obj = env.pop_stack();

        let value = match &obj.borrow().data {
            MonaObjectEnum::Number(x) => x.value,
            _ => {
                return Err(RuntimeError::new(RuntimeErrorEnum::TypeError, &format!("expecting `number`, found `{}`", obj.borrow().get_type())));
            }
        };

        let obj = MonaObject::new_number(-value);
        env.push_stack(Rc::new(RefCell::new(obj)));

        Ok(())
    }
}