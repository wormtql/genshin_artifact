pub mod swords;
pub mod claymores;
pub mod polearms;
pub mod catalysts;
pub mod bows;

pub use swords::*;
pub use claymores::*;
pub use polearms::*;
pub use catalysts::*;
pub use bows::*;

use crate::attribute::Attribute;
use super::weapon_name::WeaponName;
use super::weapon_config::WeaponConfig;
use super::weapon_effect::WeaponEffect;
use crate::character::character_common_data::CharacterCommonData;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;

pub fn get_static_data(name: WeaponName) -> WeaponStaticData {
    name.get_static_data()
}

pub fn get_effect<T: Attribute>(
    name: WeaponName,
    config: &WeaponConfig,
    character_common_data: &CharacterCommonData,
) -> Option<Box<dyn WeaponEffect<T>>> {
    name.get_effect(config, character_common_data)
}
