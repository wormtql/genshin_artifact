use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct HeartOfDepthEffect {
    pub rate: f64,
}

impl HeartOfDepthEffect {
    pub fn new(config: &ArtifactEffectConfig) -> HeartOfDepthEffect {
        HeartOfDepthEffect {
            rate: config.config_heart_of_depth.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for HeartOfDepthEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusHydro, "沉沦之心2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "沉沦之心4", self.rate * 0.3);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "沉沦之心4", self.rate * 0.3);
    }
}

pub struct HeartOfDepth;

impl ArtifactTrait for HeartOfDepth {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(HeartOfDepthEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::HeartOfDepth,
        name_mona: "heartOfDepth",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "沉沦之心",
            en: "Heart of Depth",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "饰金胸花",
            en: "Gilded Corsage",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "追忆之风",
            en: "Gust of Nostalgia",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "坚铜罗盘",
            en: "Copper Compass",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "沉波之盏",
            en: "Goblet of Thundering Deep",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "酒渍船帽",
            en: "Wine-Stained Tricorne",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%水元素伤害加成。",
            en: "Hydro DMG Bonus +15%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后的15秒内，普通攻击与重击造成的伤害提高30%。",
            en: "After using Elemental Skill, increases Normal Attack and Charged Attack DMG by 30% for 15s.",
        )),
        effect5: None,
        internal_id: 15016,
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
