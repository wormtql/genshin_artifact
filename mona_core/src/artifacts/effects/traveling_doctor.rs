use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::{ArtifactEffect, ArtifactEffectNone};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;

pub struct TravelingDoctor;

impl ArtifactTrait for TravelingDoctor {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArtifactEffectNone)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TravelingDoctor,
        name_mona: "travelingDoctor",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "游医",
            en: "Traveling Doctor",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "游医的银莲",
            en: "Traveling Doctor's Silver Lotus",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "游医的枭羽",
            en: "Traveling Doctor's Owl Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "游医的怀钟",
            en: "Traveling Doctor's Pocket Watch",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "游医的药壶",
            en: "Traveling Doctor's Medicine Pot",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "游医的方巾",
            en: "Traveling Doctor's Handkerchief",
        )),
        star: (1, 3),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "角色受到的治疗效果提高20%。",
            en: "Increases incoming healing by 20%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发时，恢复20%生命值。",
            en: "Using Elemental Burst restores 20% HP.",
        )),
        effect5: None,
        internal_id: 10013,
    };
}
