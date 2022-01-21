use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

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

impl<T: Attribute> ArtifactEffect<T> for MartialArtistEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "武人2", 0.15);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "武人2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "武人4", self.rate * 0.25);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "武人4", self.rate * 0.25);
    }
}