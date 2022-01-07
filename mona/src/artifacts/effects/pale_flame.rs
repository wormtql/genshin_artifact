use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

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

impl ArtifactEffect for PaleFlameEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusPhysical, "苍白之火2", 0.25);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        let level = self.level;
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(move |n| (String::from("苍白之火4"), 0.09 * level * n.value()))
        );
        attribute.add_value(AttributeName::BonusPhysical, "苍白之火4", 0.25 * self.full_rate);
    }
}