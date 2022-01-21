use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const ROYAL_GRIMOIRE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::ATK60,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

// todo
