use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const KITAIN_CROSS_SPEAR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::EM24,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct KitainCrossSpearEffect {}

impl KitainCrossSpearEffect {
    pub fn new() -> KitainCrossSpearEffect {
        KitainCrossSpearEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for KitainCrossSpearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "喜多院十文字被动", value);
    }
}