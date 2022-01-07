use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct AdventurerEffect {}

impl AdventurerEffect {
    pub fn new() -> AdventurerEffect {
        AdventurerEffect {}
    }
}

impl ArtifactEffect for AdventurerEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::HPFixed, "冒险家2", 1000.0);
    }
}