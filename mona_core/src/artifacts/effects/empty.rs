use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::{ArtifactEffect, ArtifactEffectNone};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;

pub struct Empty;

impl ArtifactTrait for Empty {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArtifactEffectNone)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Empty,
        name_mona: "Empty",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "Empty",
            en: "Empty",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "Empty",
            en: "Empty",
        )),
        feather: None,
        sand: None,
        goblet: None,
        head: None,
        star: (4, 5),
        effect1: None,
        effect2: None,
        effect3: None,
        effect4: None,
        effect5: None,
        internal_id: usize::MAX,
    };
}
