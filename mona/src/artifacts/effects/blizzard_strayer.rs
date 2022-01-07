use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct BlizzardStrayerEffect {
    pub crit_bonus: f64,
}

impl BlizzardStrayerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BlizzardStrayerEffect {
        BlizzardStrayerEffect {
            crit_bonus: config.config_blizzard_strayer.critical_bonus
        }
    }
}

impl ArtifactEffect for BlizzardStrayerEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusCryo, "冰风迷途的勇士2", 0.15);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::CriticalBase, "冰风迷途的勇士4", self.crit_bonus);
    }
}