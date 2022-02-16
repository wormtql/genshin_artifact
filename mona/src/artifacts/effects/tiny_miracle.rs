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
        chs: "奇迹",
        flower: Some("奇迹之花"),
        feather: Some("奇迹之羽"),
        sand: Some("奇迹之沙"),
        goblet: Some("奇迹之杯"),
        head: Some("奇迹耳坠"),
        star: (3, 4),
        effect1: None,
        effect2: Some("所有元素抗性提高20%。"),
        effect3: None,
        effect4: Some("受到某个元素类型的伤害后，相应的抗性提升30%，持续10秒。该效果每10秒只能触发一次。"),
        effect5: None,
    };
}
