use crate::common::{Element, WeaponType};
use super::character_sub_stat::CharacterSubStatFamily;

pub struct CharacterStaticData {
    pub element: Element,
    pub hp: [i32; 14],
    pub atk: [i32; 14],
    pub def: [i32; 14],
    pub sub_stat: CharacterSubStatFamily,
    pub weapon_type: WeaponType,
    pub star: i32,
}
