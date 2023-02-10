use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SlingshotEffect {
    pub is_effect: bool,
    pub rate: f64,
}

impl SlingshotEffect {
    pub fn new(is_effect: bool, rate: f64) -> Self {
        Self { is_effect, rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for SlingshotEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        if self.is_effect {
            let value = (refine * 0.06 + 0.3) * self.rate;
            attribute.set_value_by(AttributeName::BonusNormalAttack, "弹弓被动等效", value);
            attribute.set_value_by(AttributeName::BonusChargedAttack, "弹弓被动等效", value);
        } else {
            attribute.set_value_by(AttributeName::BonusNormalAttack, "弹弓被动", -0.1);
            attribute.set_value_by(AttributeName::BonusChargedAttack, "弹弓被动", -0.1);
        }
    }
}

pub struct Slingshot;

impl WeaponTrait for Slingshot {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Slingshot,
        internal_name: "Bow_Sling",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate68),
        weapon_base: WeaponBaseATKFamily::ATK354,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击与重击的箭矢若在发射后的0.3秒内击中敌人，则造成的伤害增加<span style=\"color: #409EFF;\">36%-42%-48%-54%-60%</span>；否则，造成的伤害下降10%。",
            en: "If a Normal or Charged Attack hits a target within 0.3s of being fired, increases DMG by <span style=\"color: #409EFF;\">36%-42%-48%-54%-60%</span>. Otherwise, decreases DMG by 10%."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "弹弓",
            en: "Slingshot"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "is_effect",
            title: locale!(
                zh_cn: "0.3秒内命中",
                en: "Hit in 0.3s",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (is_effect, rate) = match *config {
            WeaponConfig::Slingshot { is_effect, rate } => (is_effect, rate),
            _ => (false, 0.0)
        };

        Some(Box::new(SlingshotEffect::new(is_effect, rate)))
    }
}
