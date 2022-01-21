use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const AQUILA_FAVONIA_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::PhysicalBonus90,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5,
};

pub struct AquilaFavoniaEffect {}

impl AquilaFavoniaEffect {
    pub fn new() -> AquilaFavoniaEffect {
        AquilaFavoniaEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for AquilaFavoniaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("风鹰剑被动", value);
    }
}