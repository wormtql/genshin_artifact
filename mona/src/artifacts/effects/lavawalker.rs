use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct LavawalkerEffect {
    pub rate: f64,
}

impl LavawalkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> LavawalkerEffect {
        LavawalkerEffect {
            rate: config.config_lavawalker.rate,
        }
    }
}

impl ArtifactEffect for LavawalkerEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusBase, "渡过烈火的贤人4", self.rate * 0.35);
    }
}