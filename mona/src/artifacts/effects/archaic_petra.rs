use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};
use crate::common::Element;

pub struct ArchaicPetraEffect {
    pub element: Element,
    pub rate: f64,
}

impl ArchaicPetraEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ArchaicPetraEffect {
        ArchaicPetraEffect {
            element: config.config_archaic_petra.element,
            rate: config.config_archaic_petra.rate,
        }
    }
}

impl ArtifactEffect for ArchaicPetraEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusGeo, "悠古的磐岩2", 0.15);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        let attribute_name = AttributeName::bonus_name_by_element(self.element);

        attribute.add_value(attribute_name, "悠古的磐岩4", self.rate * 0.35)
    }
}