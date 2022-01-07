use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

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

impl ArtifactEffect for RetracingBolideEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::ShieldStrength, "逆飞的流星2", 0.35);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusNormalAttack, "逆飞的流星4", self.rate * 0.4);
        attribute.add_value(AttributeName::BonusChargedAttack, "逆飞的流星4", self.rate * 0.4);
    }
}