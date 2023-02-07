use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::{ArtifactEffect, ArtifactEffectNone};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;

pub struct TinyMiracle;

impl ArtifactTrait for TinyMiracle {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArtifactEffectNone)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TinyMiracle,
        name_mona: "tinyMiracle",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "奇迹",
            en: "Tiny Miracle",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "奇迹之花",
            en: "Tiny Miracle's Flower",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "奇迹之羽",
            en: "Tiny Miracle's Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "奇迹之沙",
            en: "Tiny Miracle's Hourglass",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "奇迹之杯",
            en: "Tiny Miracle's Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "奇迹耳坠",
            en: "Tiny Miracle's Earrings",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "所有元素抗性提高20%。",
            en: "All Elemental RES increased by 20%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "受到某个元素类型的伤害后，相应的抗性提升30%，持续10秒。该效果每10秒只能触发一次。",
            en: "Incoming elemental DMG increases corresponding Elemental RES by 30% for 10s. Can only occur once every 10s.",
        )),
        effect5: None,
        internal_id: 10004,
    };
}
