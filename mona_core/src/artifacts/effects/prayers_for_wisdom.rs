use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::{ArtifactEffect, ArtifactEffectNone};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;

pub struct PrayersForWisdom;

impl ArtifactTrait for PrayersForWisdom {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArtifactEffectNone)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::PrayersForWisdom,
        name_mona: "prayersForWisdom",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "祭雷之人",
            en: "Prayers for Wisdom",
        ),
        flower: None,
        feather: None,
        sand: None,
        goblet: None,
        head: Some(crate::common::i18n::locale!(
            zh_cn: "祭雷礼冠",
            en: "Tiara of Thunder",
        )),
        star: (3, 4),
        effect1: Some(crate::common::i18n::locale!(
            zh_cn: "受到的雷元素附着效果的持续时间减少40%。",
            en: "Affected by Electro for 40% less time.",
        )),
        effect2: None,
        effect3: None,
        effect4: None,
        effect5: None,
        internal_id: 15011,
    };
}
