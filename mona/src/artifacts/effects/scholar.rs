use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct ScholarEffect {}

impl ScholarEffect {
    pub fn new() -> ScholarEffect {
        ScholarEffect {}
    }
}

impl ArtifactEffect for ScholarEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::Recharge, "学士2", 0.2);
    }
}