use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct TheExileEffect {}

impl TheExileEffect {
    pub fn new(config: &ArtifactEffectConfig) -> TheExileEffect {
        TheExileEffect {}
    }
}

impl ArtifactEffect for TheExileEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::Recharge, "流放者2", 0.2);
    }
}