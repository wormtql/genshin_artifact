use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct HuskOfOpulentDreamsEffect {
    pub level: f64,
}

impl HuskOfOpulentDreamsEffect {
    pub fn new(config: &ArtifactEffectConfig) -> HuskOfOpulentDreamsEffect {
        HuskOfOpulentDreamsEffect {
            level: config.config_husk_of_opulent_dreams.level,
        }
    }
}

impl ArtifactEffect for HuskOfOpulentDreamsEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            Box::new(|n| (String::from("华馆梦醒形骸记2"), 0.3 * n.value()))
        );
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        let level = self.level;
        attribute.add_edge(
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            Box::new(move |n| (String::from("华馆梦醒形骸记4"), 0.06 * level * n.value()))
        );
        attribute.add_value(AttributeName::BonusGeo, "华馆梦醒形骸记4", self.level * 0.06);
    }
}