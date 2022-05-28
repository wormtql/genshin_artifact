use std::cell::RefCell;
use std::rc::Rc;
use crate::code::byte_code::ByteCode;
use crate::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::RuntimeError;
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::vm::env::MonaEnv;

pub struct ByteCodeEq;

impl ByteCode for ByteCodeEq {
    fn to_string(&self, ctx: &CodeObject) -> String {
        String::from("eq")
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

        let result = if op1.borrow().is_number() && op2.borrow().is_number() {
            let value1 = op1.borrow().get_number();
            let value2 = op2.borrow().get_number();
            value1 == value2
        } else if op1.borrow().is_bool() && op2.borrow().is_bool() {
            let value1 = op1.borrow().get_bool();
            let value2 = op2.borrow().get_bool();
            value1 == value2
        } else if op1.borrow().is_string() && op2.borrow().is_string() {
                op1.borrow().get_string() == op2.borrow().get_string()
            // value1 == value2
        } else {
            op1.as_ptr() == op2.as_ptr()
        };

        let obj = MonaObject::new_bool(result);

        env.push_stack(Rc::new(RefCell::new(obj)));

        Ok(())
    }
}