use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const THE_CATCH_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::Recharge100,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

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