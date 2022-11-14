use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ViridescentVenererEffect;

impl<T: Attribute> ArtifactEffect<T> for ViridescentVenererEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusAnemo, "翠绿之影2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceSwirlBase, "翠绿之影4", 0.6);
    }
}

pub struct ViridescentVenerer;

impl ArtifactTrait for ViridescentVenerer {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ViridescentVenererEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ViridescentVenerer,
        name_mona: "viridescentVenerer",
        chs: "翠绿之影",
        flower: Some("野花记忆的绿野"),
        feather: Some("猎人青翠的箭羽"),
        sand: Some("翠绿猎人的笃定"),
        goblet: Some("翠绿猎人的容器"),
        head: Some("翠绿的猎人之冠"),
        star: (4, 5),
        effect1: None,
        effect2: Some("获得15%风元素伤害加成。"),
        effect3: None,
        effect4: Some("扩散反应造成的伤害提升60%，根据扩散的元素类型，降低受到影响的敌人40%的对应元素抗性，持续10秒。"),
        effect5: None,
        internal_id: 15002,
    };
}
