use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

pub struct ThundersootherEffect {
    pub rate: f64,
}

impl ThundersootherEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ThundersootherEffect {
        ThundersootherEffect {
            rate: config.config_thundersoother.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ThundersootherEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "平息鸣雷的尊者4", self.rate * 0.35);
    }
}