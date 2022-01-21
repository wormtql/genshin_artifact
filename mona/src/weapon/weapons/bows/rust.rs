use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const RUST_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct RustEffect {}

impl RustEffect {
    pub fn new() -> RustEffect {
        RustEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for RustEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "弓藏被动", data.refine as f64 * 0.1 + 0.3);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "弓藏被动", -0.1);
    }
}