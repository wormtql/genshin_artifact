use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct AdventurerEffect {}

impl AdventurerEffect {
    pub fn new() -> AdventurerEffect {
        AdventurerEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for AdventurerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HPFixed, "冒险家2", 1000.0);
    }
}