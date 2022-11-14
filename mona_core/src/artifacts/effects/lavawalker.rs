use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct LavawalkerEffect {
    pub rate: f64,
}

impl LavawalkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> LavawalkerEffect {
        LavawalkerEffect {
            rate: config.config_lavawalker.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for LavawalkerEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "渡过烈火的贤人4", self.rate * 0.35);
    }
}

pub struct Lavawalker;

impl ArtifactTrait for Lavawalker {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(LavawalkerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Lavawalker,
        name_mona: "lavaWalker",
        chs: "渡过烈火的贤人",
        flower: Some("渡火者的决绝"),
        feather: Some("渡火者的解脱"),
        sand: Some("渡火者的煎熬"),
        goblet: Some("渡火者的醒悟"),
        head: Some("渡火者的智慧"),
        star: (4, 5),
        effect1: None,
        effect2: Some("火元素抗性提高40%。"),
        effect3: None,
        effect4: Some("对处于火元素影响下的敌人造成的伤害提升35%。"),
        effect5: None,
        internal_id: 14003,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a7",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
