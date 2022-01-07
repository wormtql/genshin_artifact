use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct MaidenBelovedEffect {}

impl MaidenBelovedEffect {
    pub fn new() -> MaidenBelovedEffect {
        MaidenBelovedEffect {}
    }
}

impl ArtifactEffect for MaidenBelovedEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::HealingBonus, "被怜爱的少女2", 0.15);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}