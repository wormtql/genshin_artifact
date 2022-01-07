use crate::common::WeaponType;
use super::weapon_sub_stat::{WeaponSubStatFamily};
use super::weapon_base_atk::WeaponBaskATKFamily;
use super::weapon_name::WeaponName;

pub struct WeaponData {
    pub weapon_type: WeaponType,
    pub weapon_sub_stat: WeaponSubStatFamily,
    pub weapon_base: WeaponBaskATKFamily,
}

impl WeaponData {
    pub fn weapon_data(name: WeaponName) -> WeaponData {
        match name {
            WeaponName::MistsplitterReforged => WeaponData {
                weapon_type: WeaponType::Sword,
                weapon_sub_stat: WeaponSubStatFamily::CriticalDamage96,
                weapon_base: WeaponBaskATKFamily::ATK674,
            },
            _ => panic!("unimplemented weapon"),
        }
    }
}