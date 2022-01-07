use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct GamblerEffect {}

impl GamblerEffect {
    pub fn new() -> GamblerEffect {
        GamblerEffect {}
    }
}

impl ArtifactEffect for GamblerEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusElementalSkill, "赌徒2", 0.2);
    }
}