use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const SKYWARD_BLADE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::Recharge120,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct SkywardBladeEffect {}

impl SkywardBladeEffect {
    pub fn new() -> SkywardBladeEffect {
        SkywardBladeEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let crit = data.refine as f64 * 0.01 + 0.03;
        attribute.set_value_by(AttributeName::CriticalBase, "天空之刃被动", crit);
    }
}