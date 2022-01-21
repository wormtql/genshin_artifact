use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const REDHORN_STONETHRESHER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage192,
    weapon_base: WeaponBaseATKFamily::ATK542,
    star: 5
};

pub struct RedhornStonethresherEffect {}

impl RedhornStonethresherEffect {
    pub fn new() -> RedhornStonethresherEffect {
        RedhornStonethresherEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for RedhornStonethresherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let def_bonus = refine * 0.07 + 0.21;
        attribute.add_def_percentage("赤角石溃杵被动", def_bonus);
        let bonus = refine * 0.1 + 0.3;
        attribute.set_value_by(AttributeName::DEFRatioNormalAttack, "赤角石溃杵被动", bonus);
        attribute.set_value_by(AttributeName::DEFRatioChargedAttack, "赤角石溃杵被动", bonus);
    }
}