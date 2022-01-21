use crate::common::WeaponType;
use super::weapon_sub_stat::{WeaponSubStatFamily};
use super::weapon_base_atk::WeaponBaseATKFamily;

pub struct WeaponStaticData {
    pub weapon_type: WeaponType,
    pub weapon_sub_stat: WeaponSubStatFamily,
    pub weapon_base: WeaponBaseATKFamily,
    pub star: usize,
}