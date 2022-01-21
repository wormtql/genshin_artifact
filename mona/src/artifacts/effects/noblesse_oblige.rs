use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct NoblesseObligeEffect {
    pub rate: f64,
}

impl NoblesseObligeEffect {
    pub fn new(config: &ArtifactEffectConfig) -> NoblesseObligeEffect {
        NoblesseObligeEffect {
            rate: config.config_noblesse_oblige.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for NoblesseObligeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalBurst, "昔日宗室之仪2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("昔日宗室之仪4", self.rate * 0.2);
    }
}