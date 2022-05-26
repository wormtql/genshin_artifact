use std::cell::RefCell;
use std::rc::Rc;
use smallvec::SmallVec;
use crate::error::runtime_error::RuntimeError;
use crate::object::mona_object::{MonaObject, MonaObjectTrait};
use crate::vm::env::MonaEnv;

pub type ParamVecType = SmallVec<[Rc<RefCell<MonaObject>>; 5]>;
pub type FunctionReturnType = Result<Option<Rc<RefCell<MonaObject>>>, RuntimeError>;
pub type BuiltinFunctionType = Box<dyn Fn(ParamVecType, &mut MonaEnv) -> FunctionReturnType>;

pub struct MonaObjectBuiltinFunction {
    // pub param_count: usize,
    pub handler: BuiltinFunctionType,
    pub name: String,
}

impl MonaObjectBuiltinFunction {
    pub fn call(&self, params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
        (*self.handler)(params, env)
    }
}

impl MonaObjectTrait for MonaObjectBuiltinFunction {

}