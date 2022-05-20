use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct RetracingBolideEffect {
    pub rate: f64,
}

impl RetracingBolideEffect {
    pub fn new(config: &ArtifactEffectConfig) -> RetracingBolideEffect {
        RetracingBolideEffect {
            rate: config.config_retracing_bolide.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for RetracingBolideEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ShieldStrength, "逆飞的流星2", 0.35);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "逆飞的流星4", self.rate * 0.4);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "逆飞的流星4", self.rate * 0.4);
    }
}

pub struct RetracingBolide;

impl ArtifactTrait for RetracingBolide {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(RetracingBolideEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::RetracingBolide,
        name_mona: "retracingBolide",
        chs: "逆飞的流星",
        flower: Some("夏祭之花"),
        feather: Some("夏祭终末"),
        sand: Some("夏祭之刻"),
        goblet: Some("夏祭水玉"),
        head: Some("夏祭之面"),
        star: (4, 5),
        effect1: None,
        effect2: Some("护盾强效提高35%"),
        effect3: None,
        effect4: Some("处于护盾庇护下时，额外获得40%普通攻击和重击伤害加成。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "护盾覆盖率",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
