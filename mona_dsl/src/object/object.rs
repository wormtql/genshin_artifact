use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::runtime_error::RuntimeError;
use crate::object::mona_object::{MonaObject, MonaObjectTrait};

pub struct MonaObjectObject {
    pub map: HashMap<u64, Rc<RefCell<MonaObject>>>
}

impl MonaObjectTrait for MonaObjectObject {
    fn access(&self, key: &MonaObject) -> Result<Rc<RefCell<MonaObject>>, RuntimeError> {
        todo!()
    }
}
