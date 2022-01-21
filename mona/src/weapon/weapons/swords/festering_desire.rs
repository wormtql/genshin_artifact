use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const FESTERING_DESIRE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::Recharge100,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct FesteringDesireEffect {}

impl FesteringDesireEffect {
    pub fn new() -> FesteringDesireEffect {
        FesteringDesireEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for FesteringDesireEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::CriticalElementalSkill, "腐殖之剑被动", bonus1);
        let bonus2 = refine * 0.04 + 0.12;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "腐殖之剑被动", bonus2);
    }
}