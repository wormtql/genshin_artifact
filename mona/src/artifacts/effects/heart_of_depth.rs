use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct HeartOfDepthEffect {
    pub rate: f64,
}

impl HeartOfDepthEffect {
    pub fn new(config: &ArtifactEffectConfig) -> HeartOfDepthEffect {
        HeartOfDepthEffect {
            rate: config.config_heart_of_depth.rate,
        }
    }
}

impl ArtifactEffect for HeartOfDepthEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusHydro, "沉沦之心2", 0.12);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusNormalAttack, "沉沦之心4", self.rate * 0.3);
        attribute.add_value(AttributeName::BonusChargedAttack, "沉沦之心4", self.rate * 0.3);
    }
}