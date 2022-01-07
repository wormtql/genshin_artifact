use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct OceanHuedClamEffect {}

impl OceanHuedClamEffect {
    pub fn new() -> OceanHuedClamEffect {
        OceanHuedClamEffect {}
    }
}

impl ArtifactEffect for OceanHuedClamEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::HealingBonus, "海染砗磲2", 0.15);
    }

    // todo
    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}