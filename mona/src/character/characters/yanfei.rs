use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, StatName, WeaponType};

pub struct YanfeiSkillType {
    // todo yanfei skill
}

pub const YANFEI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Pyro,
    hp: [784, 2014, 2160, 3895, 4311, 4959, 5514, 6161, 6578, 7225, 7641, 8289, 8705, 9352],
    atk: [20, 52, 67, 100, 111, 127, 141, 158, 169, 185, 196, 213, 223, 240],
    def: [49, 126, 163, 244, 271, 311, 346, 387, 413, 453, 480, 520, 546, 587],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::PyroBonus),
    weapon_type: WeaponType::Catalyst,
    star: 4
};