use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct TenacityOfTheMillelithEffect {
    pub rate: f64,
}

impl TenacityOfTheMillelithEffect {
    pub fn new(config: &ArtifactEffectConfig) -> TenacityOfTheMillelithEffect {
        TenacityOfTheMillelithEffect {
            rate: config.config_tenacity_of_the_millelith.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for TenacityOfTheMillelithEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_hp_percentage("千岩牢固2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("千岩牢固4", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ShieldStrength, "千岩牢固4", self.rate * 0.3);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "千岩牢固4", self.rate * 0.2);
    }
}

pub struct TenacityOfTheMillelith;

impl ArtifactTrait for TenacityOfTheMillelith {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(TenacityOfTheMillelithEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TenacityOfTheMillelith,
        name_mona: "tenacityOfTheMillelith",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "千岩牢固",
            en: "Tenacity of the Millelith",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "勋绩之花",
            en: "Flower of Accolades",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "昭武翎羽",
            en: "Ceremonial War-Plume",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "金铜时晷",
            en: "Orichalceous Time-Dial",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "盟誓金爵",
            en: "Noble's Pledging Vessel",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "将帅兜鍪",
            en: "General's Ancient Helm",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "生命值提升20%",
            en: "HP increased by 20%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技命中敌人后，使队伍中附近的所有角色攻击力提升20%，护盾强效提升30%，持续3秒。该效果每0.5秒至多触发一次。装备此圣遗物套装的角色处于队伍后台时，依然能触发该效果。",
            en: "When an Elemental Skill hits an opponent, the ATK of all nearby party members is increased by 20% and their Shield Strength is increased by 30% for 3s. This effect can be triggered once every 0.5s. This effect can still be triggered even when the character who is using this artifact set is not on the field.",
        )),
        effect5: None,
        internal_id: 15017,
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
