use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ThunderingFuryEffect;

impl<T: Attribute> ArtifactEffect<T> for ThunderingFuryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElectro, "如雷的盛怒2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceOverload, "如雷的盛怒4", 0.4);
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "如雷的盛怒4", 0.4);
        attribute.set_value_by(AttributeName::EnhanceSuperconduct, "如雷的盛怒4", 0.4);
        attribute.set_value_by(AttributeName::EnhanceHyperbloom, "如雷的盛怒4", 0.4);
        attribute.set_value_by(AttributeName::EnhanceAggravate, "如雷的盛怒4", 0.2);
    }
}

pub struct ThunderingFury;

impl ArtifactTrait for ThunderingFury {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ThunderingFuryEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ThunderingFury,
        name_mona: "thunderingFury",
        chs: "如雷的盛怒",
        flower: Some("雷鸟的怜悯"),
        feather: Some("雷灾的孑遗"),
        sand: Some("雷霆的时计"),
        goblet: Some("降雷的凶兆"),
        head: Some("唤雷的头冠"),
        star: (4, 5),
        effect1: None,
        effect2: Some("获得15%雷元素伤害加成。"),
        effect3: None,
        effect4: Some("超载、感电、超导反应造成的伤害提升40%。触发这些元素反应时，元素战技冷却时间减少1秒。该效果每0.8秒最多触发一次。"),
        effect5: None
    };
}
