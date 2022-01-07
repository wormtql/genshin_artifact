use super::target_function_config::TargetFunctionConfig;
use super::target_function::TargetFunction;
use super::target_function_name::TargetFunctionName;
use crate::common::{Element, SkillType};

pub use expectation::ExpectationTargetFunction;

pub mod expectation;


pub fn get_target_function(name: TargetFunctionName, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
    match name {
        TargetFunctionName::Expectation => Box::new(ExpectationTargetFunction::new(config)),
    }
}