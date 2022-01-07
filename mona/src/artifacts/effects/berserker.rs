use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct BerserkerEffect {
    pub rate: f64,
}

impl BerserkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BerserkerEffect {
        BerserkerEffect {
            rate: config.config_berserker.rate,
        }
    }
}

impl ArtifactEffect for BerserkerEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    }
}