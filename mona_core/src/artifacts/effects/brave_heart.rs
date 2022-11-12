use serde::__private::de::IdentifierDeserializer;
use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct BraveHeartEffect {
    pub rate: f64,
}

impl BraveHeartEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BraveHeartEffect {
        BraveHeartEffect {
            rate: config.config_brave_heart.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BraveHeartEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("勇士之心2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "勇士之心4", self.rate * 0.3);
    }
}

pub struct BraveHeart;

impl ArtifactTrait for BraveHeart {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BraveHeartEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BraveHeart,
        name_mona: "braveHeart",
        chs: "勇士之心",
        flower: Some("勇士的勋章"),
        feather: Some("勇士的期许"),
        sand: Some("勇士的坚毅"),
        goblet: Some("勇士的壮行"),
        head: Some("勇士的冠冕"),
        star: (3, 4),
        effect1: None,
        effect2: Some("攻击力提高18%。"),
        effect3: None,
        effect4: Some("对生命值高于50%的敌人，造成的伤害增加30%。"),
        effect5: None,
        internal_id: 10002,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
