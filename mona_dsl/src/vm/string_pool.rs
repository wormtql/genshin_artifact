use std::collections::HashMap;
use std::rc::Rc;

pub struct StringPool {
    pub pool: HashMap<String, Rc<String>>
}
