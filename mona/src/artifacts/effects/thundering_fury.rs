use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct ThunderingFuryEffect {}

impl ThunderingFuryEffect {
    pub fn new() -> ThunderingFuryEffect {
        ThunderingFuryEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for ThunderingFuryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElectro, "如雷的盛怒2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceOverload, "如雷的盛怒4", 0.4);
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "如雷的盛怒4", 0.4);
        attribute.set_value_by(AttributeName::EnhanceSuperconduct, "如雷的盛怒4", 0.4);
    }
}