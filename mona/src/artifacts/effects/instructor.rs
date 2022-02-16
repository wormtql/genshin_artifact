use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct InstructorEffect {
    pub rate: f64,
}

impl InstructorEffect {
    pub fn new(config: &ArtifactEffectConfig) -> InstructorEffect {
        InstructorEffect {
            rate: config.config_instructor.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for InstructorEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "教官2", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "教官4", self.rate * 120.0);
    }
}

pub struct Instructor;

impl ArtifactTrait for Instructor {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(InstructorEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Instructor,
        name_mona: "instructor",
        chs: "教官",
        flower: Some("教官的胸花"),
        feather: Some("教官的羽饰"),
        sand: Some("教官的怀表"),
        goblet: Some("教官的茶杯"),
        head: Some("教官的帽子"),
        star: (3, 4),
        effect1: None,
        effect2: Some("元素精通提高80点。"),
        effect3: None,
        effect4: Some("触发元素反应后。队伍中所有角色元素精通提高120点，持续8秒。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "效果应用比例",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
