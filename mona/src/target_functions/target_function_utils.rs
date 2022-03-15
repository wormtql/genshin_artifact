use crate::attribute::{SimpleAttributeGraph2};
use super::target_function::TargetFunction;
use super::target_function_name::TargetFunctionName;
use super::target_function_config::TargetFunctionConfig;

use crate::weapon::Weapon;
use crate::character::Character;
use crate::target_functions::target_functions::get_target_function;

pub struct TargetFunctionUtils {}

impl TargetFunctionUtils {
    pub fn new_target_function(
        name: TargetFunctionName,
        character: &Character<SimpleAttributeGraph2>,
        weapon: &Weapon<SimpleAttributeGraph2>,
        config: &TargetFunctionConfig
    ) -> Box<dyn TargetFunction> {
        get_target_function(
            name, character, weapon, config
        )
    }
}