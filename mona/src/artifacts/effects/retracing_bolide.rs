use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

pub struct RetracingBolideEffect {
    pub rate: f64,
}

impl RetracingBolideEffect {
    pub fn new(config: &ArtifactEffectConfig) -> RetracingBolideEffect {
        RetracingBolideEffect {
            rate: config.config_retracing_bolide.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for RetracingBolideEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ShieldStrength, "逆飞的流星2", 0.35);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "逆飞的流星4", self.rate * 0.4);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "逆飞的流星4", self.rate * 0.4);
    }
}