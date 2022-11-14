use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct GildedDreamsEffect {
    pub same_count: usize,
    pub diff_count: usize,
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for GildedDreamsEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "饰金之梦2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        if self.same_count > 0 {
            attribute.add_atk_percentage("饰金之梦4", (self.same_count.min(3) as f64 * 0.14) * self.rate);
        }
        if self.diff_count > 0 {
            attribute.set_value_by(AttributeName::ElementalMastery, "饰金之梦4", (self.diff_count.min(3) as f64 * 50.0) * self.rate);
        }
    }
}

pub struct GildedDreams;

impl ArtifactTrait for GildedDreams {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GildedDreamsEffect {
            same_count: config.config_gilded_dreams.same_count,
            diff_count: config.config_gilded_dreams.diff_count,
            rate: config.config_gilded_dreams.rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::GildedDreams,
        name_mona: "GildedDreams",
        chs: "",
        flower: Some("梦中的铁花"),
        feather: Some("裁断的翎羽"),
        sand: Some("沉金的岁月"),
        goblet: Some("如蜜的终宴"),
        head: Some("沙王的投影"),
        star: (4, 5),
        effect1: None,
        effect2: Some(""),
        effect3: None,
        effect4: Some(""),
        effect5: None,
        internal_id: 15026,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "same_count",
            title: "a14",
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "diff_count",
            title: "a15",
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}