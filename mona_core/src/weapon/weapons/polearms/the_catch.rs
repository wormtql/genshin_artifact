use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct TheCatchEffect {}

impl TheCatchEffect {
    pub fn new() -> TheCatchEffect {
        TheCatchEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for TheCatchEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::BonusElementalBurst, "「渔获」被动", refine * 0.04 + 0.12);
        attribute.set_value_by(AttributeName::CriticalElementalBurst, "「渔获」被动", refine * 0.015 + 0.045);
    }
}

pub struct TheCatch;

impl WeaponTrait for TheCatch {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheCatch,
        internal_name: "Pole_Mori",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素爆发造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，元素爆发的暴击率提升<span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>。",
            en: "Increases Elemental Burst DMG by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> and Elemental Burst CRIT Rate by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "「渔获」",
            en: "\"The Catch\""
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheCatchEffect::new()))
    }
}
