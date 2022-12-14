use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct DesertPavilionChronicleEffect {
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for DesertPavilionChronicleEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusAnemo, "沙上楼阁史话2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "沙上楼阁史话4", 0.4 * self.rate);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "沙上楼阁史话4", 0.4 * self.rate);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "沙上楼阁史话4", 0.4 * self.rate);
    }
}

pub struct DesertPavilionChronicle;

impl ArtifactTrait for DesertPavilionChronicle {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DesertPavilionChronicleEffect {
            rate: config.config_desert_pavilion_chronicle.rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DesertPavilionChronicle,
        name_mona: "DesertPavilionChronicle",
        chs: "沙上楼阁史话",
        flower: Some("众王之都的开端"),
        feather: Some("黄金邦国的结末"),
        sand: Some("失落迷途的机芯"),
        goblet: Some("迷醉长梦的守护"),
        head: Some("流沙贵嗣的遗宝"),
        star: (4, 5),
        effect1: None,
        effect2: Some(""),
        effect3: None,
        effect4: Some(""),
        effect5: None,
        internal_id: 15027,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);
}