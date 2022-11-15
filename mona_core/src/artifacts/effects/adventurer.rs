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
        chs: "冒险家",
        flower: Some("冒险家之花"),
        feather: Some("冒险家尾羽"),
        sand: Some("冒险家怀表"),
        goblet: Some("冒险家金杯"),
        head: Some("冒险家头带"),
        star: (1, 3),
        effect1: None,
        effect2: Some("生命值上限提升1000点。"),
        effect3: None,
        effect4: Some("生命值上限提升1000点。"),
        effect5: None,
        internal_id: 10010,
    };
}