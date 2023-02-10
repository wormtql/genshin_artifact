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
        name_locale: crate::common::i18n::locale!(
            zh_cn: "逆飞的流星",
            en: "Retracing Bolide",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "夏祭之花",
            en: "Summer Night's Bloom",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "夏祭终末",
            en: "Summer Night's Finale",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "夏祭之刻",
            en: "Summer Night's Moment",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "夏祭水玉",
            en: "Summer Night's Waterballoon",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "夏祭之面",
            en: "Summer Night's Mask",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "护盾强效提高35%。",
            en: "Increases Shield Strength by 35%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "处于护盾庇护下时，额外获得40%普通攻击和重击伤害加成。",
            en: "While protected by a shield, gain an additional 40% Normal and Charged Attack DMG.",
        )),
        effect5: None,
        internal_id: 15015,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "护盾覆盖率",
                en: "Shield Coverage",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
