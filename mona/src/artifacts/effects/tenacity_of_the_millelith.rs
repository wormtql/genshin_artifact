use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct TenacityOfTheMillelithEffect {
    pub rate: f64,
}

impl TenacityOfTheMillelithEffect {
    pub fn new(config: &ArtifactEffectConfig) -> TenacityOfTheMillelithEffect {
        TenacityOfTheMillelithEffect {
            rate: config.config_tenacity_of_the_millelith.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for TenacityOfTheMillelithEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_hp_percentage("千岩牢固2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("千岩牢固4", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ShieldStrength, "千岩牢固4", self.rate * 0.3);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "千岩牢固4", self.rate * 0.2);
    }
}
