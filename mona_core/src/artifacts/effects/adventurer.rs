use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct AdventurerEffect;

impl<T: Attribute> ArtifactEffect<T> for AdventurerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HPFixed, "冒险家2", 1000.0);
    }
}

pub struct Adventurer;

impl ArtifactTrait for Adventurer {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(AdventurerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Adventurer,
        name_mona: "adventurer",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "冒险家",
            en: "Adventurer",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "冒险家之花",
            en: "Adventurer's Flower",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "冒险家尾羽",
            en: "Adventurer's Tail Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "冒险家怀表",
            en: "Adventurer's Pocket Watch",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "冒险家金杯",
            en: "Adventurer's Golden Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "冒险家头带",
            en: "Adventurer's Bandana",
        )),
        star: (1, 3),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "生命值上限提高1000点。",
            en: "Max HP increased by 1000.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "开启各类宝箱后的5秒内，持续恢复30%生命值。",
            en: "Opening a chest regenerates 30% Max HP over 5s.",
        )),
        effect5: None,
        internal_id: 10010,
    };
}