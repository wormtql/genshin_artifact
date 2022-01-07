use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct EmblemOfSeveredFataEffect {}

impl EmblemOfSeveredFataEffect {
    pub fn new() -> EmblemOfSeveredFataEffect {
        EmblemOfSeveredFataEffect {}
    }
}

impl ArtifactEffect for EmblemOfSeveredFataEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::Recharge, "绝缘之旗印2", 0.2);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::Recharge,
            AttributeName::BonusElementalBurst,
            Box::new(|n| (String::from("绝缘之旗印4"), (n.value() * 0.25).min(0.75)))
        );
    }
}