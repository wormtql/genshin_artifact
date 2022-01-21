use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

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

impl<T: Attribute> ArtifactEffect<T> for CrimsonWitchOfFlamesEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPyro, "炽烈的炎之魔女2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        let key = "炽烈的炎之魔女4";
        attribute.set_value_by(AttributeName::EnhanceOverload, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceBurning, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceVaporize, key, 0.15);
        attribute.set_value_by(AttributeName::EnhanceMelt, key, 0.15);
        attribute.set_value_by(AttributeName::BonusPyro, key, self.level * 0.075);
    }
}