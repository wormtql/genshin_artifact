use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct ViridescentVenererEffect {}

impl ViridescentVenererEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ViridescentVenererEffect {
        ViridescentVenererEffect {}
    }
}

impl ArtifactEffect for ViridescentVenererEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusAnemo, "翠绿之影2", 0.15);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::EnhanceSwirlBase, "翠绿之影4", 0.6);
    }
}