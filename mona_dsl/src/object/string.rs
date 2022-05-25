use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::object::mona_object::{MonaObject, MonaObjectTrait};

pub struct MonaObjectString {
    pub value: Rc<String>,
    pub hash: u64,
}

impl MonaObjectString {
    pub fn new(s: Rc<String>) -> Self {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let h = hasher.finish();

        MonaObjectString {
            value: s,
            hash: h
        }
    }
}

impl MonaObjectTrait for MonaObjectString {

}