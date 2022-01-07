use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

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

impl ArtifactEffect for BraveHeartEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(|n| (String::from("勇士之心2"), n.value() * 0.18))
        );
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusBase, "勇士之心4", self.rate * 0.3);
    }
}