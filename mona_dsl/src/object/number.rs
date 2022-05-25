use crate::object::mona_object::MonaObjectTrait;

pub struct MonaObjectNumber {
    pub value: f64,
    pub hash: u64,
}

impl MonaObjectTrait for MonaObjectNumber {

}