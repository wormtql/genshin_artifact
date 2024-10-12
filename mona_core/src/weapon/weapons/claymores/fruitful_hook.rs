use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

struct FruitfulHookEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for FruitfulHookEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus = 0.12 + refine * 0.04;
        attribute.set_value_by(AttributeName::CriticalPlungingAttack, "硕果钩被动", bonus);

        attribute.set_value_by(AttributeName::BonusNormalAttack, "硕果钩被动", bonus * self.rate);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "硕果钩被动", bonus * self.rate);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "硕果钩被动", bonus * self.rate);
    }
}

pub struct FruitfulHook;

impl WeaponTrait for FruitfulHook {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FruitfulHook,
        internal_name: "Claymore_Umpakati",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "下落攻击的暴击率提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>；下落攻击命中敌人后，普通攻击、重击、下落攻击造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，持续10秒。",
            en: "Increase Plunging Attack CRIT Rate by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>; After a Plunging Attack hits an opponent, Normal, Charged, and Plunging Attack DMG increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 10s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "硕果钩",
            en: "Fruitful Hook"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::FruitfulHook { rate } => Some(Box::new(FruitfulHookEffect {
                rate
            })),
            _ => None
        }
    }
}
