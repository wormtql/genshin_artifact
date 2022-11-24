use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct EmblemOfSeveredFateEffect;

impl<T: Attribute> ArtifactEffect<T> for EmblemOfSeveredFateEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "绝缘之旗印2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_edge1(
            AttributeName::Recharge,
            AttributeName::BonusElementalBurst,
            Box::new(|x, _| (x * 0.25).min(0.75)),
            Box::new(|grad, x, _| (if x < 3.0 { grad * 0.25 } else { 0.0 }, 0.0)),
            "绝缘之旗印4"
        );
    }
}

pub struct EmblemOfSeveredFate;

impl ArtifactTrait for EmblemOfSeveredFate {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(EmblemOfSeveredFateEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::EmblemOfSeveredFate,
        name_mona: "emblemOfSeveredFate",
        chs: "绝缘之旗印",
        flower: Some("明威之镡"),
        feather: Some("切落之羽"),
        sand: Some("雷云之笼"),
        goblet: Some("绯花之壶"),
        head: Some("华饰之兜"),
        star: (4, 5),
        effect1: None,
        effect2: Some("元素充能效率提高20%。"),
        effect3: None,
        effect4: Some("基于元素充能效率的25%，提高元素爆发造成的伤害。至多通过这种方式获得75%提升。"),
        effect5: None,
        internal_id: 15020,
    };
}
