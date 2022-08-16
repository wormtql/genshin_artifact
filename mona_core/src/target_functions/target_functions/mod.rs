use super::target_function_config::TargetFunctionConfig;
use super::target_function::TargetFunction;
use super::target_function_name::TargetFunctionName;
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::Character;
use crate::weapon::Weapon;

pub mod pyro;
pub mod electro;
pub mod geo;
pub mod anemo;
pub mod cryo;
pub mod hydro;
pub mod common;
pub mod dendro;

pub use pyro::*;
pub use electro::*;
pub use geo::*;
pub use anemo::*;
pub use cryo::*;
pub use hydro::*;
pub use common::*;
pub use dendro::*;

pub fn get_target_function(
    name: TargetFunctionName,
    character: &Character<SimpleAttributeGraph2>,
    weapon: &Weapon<SimpleAttributeGraph2>,
    config: &TargetFunctionConfig
) -> Box<dyn TargetFunction> {
    name.create(&character.common_data, &weapon.common_data, config)
}
