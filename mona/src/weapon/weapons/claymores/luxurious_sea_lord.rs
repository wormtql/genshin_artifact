use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const LUXURIOUS_SEA_LORD_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::ATK120,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct LuxuriousSeaLordEffect {}

impl LuxuriousSeaLordEffect {
    pub fn new() -> LuxuriousSeaLordEffect {
        LuxuriousSeaLordEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for LuxuriousSeaLordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusElementalBurst, "衔珠海皇被动", value);
    }
}