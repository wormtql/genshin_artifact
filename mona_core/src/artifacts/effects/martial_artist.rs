use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

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

pub struct MartialArtist;

impl ArtifactTrait for MartialArtist {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(MartialArtistEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::MartialArtist,
        name_mona: "martialArtist",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "武人",
            en: "Martial Artist",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "武人的红花",
            en: "Martial Artist's Red Flower",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "武人的羽饰",
            en: "Martial Artist's Feather Accessory",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "武人的水漏",
            en: "Martial Artist's Water Hourglass",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "武人的酒杯",
            en: "Martial Artist's Wine Cup",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "武人的头巾",
            en: "Martial Artist's Bandana",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击与重击造成的伤害提高15%；",
            en: "Increases Normal Attack and Charged Attack DMG by 15%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后的8秒内，普通攻击和重击造成的伤害提升25%。",
            en: "After using Elemental Skill, increases Normal Attack and Charged Attack DMG by 25% for 8s.",
        )),
        effect5: None,
        internal_id: 10006,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
