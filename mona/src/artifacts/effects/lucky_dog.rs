use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct LuckyDogEffect {}

impl LuckyDogEffect {
    pub fn new() -> LuckyDogEffect {
        LuckyDogEffect {}
    }
}

impl ArtifactEffect for LuckyDogEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::DEFFixed, "幸运儿2", 100.0);
    }
}