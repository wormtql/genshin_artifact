use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct ShimenawasReminiscenceEffect {
    pub rate: f64,
}

impl ShimenawasReminiscenceEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ShimenawasReminiscenceEffect {
        ShimenawasReminiscenceEffect {
            rate: config.config_shimenawas_reminiscence.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ShimenawasReminiscenceEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("追忆之注连2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "追忆之注连4", self.rate * 0.5);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "追忆之注连4", self.rate * 0.5);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "追忆之注连4", self.rate * 0.5);
    }
}

pub struct ShimenawasReminiscence;

impl ArtifactTrait for ShimenawasReminiscence {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ShimenawasReminiscenceEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ShimenawasReminiscence,
        name_mona: "shimenawaReminiscence",
        chs: "追忆之注连",
        flower: Some("羁缠之花"),
        feather: Some("思忆之矢"),
        sand: Some("朝露之时"),
        goblet: Some("祈望之心"),
        head: Some("无常之面"),
        star: (4, 5),
        effect1: None,
        effect2: Some("攻击力提高18%。"),
        effect3: None,
        effect4: Some("施放元素战技时，如果角色的元素能量高于或等于15点，则会流失15点元素能量，使接下来的10秒内，普通攻击、重击、下落攻击造成的伤害提高50%。"),
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
