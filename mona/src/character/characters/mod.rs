use crate::character::{CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;

pub use mona::{MonaEffect, MONA_STATIC_DATA, MONA_SKILL};
pub use albedo::{ALBEDO_STATIC_DATA, ALBEDO_SKILL};
pub use aloy::{ALOY_STATIC_DATA, ALOY_SKILL};
pub use amber::{AMBER_STATIC_DATA, AMBER_SKILL, AmberEffect};

pub mod mona;
pub mod albedo;
pub mod aloy;
pub mod amber;

pub fn get_static_data(name: CharacterName) -> CharacterStaticData {
    match name {
        CharacterName::Mona => MONA_STATIC_DATA,
        CharacterName::Albedo => ALBEDO_STATIC_DATA,
        CharacterName::Aloy => ALOY_STATIC_DATA,
        CharacterName::Amber => AMBER_STATIC_DATA,
        _ => panic!("unimplemented character"),
    }
}

pub fn get_effect(name: CharacterName, common_data: &CharacterCommonData) -> Option<Box<dyn ChangeAttribute>> {
    match name {
        CharacterName::Mona => Some(Box::new(MonaEffect::new(common_data))),
        CharacterName::Amber => Some(Box::new(AmberEffect::new(common_data))),
        _ => None,
    }
}