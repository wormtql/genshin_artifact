use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

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

impl<T: Attribute> ArtifactEffect<T> for BloodstainedChivalryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "染血的骑士道2", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusChargedAttack, "染血的骑士道4", self.rate * 0.5);
    }
}