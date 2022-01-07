use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct CrimsonWitchOfFlamesEffect {
    pub level: f64,
}

impl CrimsonWitchOfFlamesEffect {
    pub fn new(config: &ArtifactEffectConfig) -> CrimsonWitchOfFlamesEffect {
        CrimsonWitchOfFlamesEffect {
            level: config.config_crimson_witch_of_flames.level.min(3.0),
        }
    }
}

impl ArtifactEffect for CrimsonWitchOfFlamesEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusPyro, "炽烈的炎之魔女2", 0.15);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        let key = "炽烈的炎之魔女4";
        attribute.add_value(AttributeName::EnhanceOverload, key, 0.4);
        attribute.add_value(AttributeName::EnhanceBurning, key, 0.4);
        attribute.add_value(AttributeName::EnhanceVaporize, key, 0.15);
        attribute.add_value(AttributeName::EnhanceMelt, key, 0.15);
        attribute.add_value(AttributeName::BonusPyro, key, self.level * 0.075);
    }
}