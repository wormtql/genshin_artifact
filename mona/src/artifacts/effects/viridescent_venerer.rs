use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct ViridescentVenererEffect {}

impl ViridescentVenererEffect {
    pub fn new() -> ViridescentVenererEffect {
        ViridescentVenererEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for ViridescentVenererEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusAnemo, "翠绿之影2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceSwirlBase, "翠绿之影4", 0.6);
    }
}