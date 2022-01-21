use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const THE_BLACK_SWORD_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate60,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct TheBlackSwordEffect {}

impl TheBlackSwordEffect {
    pub fn new() -> TheBlackSwordEffect {
        TheBlackSwordEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for TheBlackSwordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let bonus = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "黑剑被动", bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "黑剑被动", bonus);
    }
}