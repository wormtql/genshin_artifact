use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct MartialArtistEffect {
    pub rate: f64,
}

impl MartialArtistEffect {
    pub fn new(config: &ArtifactEffectConfig) -> MartialArtistEffect {
        MartialArtistEffect {
            rate: config.config_martial_artist.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for MartialArtistEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "武人2", 0.15);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "武人2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "武人4", self.rate * 0.25);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "武人4", self.rate * 0.25);
    }
}

pub struct MartialArtist;

impl ArtifactTrait for MartialArtist {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(MartialArtistEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::MartialArtist,
        name_mona: "martialArtist",
        chs: "武人",
        flower: Some("武人的红花"),
        feather: Some("武人的羽饰"),
        sand: Some("武人的水漏"),
        goblet: Some("武人的酒杯"),
        head: Some("武人的头巾"),
        star: (3, 4),
        effect1: None,
        effect2: Some("普通攻击与重击造成的伤害提高15%。"),
        effect3: None,
        effect4: Some("施放元素战技后的8秒内，普通攻击和重击造成的伤害提升25%。"),
        effect5: None
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
