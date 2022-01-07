use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct ResolutionOfSojournerEffect {}

impl ResolutionOfSojournerEffect {
    pub fn new() -> ResolutionOfSojournerEffect {
        ResolutionOfSojournerEffect {}
    }
}

impl ArtifactEffect for ResolutionOfSojournerEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(|n| (String::from("行者之心2"), 0.18 * n.value()))
        );
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::CriticalChargedAttack, "行者之心4", 0.3);
    }
}