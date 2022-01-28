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
pub use beidou_default::BeidouDefaultTargetFunction;
pub use bennett_default::BennettDefaultTargetFunction;
pub use chongyun_default::ChongyunDefaultTargetFunction;
pub use diluc_default::DilucDefaultTargetFunction;
pub use diona_default::DionaDefaultTargetFunction;
pub use eula_default::EulaDefaultTargetFunction;
pub use fischl_default::FischlDefaultTargetFunction;
pub use gorou_default::GorouDefaultTargetFunction;
pub use jean_default::JeanDefaultTargetFunction;
pub use kaedehara_kazuha_default::KaedeharaKazuhaDefaultTargetFunction;
pub use kaeya_default::KaeyaDefaultTargetFunction;
pub use kamisato_ayaka_default::KamisatoAyakaDefaultTargetFunction;

pub mod albedo_default;
pub mod amber_default;
pub mod arataki_itto_default;
pub mod barbara_default;
pub mod beidou_default;
pub mod bennett_default;
pub mod chongyun_default;
pub mod diluc_default;
pub mod diona_default;
pub mod eula_default;
pub mod fischl_default;
pub mod ganyu_default;
pub mod gorou_default;
pub mod hu_tao_default;
pub mod jean_default;
pub mod kaedehara_kazuha_default;
pub mod kaeya_default;
pub mod kamisato_ayaka_default;

pub fn get_target_function(
    name: TargetFunctionName,
    character: &Character<SimpleAttributeGraph2>,
    weapon: &Weapon<SimpleAttributeGraph2>,
    config: &TargetFunctionConfig
) -> Box<dyn TargetFunction> {
    match name {
        // TargetFunctionName::Expectation => Box::new(ExpectationTargetFunction::new(config)),
        TargetFunctionName::GanyuDefault => Box::new(GanyuDefaultTargetFunction::new(config)),
        TargetFunctionName::HuTaoDefault => Box::new(HuTaoDefaultTargetFunction::new(config)),
        TargetFunctionName::AlbedoDefault => Box::new(AlbedoDefaultTargetFunction),
        TargetFunctionName::AmberDefault => Box::new(AmberDefaultTargetFunction::new()),
        TargetFunctionName::AratakiIttoDefault => Box::new(AratakiIttoDefaultTargetFunction),
        TargetFunctionName::BarbaraDefault => Box::new(BarbaraDefaultTargetFunction),
        TargetFunctionName::BeidouDefault => Box::new(BeidouDefaultTargetFunction),
        TargetFunctionName::BennettDefault => Box::new(BennettDefaultTargetFunction),
        TargetFunctionName::ChongyunDefault => Box::new(ChongyunDefaultTargetFunction),
        TargetFunctionName::DilucDefault => Box::new(DilucDefaultTargetFunction),
        TargetFunctionName::DionaDefault => Box::new(DionaDefaultTargetFunction),
        TargetFunctionName::EulaDefault => Box::new(EulaDefaultTargetFunction::new(character)),
        TargetFunctionName::FischlDefault => Box::new(FischlDefaultTargetFunction),
        TargetFunctionName::GorouDefault => Box::new(GorouDefaultTargetFunction::new(config)),
        TargetFunctionName::JeanDefault => Box::new(JeanDefaultTargetFunction::new(config)),
        TargetFunctionName::KaedeharaKazuhaDefault => Box::new(KaedeharaKazuhaDefaultTargetFunction),
        TargetFunctionName::KaeyaDefault => Box::new(KaeyaDefaultTargetFunction),
        TargetFunctionName::KamisatoAyakaDefault => Box::new(KamisatoAyakaDefaultTargetFunction),
        _ => panic!("unimplemented target function")
    }
}