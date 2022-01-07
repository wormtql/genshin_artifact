use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct TenacityOfTheMillelithEffect {
    pub rate: f64,
}

impl TenacityOfTheMillelithEffect {
    pub fn new(config: &ArtifactEffectConfig) -> TenacityOfTheMillelithEffect {
        TenacityOfTheMillelithEffect {
            rate: config.config_tenacity_of_the_millelith.rate,
        }
    }
}

impl ArtifactEffect for TenacityOfTheMillelithEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            Box::new(|n| (String::from("千岩牢固2"), n.value() * 0.2))
        );
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        let rate = self.rate;
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(move |n| (String::from("千岩牢固4"), n.value() * rate * 0.2))
        );
        attribute.add_value(AttributeName::ShieldStrength, "千岩牢固4", self.rate * 0.3);
    }
}