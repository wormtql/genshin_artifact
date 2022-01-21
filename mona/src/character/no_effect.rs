use crate::attribute::Attribute;
use crate::common::ChangeAttribute;

pub struct NoEffect;

impl<T: Attribute> ChangeAttribute<T> for NoEffect {
    fn change_attribute(&self, attribute: &mut T) {

    }
}