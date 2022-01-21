use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct GamblerEffect {}

impl GamblerEffect {
    pub fn new() -> GamblerEffect {
        GamblerEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for GamblerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalSkill, "赌徒2", 0.2);
    }
}