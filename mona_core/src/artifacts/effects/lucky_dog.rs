use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct LuckyDogEffect;

impl<T: Attribute> ArtifactEffect<T> for LuckyDogEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::DEFFixed, "幸运儿2", 100.0);
    }
}

pub struct LuckyDog;

impl ArtifactTrait for LuckyDog {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(LuckyDogEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::LuckyDog,
        name_mona: "luckyDog",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "幸运儿",
            en: "Lucky Dog",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "幸运儿绿花",
            en: "Lucky Dog's Clover",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "幸运儿鹰羽",
            en: "Lucky Dog's Eagle Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "幸运儿沙漏",
            en: "Lucky Dog's Hourglass",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "幸运儿之杯",
            en: "Lucky Dog's Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "幸运儿银冠",
            en: "Lucky Dog's Silver Circlet",
        )),
        star: (1, 3),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "防御力提高100点。",
            en: "DEF increased by 100.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "拾取摩拉时，恢复300点生命值。",
            en: "Picking up Mora restores 300 HP.",
        )),
        effect5: None,
        internal_id: 10011,
    };
}
