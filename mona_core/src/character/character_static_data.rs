use crate::character::CharacterName;
use crate::common::{Element, WeaponType};
use crate::common::i18n::I18nLocale;
use super::character_sub_stat::CharacterSubStatFamily;

pub struct CharacterStaticData {
    pub name: CharacterName,
    pub internal_name: &'static str,
    pub name_locale: I18nLocale,
    pub element: Element,
    pub hp: [i32; 15],
    pub atk: [i32; 15],
    pub def: [i32; 15],
    pub sub_stat: CharacterSubStatFamily,
    pub weapon_type: WeaponType,
    pub star: i32,

    pub skill_name1: I18nLocale,
    pub skill_name2: I18nLocale,
    pub skill_name3: I18nLocale,
}
