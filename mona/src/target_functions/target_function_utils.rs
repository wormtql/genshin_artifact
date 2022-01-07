use super::target_function::TargetFunction;
use super::target_function_name::TargetFunctionName;
use super::target_function_config::TargetFunctionConfig;

use super::target_functions::ExpectationTargetFunction;
use crate::weapon::Weapon;
use crate::character::Character;

pub struct TargetFunctionUtils {}

impl TargetFunctionUtils {
    pub fn new_target_function(
        name: TargetFunctionName,
        character: &Character,
        weapon: &Weapon,
        config: &TargetFunctionConfig
    ) -> Box<dyn TargetFunction> {
        match name {
            TargetFunctionName::Expectation => Box::new(ExpectationTargetFunction::new(config)),
            // _ => unreachable!(),
        }
    }
}