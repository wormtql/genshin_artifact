use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct BraveHeartEffect {
    pub rate: f64,
}

impl BraveHeartEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BraveHeartEffect {
        BraveHeartEffect {
            rate: config.config_brave_heart.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BraveHeartEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("勇士之心2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "勇士之心4", self.rate * 0.3);
    }
}