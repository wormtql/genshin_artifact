use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const THE_STRINGLESS_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::EM36,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct TheStringlessEffect {}

impl TheStringlessEffect {
    pub fn new() -> TheStringlessEffect {
        TheStringlessEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for TheStringlessEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.06 + 0.18;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "绝弦被动", value);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "绝弦被动", value);
    }
}