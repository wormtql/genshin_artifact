use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct FlowerOfParadiseLostEffect {
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for FlowerOfParadiseLostEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "乐园遗落之花2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceBloom, "乐园遗落之花4", 0.4 * (1.0 + 0.25 * self.stack));
        attribute.set_value_by(AttributeName::EnhanceBurgeon, "乐园遗落之花4", 0.4 * (1.0 + 0.25 * self.stack));
        attribute.set_value_by(AttributeName::EnhanceHyperbloom, "乐园遗落之花4", 0.4 * (1.0 + 0.25 * self.stack));
    }
}

pub struct FlowerOfParadiseLost;

impl ArtifactTrait for FlowerOfParadiseLost {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(FlowerOfParadiseLostEffect {
            stack: config.config_flower_of_paradise_lost.stack,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::FlowerOfParadiseLost,
        name_mona: "FlowerOfParadiseLost",
        chs: "乐园遗落之花",
        flower: Some("月女的华彩"),
        feather: Some("谢落的筵席"),
        sand: Some("凝结的时刻"),
        goblet: Some("守秘的魔瓶"),
        head: Some("紫晶的花冠"),
        star: (4, 5),
        effect1: None,
        effect2: Some(""),
        effect3: None,
        effect4: Some(""),
        effect5: None,
        internal_id: 15028,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "a4",
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 },
        }
    ]);
}