use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct GamblerEffect;

impl<T: Attribute> ArtifactEffect<T> for GamblerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalSkill, "赌徒2", 0.2);
    }
}

pub struct Gambler;

impl ArtifactTrait for Gambler {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GamblerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Gambler,
        name_mona: "gambler",
        chs: "赌徒",
        flower: Some("赌徒的胸花"),
        feather: Some("赌徒的羽饰"),
        sand: Some("赌徒的怀表"),
        goblet: Some("赌徒的骰蛊"),
        head: Some("赌徒的耳环"),
        star: (3, 4),
        effect1: None,
        effect2: Some("元素战技造成的伤害提升20%"),
        effect3: None,
        effect4: Some("击败敌人时，有100%概率清除元素战技的冷却时间。该效果每15秒至多触发一次。"),
        effect5: None,
    };
}
