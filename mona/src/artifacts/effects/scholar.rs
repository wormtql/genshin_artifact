use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct ScholarEffect {}

impl ScholarEffect {
    pub fn new() -> ScholarEffect {
        ScholarEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for ScholarEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "学士2", 0.2);
    }
}