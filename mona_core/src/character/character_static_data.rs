use crate::character::CharacterName;
use crate::common::{Element, WeaponType};
use super::character_sub_stat::CharacterSubStatFamily;

pub struct CharacterStaticData {
    pub name: CharacterName,
    pub chs: &'static str,
    pub element: Element,
    pub hp: [i32; 14],
    pub atk: [i32; 14],
    pub def: [i32; 14],
    pub sub_stat: CharacterSubStatFamily,
    pub weapon_type: WeaponType,
    pub star: i32,

    pub skill_name1: &'static str,
    pub skill_name2: &'static str,
    pub skill_name3: &'static str,
}
