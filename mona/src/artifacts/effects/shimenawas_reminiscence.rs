use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

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

impl ArtifactEffect for ShimenawasReminiscenceEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(|n| (String::from("追忆之注连2"), 0.18 * n.value()))
        );
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusNormalAttack, "追忆之注连4", self.rate * 0.5);
        attribute.add_value(AttributeName::BonusChargedAttack, "追忆之注连4", self.rate * 0.5);
        attribute.add_value(AttributeName::BonusPlungingAttack, "追忆之注连4", self.rate * 0.5);
    }
}