use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct MartialArtistEffect {
    pub rate: f64,
}

impl MartialArtistEffect {
    pub fn new(config: &ArtifactEffectConfig) -> MartialArtistEffect {
        MartialArtistEffect {
            rate: config.config_martial_artist.rate,
        }
    }
}

impl ArtifactEffect for MartialArtistEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusNormalAttack, "武人2", 0.15);
        attribute.add_value(AttributeName::BonusChargedAttack, "武人2", 0.15);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::BonusNormalAttack, "武人4", self.rate * 0.25);
        attribute.add_value(AttributeName::BonusChargedAttack, "武人4", self.rate * 0.25);
    }
}