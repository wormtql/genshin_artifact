use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct NoblesseObligeEffect {
    pub rate: f64,
}

impl NoblesseObligeEffect {
    pub fn new(config: &ArtifactEffectConfig) -> NoblesseObligeEffect {
        NoblesseObligeEffect {
            rate: config.config_noblesse_oblige.rate,
        }
    }
}

impl ArtifactEffect for NoblesseObligeEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusElementalBurst, "昔日宗室之仪2", 0.2);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        let rate = self.rate;
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(move |n| (String::from("昔日宗室之仪4"), rate * n.value()))
        );
    }
}