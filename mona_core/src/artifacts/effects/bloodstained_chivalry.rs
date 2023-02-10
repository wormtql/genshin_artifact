use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct BloodstainedChivalryEffect {
    pub rate: f64,
}

impl BloodstainedChivalryEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BloodstainedChivalryEffect {
        BloodstainedChivalryEffect {
            rate: config.config_bloodstained_chivalry.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BloodstainedChivalryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "染血的骑士道2", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusChargedAttack, "染血的骑士道4", self.rate * 0.5);
    }
}

pub struct BloodstainedChivalry;

impl ArtifactTrait for BloodstainedChivalry {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BloodstainedChivalryEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BloodstainedChivalry,
        name_mona: "bloodstainedChivalry",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "染血的骑士道",
            en: "Bloodstained Chivalry",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "染血的铁之心",
            en: "Bloodstained Flower of Iron",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "染血的黑之羽",
            en: "Bloodstained Black Plume",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "骑士染血之时",
            en: "Bloodstained Final Hour",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "染血骑士之杯",
            en: "Bloodstained Chevalier's Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "染血的铁假面",
            en: "Bloodstained Iron Mask",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "造成的物理伤害提高25%。",
            en: "Physical DMG +25%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "击败敌人后的10秒内，施放重击时不消耗体力，且重击造成的伤害提升50%。",
            en: "After defeating an opponent, increases Charged Attack DMG by 50%, and reduces its Stamina cost to 0 for 10s.",
        )),
        effect5: None,
        internal_id: 15008,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
