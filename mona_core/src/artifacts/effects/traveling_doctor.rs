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
        chs: "游医",
        flower: Some("游医的银莲"),
        feather: Some("游医的枭羽"),
        sand: Some("游医的怀钟"),
        goblet: Some("游医的药壶"),
        head: Some("游医的方巾"),
        star: (1, 3),
        effect1: None,
        effect2: Some("角色受到的治疗效果提高20%。"),
        effect3: None,
        effect4: Some("施放元素爆发时，恢复20%生命值。"),
        effect5: None,
        internal_id: 10013,
    };
}
