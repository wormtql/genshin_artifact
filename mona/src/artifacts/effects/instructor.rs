use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

pub struct InstructorEffect {
    pub rate: f64,
}

impl InstructorEffect {
    pub fn new(config: &ArtifactEffectConfig) -> InstructorEffect {
        InstructorEffect {
            rate: config.config_instructor.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for InstructorEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "教官2", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "教官4", self.rate * 120.0);
    }
}