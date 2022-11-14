use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct MaidenBelovedEffect;

impl<T: Attribute> ArtifactEffect<T> for MaidenBelovedEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HealingBonus, "被怜爱的少女2", 0.15);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}

pub struct MaidenBeloved;

impl ArtifactTrait for MaidenBeloved {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(MaidenBelovedEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::MaidenBeloved,
        name_mona: "maidenBeloved",
        chs: "被怜爱的少女",
        flower: Some("远方的少女的心"),
        feather: Some("少女飘摇的思念"),
        sand: Some("少女苦短的良辰"),
        goblet: Some("少女片刻的闲暇"),
        head: Some("少女易逝的芳颜"),
        star: (4, 5),
        effect1: None,
        effect2: Some("角色造成的治疗效果提升15%"),
        effect3: None,
        effect4: Some("施放元素战技或元素爆发后的10秒内，队伍中所有角色受治疗效果加成提高20%。"),
        effect5: None,
        internal_id: 14004,
    };
}
