use super::target_function_config::TargetFunctionConfig;
use super::target_function::TargetFunction;
use super::target_function_name::TargetFunctionName;
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::Character;
use crate::weapon::Weapon;

pub use ganyu_default::GanyuDefaultTargetFunction;
pub use hu_tao_default::HuTaoDefaultTargetFunction;
pub use albedo_default::AlbedoDefaultTargetFunction;
pub use amber_default::AmberDefaultTargetFunction;
pub use arataki_itto_default::AratakiIttoDefaultTargetFunction;
pub use barbara_default::BarbaraDefaultTargetFunction;

pub mod ganyu_default;
pub mod hu_tao_default;
pub mod albedo_default;
pub mod amber_default;
pub mod arataki_itto_default;
pub mod barbara_default;


pub fn get_target_function(
    name: TargetFunctionName,
    character: &Character<SimpleAttributeGraph2>,
    weapon: &Weapon<SimpleAttributeGraph2>,
    config: &TargetFunctionConfig
) -> Box<dyn TargetFunction> {
    match name {
        // TargetFunctionName::Expectation => Box::new(ExpectationTargetFunction::new(config)),
        TargetFunctionName::GanyuDefault => Box::new(GanyuDefaultTargetFunction::new(character, config)),
        TargetFunctionName::HuTaoDefault => Box::new(HuTaoDefaultTargetFunction::new(character, config)),
        TargetFunctionName::AlbedoDefault => Box::new(AlbedoDefaultTargetFunction::new()),
        TargetFunctionName::AmberDefault => Box::new(AmberDefaultTargetFunction::new()),
        TargetFunctionName::AratakiIttoDefault => Box::new(AratakiIttoDefaultTargetFunction),
        TargetFunctionName::BarbaraDefault => Box::new(BarbaraDefaultTargetFunction),
        _ => panic!("unimplemented target function")
    }
}