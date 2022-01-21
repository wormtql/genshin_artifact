use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const SKYWARD_HARP_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate48,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5
};

pub struct SkywardHarpEffect {}

impl SkywardHarpEffect {
    pub fn new() -> SkywardHarpEffect {
        SkywardHarpEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardHarpEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalDamageBase, "天空之翼被动", data.refine as f64 * 0.05 + 0.15);
    }
}