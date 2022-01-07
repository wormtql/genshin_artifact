pub mod swords;

pub use swords::mistsplitter_reforged::{MistsplitterReforgedEffect};
use super::weapon_name::WeaponName;
use super::weapon_config::WeaponConfig;
use super::weapon_effect::WeaponEffect;
use crate::character::Character;

pub fn get_effect(
    name: WeaponName,
    config: &WeaponConfig,
    character: &Character,
) -> Box<dyn WeaponEffect> {
    match name {
        WeaponName::MistsplitterReforged => Box::new(MistsplitterReforgedEffect::new(config, character)),
        _ => panic!("unimplemented weapon"),
    }
}