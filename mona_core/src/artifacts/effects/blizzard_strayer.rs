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
        name_locale: crate::common::i18n::locale!(
            zh_cn: "冰风迷途的勇士",
            en: "Blizzard Strayer",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "历经风雪的思念",
            en: "Snowswept Memory",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "摧冰而行的执望",
            en: "Icebreaker's Resolve",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "冰雪故园的终期",
            en: "Frozen Homeland's Demise",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "遍结寒霜的傲骨",
            en: "Frost-Weaved Dignity",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "破冰踏雪的回音",
            en: "Broken Rime's Echo",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%冰元素伤害加成。",
            en: "Cryo DMG Bonus +15%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "攻击处于冰元素影响下的敌人时，暴击率提高20%；若敌人处于冻结状态下，则暴击率额外提高20%。",
            en: "When a character attacks an opponent affected by Cryo, their CRIT Rate is increased by 20%. If the opponent is Frozen, CRIT Rate is increased by an additional 20%.",
        )),
        effect5: None,
        internal_id: 14001,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "critical_bonus",
            title: crate::common::i18n::locale!(
                zh_cn: "等效暴击率",
                en: "Equivalent Crit Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 0.4, default: 0.0 }
        }
    ]);
}
