use super::super::buff::Buff;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct BuffAtkPercentage {
    pub value: f64,
}

impl BuffAtkPercentage {
    pub fn new(value: f64) -> BuffAtkPercentage {
        BuffAtkPercentage {
            value
        }
    }
}

impl<T: Attribute> Buff<T> for BuffAtkPercentage {
    fn change_attribute(&self, attribute: &mut T) {
        attribute.add_atk_percentage("buff", self.value);
    }
}