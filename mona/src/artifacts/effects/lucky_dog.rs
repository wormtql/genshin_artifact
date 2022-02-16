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
        chs: "幸运儿",
        flower: Some("幸运儿绿花"),
        feather: Some("幸运儿鹰羽"),
        sand: Some("幸运儿沙漏"),
        goblet: Some("幸运儿之杯"),
        head: Some("幸运儿银冠"),
        star: (1, 3),
        effect1: None,
        effect2: Some("防御力提高100点。"),
        effect3: None,
        effect4: Some("拾取摩拉时，恢复300点生命值。"),
        effect5: None
    };
}
