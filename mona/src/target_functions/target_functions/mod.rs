use super::target_function_config::TargetFunctionConfig;
use super::target_function::TargetFunction;
use super::target_function_name::TargetFunctionName;
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::Character;
use crate::weapon::Weapon;

pub use ganyu_default::GanyuDefaultTargetFunction;
pub use hu_tao_default::HuTaoDefaultTargetFunction;
pub use albedo_default::AlbedoDefaultTargetFunction;
pub use aloy_default::AloyDefaultTargetFunction;
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
pub use keqing_default::KeqingDefaultTargetFunction;
pub use klee_default::KleeDefaultTargetFunction;
pub use kujou_sara_default::KujouSaraDefaultTargetFunction;
pub use lisa_default::LisaDefaultTargetFunction;
pub use mona_default::MonaDefaultTargetFunction;
pub use ningguang_default::NingguangDefaultTargetFunction;
pub use noelle_default::NoelleDefaultTargetFunction;
pub use qiqi_default::QiqiDefaultTargetFunction;
pub use raiden_shogun_default::RaidenShogunDefaultTargetFunction;
pub use razor_default::RazorDefaultTargetFunction;
pub use rosaria_default::RosariaDefaultTargetFunction;
pub use sangonomiya_kokomi_default::SangonomiyaKokomiDefaultTargetFunction;
pub use sayu_default::SayuDefaultTargetFunction;
pub use sucrose_default::SucroseDefaultTargetFunction;
pub use tartaglia_default::TartagliaDefaultTargetFunction;
pub use thoma_default::ThomaDefaultTargetFunction;
pub use venti_default::VentiDefaultTargetFunction;
pub use xiangling_default::XianglingDefaultTargetFunction;
pub use xiao_default::XiaoDefaultTargetFunction;
pub use xingqiu_default::XingqiuDefaultTargetFunction;
pub use xinyan_default::XinyanDefaultTargetFunction;
pub use yanfei_default::YanfeiDefaultTargetFunction;
pub use yoimiya_default::YoimiyaDefaultTargetFunction;
pub use yunjin_default::YunjinDefaultTargetFunction;
pub use zhongli_default::ZhongliDefaultTargetFunction;
pub use shenhe_default::ShenheDefaultTargetFunction;

pub mod albedo_default;
pub mod aloy_default;
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
pub mod keqing_default;
pub mod klee_default;
pub mod kujou_sara_default;
pub mod lisa_default;
pub mod mona_default;
pub mod ningguang_default;
pub mod noelle_default;
pub mod qiqi_default;
pub mod raiden_shogun_default;
pub mod razor_default;
pub mod rosaria_default;
pub mod sangonomiya_kokomi_default;
pub mod sayu_default;
pub mod shenhe_default;
pub mod sucrose_default;
pub mod tartaglia_default;
pub mod thoma_default;
pub mod venti_default;
pub mod xiangling_default;
pub mod xiao_default;
pub mod xingqiu_default;
pub mod xinyan_default;
pub mod yanfei_default;
pub mod yoimiya_default;
pub mod yunjin_default;
pub mod zhongli_default;

pub fn get_target_function(
    name: TargetFunctionName,
    character: &Character<SimpleAttributeGraph2>,
    weapon: &Weapon<SimpleAttributeGraph2>,
    config: &TargetFunctionConfig
) -> Box<dyn TargetFunction> {
    name.create(&character.common_data, &weapon.common_data, config)
}
