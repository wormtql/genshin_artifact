use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct ThundersootherEffect {
    pub rate: f64,
}

impl ThundersootherEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ThundersootherEffect {
        ThundersootherEffect {
            rate: config.config_thundersoother.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ThundersootherEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "平息鸣雷的尊者4", self.rate * 0.35);
    }
}

pub struct Thundersoother;

impl ArtifactTrait for Thundersoother {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ThundersootherEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Thundersoother,
        name_mona: "thunderSmoother",
        chs: "平息鸣雷的尊者",
        flower: Some("平雷之心"),
        feather: Some("平雷之羽"),
        sand: Some("平雷之刻"),
        goblet: Some("平雷之器"),
        head: Some("平雷之冠"),
        star: (4, 5),
        effect1: None,
        effect2: Some("雷元素抗性提高40%。"),
        effect3: None,
        effect4: Some("对处于雷元素影响下的敌人造成的伤害提升35%。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a11",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
