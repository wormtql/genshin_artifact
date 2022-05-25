use std::cell::RefCell;
use std::rc::Rc;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::mona_object::{MonaObject, MonaObjectTrait};

pub struct MonaObjectBool {
    pub value: bool
}

impl MonaObjectTrait for MonaObjectBool {
}