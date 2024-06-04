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

pub struct AbsolutionEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for AbsolutionEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::CriticalDamageBase, "赦罪被动", 0.15 + refine * 0.05);
        attribute.set_value_by(AttributeName::BonusBase, "赦罪被动", self.stack * (0.12 + 0.04 * refine));
    }
}

pub struct Absolution;

impl WeaponTrait for Absolution {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Absolution,
        internal_name: "Absolution",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "暴击伤害提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>；生命之契的数值增加时，装备者造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。该效果持续6秒，至多叠加3次。",
            en: "CRIT DMG increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Increasing the value of a Bond of Life increases the DMG the equipping character deals by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 6s. Max 3 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "赦罪",
            en: "Absolution"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::Absolution { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(AbsolutionEffect {
            stack
        }))
    }
}
