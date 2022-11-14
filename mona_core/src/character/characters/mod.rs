use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{ChangeAttribute, Element};
use crate::attribute::Attribute;

use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub mod traveller;
pub mod pyro;
pub mod electro;
pub mod cryo;
pub mod anemo;
pub mod geo;
pub mod hydro;
pub mod dendro;

pub use anemo::*;
pub use cryo::*;
pub use dendro::*;
pub use electro::*;
pub use geo::*;
pub use hydro::*;
pub use pyro::*;
pub use traveller::*;

pub fn get_static_data(name: CharacterName) -> CharacterStaticData {
    name.get_static_data()
}

pub fn get_effect<T: Attribute>(
    name: CharacterName,
    common_data: &CharacterCommonData,
    config: &CharacterConfig
) -> Option<Box<dyn ChangeAttribute<T>>> {
    name.get_effect(common_data, config)
}

pub fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, skill_index: usize, skill_config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
    CharacterName::damage::<D>(context, skill_index, skill_config, fumo)
}

pub fn get_target_function_by_role(
    role_index: usize,
    team: &TeamQuantization,
    character: &CharacterCommonData,
    weapon: &WeaponCommonData
) -> Box<dyn TargetFunction> {
    CharacterName::get_target_function_by_role(role_index, team, character, weapon)
}

pub fn get_skill_from_str(name: CharacterName, s: &str) -> Option<usize> {
    name.get_skill_from_str(s)
}
