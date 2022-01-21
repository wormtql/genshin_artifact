use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct TheExileEffect {}

impl TheExileEffect {
    pub fn new() -> TheExileEffect {
        TheExileEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for TheExileEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "流放者2", 0.2);
    }
}