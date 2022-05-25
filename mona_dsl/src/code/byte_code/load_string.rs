use std::cell::RefCell;
use std::rc::Rc;
use crate::code::byte_code::ByteCode;
use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::RuntimeError;
use crate::object::mona_object::MonaObject;
use crate::vm::env::MonaEnv;

pub struct ByteCodeLoadString {
    pub index: u32
}

impl ByteCode for ByteCodeLoadString {
    fn to_string(&self, ctx: &CodeObject) -> String {
        format!("load_string {}({})", self.index, ctx.names[self.index as usize])
    }

    fn get_bits(&self) -> u64 {
        todo!()
    }

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized {
        todo!()
    }

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError> {
        let s = &env.code_object.names[self.index as usize];
        let obj = MonaObject::new_string(Rc::new(s.clone()));
        env.push_stack(Rc::new(RefCell::new(obj)));

        Ok(())
    }
}
