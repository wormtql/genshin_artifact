use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct ShimenawasReminiscenceEffect {
    pub rate: f64,
}

impl ShimenawasReminiscenceEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ShimenawasReminiscenceEffect {
        ShimenawasReminiscenceEffect {
            rate: config.config_shimenawas_reminiscence.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ShimenawasReminiscenceEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("追忆之注连2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "追忆之注连4", self.rate * 0.5);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "追忆之注连4", self.rate * 0.5);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "追忆之注连4", self.rate * 0.5);
    }
}