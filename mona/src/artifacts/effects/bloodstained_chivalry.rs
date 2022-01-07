use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct BloodstainedChivalryEffect {
    pub rate: f64,
}

impl BloodstainedChivalryEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BloodstainedChivalryEffect {
        BloodstainedChivalryEffect {
            rate: config.config_bloodstained_chivalry.rate,
        }
    }
}

impl ArtifactEffect for BloodstainedChivalryEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusPhysical, "染血的骑士道2", 0.25);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusChargedAttack, "染血的骑士道4", self.rate * 0.5);
    }
}