use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const SKYWARD_ATLAS_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::ATK72,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5
};

pub struct SkywardAtlasEffect {}

impl SkywardAtlasEffect {
    pub fn new() -> SkywardAtlasEffect {
        SkywardAtlasEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardAtlasEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.add_elemental_bonus("天空之卷", value);
    }
}