use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;
use crate::attribute::{Attribute};

pub use albedo::{Albedo};
pub use aloy::{Aloy};
pub use amber::{Amber};
pub use arataki_itto::{AratakiItto};
pub use barbara::{Barbara};
pub use beidou::{Beidou};
pub use bennett::{Bennett};
pub use chongyun::{Chongyun};
pub use diluc::{Diluc};
pub use diona::{Diona};
pub use eula::{Eula};
pub use fischl::{Fischl};
pub use ganyu::{Ganyu};
pub use gorou::{Gorou};
pub use hu_tao::{HuTao};
pub use jean::{Jean};
pub use kaedehara_kazuha::{KaedeharaKazuha};
pub use kaeya::{Kaeya};
pub use kamisato_ayaka::{KamisatoAyaka};
pub use keqing::{Keqing};
pub use klee::{Klee};
pub use kujou_sara::{KujouSara};
pub use lisa::{Lisa};
pub use mona::{Mona};
pub use ningguang::{Ningguang};
pub use noelle::{Noelle};
pub use qiqi::{Qiqi};
pub use raiden_shogun::{RaidenShogun};
pub use razor::{Razor};
pub use rosaria::{Rosaria};
pub use sangonomiya_kokomi::{SangonomiyaKokomi};
pub use sayu::{Sayu};
pub use shenhe::{Shenhe};
pub use sucrose::{Sucrose};
pub use tartaglia::{Tartaglia};
pub use thoma::{Thoma};
pub use venti::{Venti};
pub use xiangling::{Xiangling};
pub use xiao::{Xiao};
pub use xingqiu::{Xingqiu};
pub use xinyan::{Xinyan};
pub use yae_miko::YaeMiko;
pub use yanfei::{Yanfei};
pub use yoimiya::{Yoimiya};
pub use yunjin::{Yunjin};
pub use zhongli::{Zhongli};
pub use kamisato_ayato::KamisatoAyato;

use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub mod albedo;
pub mod aloy;
pub mod amber;
pub mod arataki_itto;
pub mod barbara;
pub mod beidou;
pub mod bennett;
pub mod chongyun;
pub mod diluc;
pub mod diona;
pub mod eula;
pub mod fischl;
pub mod ganyu;
pub mod gorou;
pub mod hu_tao;
pub mod jean;
pub mod kaedehara_kazuha;
pub mod kaeya;
pub mod kamisato_ayaka;
pub mod keqing;
pub mod klee;
pub mod kujou_sara;
pub mod lisa;
pub mod mona;
pub mod ningguang;
pub mod noelle;
pub mod qiqi;
pub mod raiden_shogun;
pub mod razor;
pub mod rosaria;
pub mod sangonomiya_kokomi;
pub mod sayu;
pub mod shenhe;
pub mod sucrose;
pub mod tartaglia;
pub mod thoma;
pub mod venti;
pub mod xiangling;
pub mod xiao;
pub mod xingqiu;
pub mod xinyan;
pub mod yae_miko;
pub mod yanfei;
pub mod yoimiya;
pub mod yunjin;
pub mod zhongli;
pub mod kamisato_ayato;

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

pub fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, skill_index: usize, skill_config: &CharacterSkillConfig) -> D::Result {
    CharacterName::damage::<D>(context, skill_index, skill_config)
}

pub fn get_target_function_by_role(
    role_index: usize,
    team: &TeamQuantization,
    character: &CharacterCommonData,
    weapon: &WeaponCommonData
) -> Box<dyn TargetFunction> {
    CharacterName::get_target_function_by_role(role_index, team, character, weapon)
}
