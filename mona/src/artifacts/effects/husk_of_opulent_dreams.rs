use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

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

impl<T: Attribute> ArtifactEffect<T> for HuskOfOpulentDreamsEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("华馆梦醒形骸记2", 0.3);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_def_percentage("华馆梦醒形骸记4", 0.06 * self.level);
        attribute.set_value_by(AttributeName::BonusGeo, "华馆梦醒形骸记4", self.level * 0.06);
    }
}