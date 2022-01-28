use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;
use crate::attribute::Attribute;
use super::traits::{CharacterTrait};

pub use albedo::{Albedo};
pub use aloy::{ALOY_STATIC_DATA, ALOY_SKILL};
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
pub use keqing::{KEQING_SKILL, KEQING_STATIC_DATA};
pub use klee::{KLEE_SKILL, KLEE_STATIC_DATA};
pub use kujou_sara::{KUJOU_SARA_SKILL, KUJOU_SARA_STATIC_DATA};
pub use lisa::{LISA_SKILL, LISA_STATIC_DATA};
pub use mona::{MonaEffect, MONA_STATIC_DATA, MONA_SKILL};
pub use ningguang::{NINGGUANG_SKILL, NINGGUANG_STATIC_DATA};
pub use noelle::{NOELLE_SKILL, NOELLE_STATIC_DATA};
pub use qiqi::{QIQI_SKILL, QIQI_STATIC_DATA};
pub use raiden_shogun::{RaidenShogunEffect, RAIDEN_SHOGUN_SKILL, RAIDEN_SHOGUN_STATIC_DATA};
pub use razor::{RAZOR_SKILL, RAZOR_STATIC_DATA};
pub use rosaria::{ROSARIA_SKILL, ROSARIA_STATIC_DATA};
pub use sangonomiya_kokomi::{SANGONOMIYA_KOKOMI_SKILL, SANGONOMIYA_KOKOMI_STATIC_DATA, SangonomiyaKokomiEffect};
pub use sayu::{SAYU_SKILL, SAYU_STATIC_DATA};
pub use sucrose::{SUCROSE_SKILL, SUCROSE_STATIC_DATA};
pub use tartaglia::{TARTAGLIA_SKILL, TARTAGLIA_STATIC_DATA};
pub use thoma::{THOMA_SKILL, THOMA_STATIC_DATA};
pub use venti::{VENTI_SKILL, VENTI_STATIC_DATA};
pub use xiangling::{XIANGLING_SKILL, XIANGLING_STATIC_DATA};
pub use xiao::{XIAO_SKILL, XIAO_STATIC_DATA};
pub use xingqiu::{XINGQIU_SKILL, XINGQIU_STATIC_DATA};
pub use xinyan::{XINYAN_SKILL, XINYAN_STATIC_DATA};
pub use yanfei::{YANFEI_STATIC_DATA};
pub use yoimiya::{YoimiyaEffect, YOIMIYA_SKILL, YOIMIYA_STATIC_DATA};
pub use zhongli::{ZhongliEffect, ZHONGLI_SKILL, ZHONGLI_STATIC_DATA};
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
pub mod sucrose;
pub mod tartaglia;
pub mod thoma;
pub mod venti;
pub mod xiangling;
pub mod xiao;
pub mod xingqiu;
pub mod xinyan;
pub mod yanfei;
pub mod yoimiya;
pub mod zhongli;

pub fn get_static_data(name: CharacterName) -> CharacterStaticData {
    match name {
        CharacterName::Albedo => Albedo::STATIC_DATA,
        CharacterName::Aloy => ALOY_STATIC_DATA,
        CharacterName::Amber => Amber::STATIC_DATA,
        CharacterName::AratakiItto => AratakiItto::STATIC_DATA,
        CharacterName::Barbara => Barbara::STATIC_DATA,
        CharacterName::Beidou => Beidou::STATIC_DATA,
        CharacterName::Bennett => Bennett::STATIC_DATA,
        CharacterName::Chongyun => Chongyun::STATIC_DATA,
        CharacterName::Diluc => Diluc::STATIC_DATA,
        CharacterName::Diona => Diona::STATIC_DATA,
        CharacterName::Eula => Eula::STATIC_DATA,
        CharacterName::Fischl => Fischl::STATIC_DATA,
        CharacterName::Ganyu => Ganyu::STATIC_DATA,
        CharacterName::Gorou => Gorou::STATIC_DATA,
        CharacterName::HuTao => HuTao::STATIC_DATA,
        CharacterName::Jean => Jean::STATIC_DATA,
        CharacterName::KaedeharaKazuha => KaedeharaKazuha::STATIC_DATA,
        CharacterName::Kaeya => Kaeya::STATIC_DATA,
        CharacterName::KamisatoAyaka => KamisatoAyaka::STATIC_DATA,
        CharacterName::Keqing => KEQING_STATIC_DATA,
        CharacterName::Klee => KLEE_STATIC_DATA,
        CharacterName::KujouSara => KUJOU_SARA_STATIC_DATA,
        CharacterName::Lisa => LISA_STATIC_DATA,
        CharacterName::Mona => MONA_STATIC_DATA,
        CharacterName::Ningguang => NINGGUANG_STATIC_DATA,
        CharacterName::Noelle => NOELLE_STATIC_DATA,
        CharacterName::Qiqi => QIQI_STATIC_DATA,
        CharacterName::RaidenShogun => RAIDEN_SHOGUN_STATIC_DATA,
        CharacterName::Razor => RAZOR_STATIC_DATA,
        CharacterName::Rosaria => ROSARIA_STATIC_DATA,
        CharacterName::SangonomiyaKokomi => SANGONOMIYA_KOKOMI_STATIC_DATA,
        CharacterName::Sayu => SAYU_STATIC_DATA,
        CharacterName::Sucrose => SUCROSE_STATIC_DATA,
        CharacterName::Tartaglia => TARTAGLIA_STATIC_DATA,
        CharacterName::Thoma => THOMA_STATIC_DATA,
        CharacterName::Venti => VENTI_STATIC_DATA,
        CharacterName::Xiangling => XIANGLING_STATIC_DATA,
        CharacterName::Xiao => XIAO_STATIC_DATA,
        CharacterName::Xingqiu => XINGQIU_STATIC_DATA,
        CharacterName::Xinyan => XINYAN_STATIC_DATA,
        CharacterName::Yanfei => YANFEI_STATIC_DATA,
        CharacterName::Yoimiya => YOIMIYA_STATIC_DATA,
        CharacterName::Zhongli => ZHONGLI_STATIC_DATA,
        _ => panic!("unimplemented character"),
    }
}

