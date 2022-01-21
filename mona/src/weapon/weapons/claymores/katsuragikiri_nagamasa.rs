use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const KATSURAGIKIRI_NAGASAMA_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::Recharge100,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct KatsuragikiriNagamasaEffect {}

impl KatsuragikiriNagamasaEffect {
    pub fn new() -> KatsuragikiriNagamasaEffect {
        KatsuragikiriNagamasaEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for KatsuragikiriNagamasaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "桂木斩长正被动", value);
    }
}