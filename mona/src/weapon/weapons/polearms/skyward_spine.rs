use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const SKYWARD_SPINE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::Recharge80,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5
};

pub struct SkywardSpineEffect {}

impl SkywardSpineEffect {
    pub fn new() -> SkywardSpineEffect {
        SkywardSpineEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardSpineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "天空之脊被动", data.refine as f64 * 0.02 + 0.06);
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "天空之脊被动", 0.12);
    }
}