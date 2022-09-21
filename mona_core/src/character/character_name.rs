// required by strum_derive::EnumString
use std::str::FromStr;

use mona_derive::{CharacterData, EnumLen};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::*;
use strum_macros::{Display, EnumString};

use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_static_data::CharacterStaticData;
use crate::character::CharacterConfig;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait};
use crate::common::ChangeAttribute;
use crate::common::element::Element;
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[derive(Display, FromPrimitive, EnumString, CharacterData, EnumLen)]
pub enum CharacterName {
    AetherAnemo,
    Albedo,
    Aloy,
    Amber,
    AratakiItto,
    Barbara,
    Beidou,
    Bennett,
    Candace,
    Chongyun,
    Collei,
    Cyno,
    Diluc,
    Diona,
    Dori,
    Eula,
    Fischl,
    Ganyu,
    Gorou,
    HuTao,
    Jean,
    KaedeharaKazuha,
    Kaeya,
    KamisatoAyaka,
    KamisatoAyato,
    Keqing,
    Klee,
    KujouSara,
    KukiShinobu,
    Lisa,
    Mona,
    Nilou,
    Ningguang,
    Noelle,
    Qiqi,
    RaidenShogun,
    Razor,
    Rosaria,
    SangonomiyaKokomi,
    Sayu,
    Shenhe,
    ShikanoinHeizou,
    Sucrose,
    Tartaglia,
    Thoma,
    Tighnari,
    // Traveler,
    Venti,
    Xiangling,
    Xiao,
    Xingqiu,
    Xinyan,
    YaeMiko,
    Yanfei,
    Yelan,
    Yoimiya,
    Yunjin,
    Zhongli,
}