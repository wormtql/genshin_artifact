use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct DefendersWillEffect {}

impl DefendersWillEffect {
    pub fn new() -> DefendersWillEffect {
        DefendersWillEffect {}
    }
}

impl ArtifactEffect for DefendersWillEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            Box::new(|n| (String::from("守护之心2"), n.value() * 0.3))
        );
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}