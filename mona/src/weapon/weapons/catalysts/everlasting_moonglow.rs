use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const EVERLASTING_MOONGLOW_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::HP108,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct EverlastingMoonglowEffect {}

impl EverlastingMoonglowEffect {
    pub fn new() -> EverlastingMoonglowEffect {
        EverlastingMoonglowEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for EverlastingMoonglowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::HealingBonus, "不灭月华被动", refine * 0.025 + 0.075);
        attribute.set_value_by(AttributeName::HPRatioNormalAttack, "不灭月华被动", refine * 0.005 + 0.005);
    }
}