pub fn get_effect<T: Attribute>(
    name: CharacterName,
    common_data: &CharacterCommonData,
    config: &CharacterConfig
) -> Option<Box<dyn ChangeAttribute<T>>> {
    match name {
        CharacterName::Amber => Some(Amber::new_effect(common_data, config)),
        CharacterName::Ganyu => Some(Ganyu::new_effect(common_data, config)),
        CharacterName::HuTao => Some(HuTao::new_effect(common_data, config)),
        CharacterName::KamisatoAyaka => Some(KamisatoAyaka::new_effect(common_data, config)),
        CharacterName::Mona => Some(Box::new(MonaEffect::new(common_data))),
        CharacterName::RaidenShogun => Some(Box::new(RaidenShogunEffect::new(common_data))),
        CharacterName::SangonomiyaKokomi => Some(Box::new(SangonomiyaKokomiEffect::new())),
        CharacterName::Yoimiya => Some(Box::new(YoimiyaEffect::new(common_data, config))),
        CharacterName::Zhongli => Some(Box::new(ZhongliEffect::new(common_data))),
        _ => None,
    }
}

pub fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, skill_index: usize, skill_config: &CharacterSkillConfig) -> D::Result {
    match context.character_common_data.name {
        CharacterName::Albedo => Albedo::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Amber => Amber::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::AratakiItto => AratakiItto::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Barbara => Barbara::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Beidou => Beidou::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Bennett => Bennett::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Chongyun => Chongyun::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Diluc => Diluc::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Diona => Diona::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Eula => Eula::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Fischl => Fischl::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Ganyu => Ganyu::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Gorou => Gorou::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::HuTao => HuTao::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Jean => Jean::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::KaedeharaKazuha => KaedeharaKazuha::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::Kaeya => Kaeya::damage_internal::<D>(context, skill_index, skill_config),
        CharacterName::KamisatoAyaka => KamisatoAyaka::damage_internal::<D>(context, skill_index, skill_config),
        _ => Amber::damage_internal::<D>(context, skill_index, skill_config)
    }
}

pub fn get_target_function_by_role(
    role_index: usize,
    team: &TeamQuantization,
    character: &CharacterCommonData,
    weapon: &WeaponCommonData
) -> Box<dyn TargetFunction> {
    match character.name {
        CharacterName::Albedo => Albedo::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Amber => Amber::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::AratakiItto => AratakiItto::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Barbara => Barbara::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Beidou => Beidou::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Bennett => Bennett::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Chongyun => Chongyun::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Diluc => Diluc::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Diona => Diona::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Eula => Eula::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Fischl => Fischl::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Ganyu => Ganyu::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Gorou => Gorou::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::HuTao => HuTao::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Jean => Jean::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::KaedeharaKazuha => KaedeharaKazuha::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::Kaeya => Kaeya::get_target_function_by_role(role_index, team, character, weapon),
        CharacterName::KamisatoAyaka => KamisatoAyaka::get_target_function_by_role(role_index, team, character, weapon),
        _ => panic!("no character")
    }
}
