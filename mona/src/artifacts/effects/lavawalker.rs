use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};

pub struct LavawalkerEffect {
    pub rate: f64,
}

impl LavawalkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> LavawalkerEffect {
        LavawalkerEffect {
            rate: config.config_lavawalker.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for LavawalkerEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "渡过烈火的贤人4", self.rate * 0.35);
    }
}