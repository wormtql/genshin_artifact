use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct ThunderingFuryEffect {}

impl ThunderingFuryEffect {
    pub fn new() -> ThunderingFuryEffect {
        ThunderingFuryEffect {}
    }
}

impl ArtifactEffect for ThunderingFuryEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusElectro, "如雷的盛怒2", 0.15);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::EnhanceOverload, "如雷的盛怒4", 0.4);
        attribute.add_value(AttributeName::EnhanceElectroCharged, "如雷的盛怒4", 0.4);
        attribute.add_value(AttributeName::EnhanceSuperconduct, "如雷的盛怒4", 0.4);
    }
}