use smallvec::SmallVec;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::WeaponName;
use super::weapon_sub_stat::{WeaponSubStatFamily};
use super::weapon_base_atk::WeaponBaseATKFamily;

pub struct WeaponStaticData {
    pub name: WeaponName,
    pub weapon_type: WeaponType,
    pub weapon_sub_stat: WeaponSubStatFamily,
    pub weapon_base: WeaponBaseATKFamily,
    pub star: usize,
    pub effect: Option<&'static str>,
    pub chs: &'static str,
}
