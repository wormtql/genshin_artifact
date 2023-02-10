use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct MitternachtsWaltzEffect {
    rate1: f64,
    rate2: f64
}

impl MitternachtsWaltzEffect {
    pub fn new(config: &WeaponConfig) -> MitternachtsWaltzEffect {
        match *config {
            WeaponConfig::MitternachtsWaltz { rate1, rate2 } => MitternachtsWaltzEffect {
                rate1,
                rate2
            },
            _ => MitternachtsWaltzEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MitternachtsWaltzEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "幽夜华尔兹被动等效", value * self.rate1);
        attribute.set_value_by(AttributeName::BonusNormalAttack, "幽夜华尔兹被动等效", value * self.rate2);
    }
}

pub struct MitternachtsWaltz;

impl WeaponTrait for MitternachtsWaltz {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MitternachtsWaltz,
        internal_name: "Bow_Nachtblind",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus113),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击命中敌人后的5秒内，元素战技造成的伤害提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>；元素战技命中敌人后的5秒内，普通攻击造成的伤害提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。",
            en: "Normal Attack hits on opponents increase Elemental Skill DMG by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> for 5s. Elemental Skill hits on opponents increase Normal Attack DMG by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> for 5s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "幽夜华尔兹",
            en: "Mitternachts Waltz"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "效果1比例",
                en: "Effect1 Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果2比例",
                en: "Effect2 Ratio"
            ),
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MitternachtsWaltzEffect::new(config)))
    }
}
