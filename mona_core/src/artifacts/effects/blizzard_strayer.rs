use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BlizzardStrayerEffect {
    pub crit_bonus: f64,
}

impl BlizzardStrayerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BlizzardStrayerEffect {
        BlizzardStrayerEffect {
            crit_bonus: config.config_blizzard_strayer.critical_bonus
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BlizzardStrayerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusCryo, "冰风迷途的勇士2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalAttacking, "冰风迷途的勇士4", self.crit_bonus);
    }
}

pub struct BlizzardStrayer;

impl ArtifactTrait for BlizzardStrayer {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BlizzardStrayerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BlizzardStrayer,
        name_mona: "blizzardStrayer",
        chs: "冰风迷途的勇士",
        flower: Some("历经风雪的思念"),
        feather: Some("摧冰而行的执望"),
        sand: Some("冰雪故园的终期"),
        goblet: Some("遍结寒霜的傲骨"),
        head: Some("破冰踏雪的回音"),
        star: (4, 5),
        effect1: None,
        effect2: Some("获得15%冰元素伤害加成"),
        effect3: None,
        effect4: Some("攻击处于冰元素影响状态下的敌人时，暴击率提高20%；若敌人处于冰冻状态下，暴击率额外提高20%。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "critical_bonus",
            title: "等效暴击率",
            config: ItemConfigType::Float { min: 0.0, max: 0.4, default: 0.0 }
        }
    ]);
}
