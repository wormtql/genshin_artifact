use serde::__private::de::IdentifierDeserializer;
use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct BraveHeartEffect {
    pub rate: f64,
}

impl BraveHeartEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BraveHeartEffect {
        BraveHeartEffect {
            rate: config.config_brave_heart.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BraveHeartEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("勇士之心2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "勇士之心4", self.rate * 0.3);
    }
}

pub struct BraveHeart;

impl ArtifactTrait for BraveHeart {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BraveHeartEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BraveHeart,
        name_mona: "braveHeart",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "勇士之心",
            en: "Brave Heart",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "勇士的勋章",
            en: "Medal of the Brave",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "勇士的期许",
            en: "Prospect of the Brave",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "勇士的坚毅",
            en: "Fortitude of the Brave",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "勇士的壮行",
            en: "Outset of the Brave",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "勇士的冠冕",
            en: "Crown of the Brave",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "对生命值高于50%的敌人，造成的伤害增加30%。",
            en: "Increases DMG by 30% against opponents with more than 50% HP.",
        )),
        effect5: None,
        internal_id: 10002,
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
