use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::mona_object::MonaObject;

pub struct Namespace {
    pub map: HashMap<String, Rc<RefCell<MonaObject>>>
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            map: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: String, value: MonaObject) {
        self.map.insert(name, Rc::new(RefCell::new(value)));
    }
}