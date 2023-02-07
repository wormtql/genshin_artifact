use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::{ArtifactEffect, ArtifactEffectNone};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;

pub struct PrayersToSpringtime;

impl ArtifactTrait for PrayersToSpringtime {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArtifactEffectNone)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::PrayersToSpringtime,
        name_mona: "prayersToSpringtime",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "祭冰之人",
            en: "Prayers to Springtime",
        ),
        flower: None,
        feather: None,
        sand: None,
        goblet: None,
        head: Some(crate::common::i18n::locale!(
            zh_cn: "祭冰礼冠",
            en: "Tiara of Frost",
        )),
        star: (3, 4),
        effect1: Some(crate::common::i18n::locale!(
            zh_cn: "受到的冰元素附着效果的持续时间减少40%。",
            en: "Affected by Cryo for 40% less time.",
        )),
        effect2: None,
        effect3: None,
        effect4: None,
        effect5: None,
        internal_id: 15013,
    };
}
