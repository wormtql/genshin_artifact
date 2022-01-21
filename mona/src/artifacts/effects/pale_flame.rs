use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct PaleFlameEffect {
    pub level: f64,
    pub full_rate: f64,
}

impl PaleFlameEffect {
    pub fn new(config: &ArtifactEffectConfig) -> PaleFlameEffect {
        PaleFlameEffect {
            level: config.config_pale_flame.avg_level,
            full_rate: config.config_pale_flame.full_rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for PaleFlameEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "苍白之火2", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("苍白之火4", 0.09 * self.level);
        attribute.set_value_by(AttributeName::BonusPhysical, "苍白之火4", 0.25 * self.full_rate);
    }
}