use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct DeepwoodMemoriesEffect {
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for DeepwoodMemoriesEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusDendro, "深林的记忆2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusDendro, "深林的记忆4", 0.3 * self.rate);
    }
}

pub struct DeepwoodMemories;

impl ArtifactTrait for DeepwoodMemories {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DeepwoodMemoriesEffect {
            rate: config.config_deepwood_memories.rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DeepwoodMemories,
        name_mona: "DeepwoodMemories",
        chs: "深林的记忆",
        flower: Some("迷宫的游人"),
        feather: Some("翠蔓的智者"),
        sand: Some("贤智的定期"),
        goblet: Some("迷误者之灯"),
        head: Some("月桂的宝冠"),
        star: (4, 5),
        effect1: None,
        effect2: Some(""),
        effect3: None,
        effect4: Some(""),
        effect5: None,
        internal_id: 15025,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}