use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

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

impl<T: Attribute> ArtifactEffect<T> for HeartOfDepthEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusHydro, "沉沦之心2", 0.12);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "沉沦之心4", self.rate * 0.3);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "沉沦之心4", self.rate * 0.3);
    }
}