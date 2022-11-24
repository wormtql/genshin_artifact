use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BerserkerEffect {
    pub rate: f64,
}

impl BerserkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BerserkerEffect {
        BerserkerEffect {
            rate: config.config_berserker.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BerserkerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "战狂2", 0.12);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    }
}

pub struct Berserker;

impl ArtifactTrait for Berserker {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BerserkerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Berserker,
        name_mona: "berserker",
        chs: "战狂",
        flower: Some("战狂的蔷薇"),
        feather: Some("战狂的翎羽"),
        sand: Some("战狂的时计"),
        goblet: Some("战狂的骨杯"),
        head: Some("战狂的鬼面"),
        star: (3, 4),
        effect1: None,
        effect2: Some("暴击率提高12%。"),
        effect3: None,
        effect4: Some("生命值低于70%时，暴击率额外提高24%。"),
        effect5: None,
        internal_id: 10005,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
