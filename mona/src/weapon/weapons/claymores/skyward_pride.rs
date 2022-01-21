use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const SKYWARD_PRIDE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::Recharge80,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5
};

pub struct  SkywardPrideEffect {}

impl SkywardPrideEffect {
    pub fn new() -> SkywardPrideEffect {
        SkywardPrideEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardPrideEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.02 + 0.06;
        attribute.set_value_by(AttributeName::BonusBase, "天空之傲被动", value);
    }
}