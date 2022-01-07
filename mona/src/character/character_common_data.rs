use super::character_static_data::{CharacterStaticData};
use super::character_name::CharacterName;
use super::character_sub_stat::{CharacterSubStat, CharacterSubStatFamily};
use crate::attribute::{AttributeName, AttributeGraph};
use crate::common::ChangeAttribute;
use super::characters::get_static_data;


pub struct CharacterCommonData {
    pub name: CharacterName,
    pub level: i32,
    pub ascend: bool,
    pub constellation: i32,
    pub base_atk: f64,
    pub base_def: f64,
    pub base_hp: f64,
    pub has_talent1: bool,
    pub has_talent2: bool,

    pub static_data: CharacterStaticData,
}

impl CharacterCommonData {
    pub fn new(name: CharacterName, level: i32, ascend: bool, constellation: i32) -> CharacterCommonData {
        let data = get_static_data(name);

        let base_atk = CharacterCommonData::base_value(&data.atk, level, ascend);
        let base_def = CharacterCommonData::base_value(&data.def, level, ascend);
        let base_hp = CharacterCommonData::base_value(&data.hp, level, ascend);

        CharacterCommonData {
            name,
            level, ascend, constellation,
            base_atk, base_def, base_hp,
            static_data: data,
            has_talent1: (level == 20 && ascend) || level > 20,
            has_talent2: (level == 60 && ascend) || level > 60,
        }
    }

    fn base_value(array: &[i32; 14], level: i32, ascend: bool) -> f64 {
        if level < 20 || (level == 20 && !ascend) {
            (level - 1) as f64 * (array[1] - array[0]) as f64 / 19.0 + array[0] as f64
        } else if level < 40 || (level == 40 && !ascend) {
            (level - 20) as f64 * (array[3] - array[2]) as f64 / 20.0 + array[2] as f64
        } else if level < 50 || (level == 50 && !ascend) {
            (level - 40) as f64 * (array[5] - array[4]) as f64 / 10.0 + array[4] as f64
        } else if level < 60 || (level == 60 && !ascend) {
            (level - 50) as f64 * (array[7] - array[6]) as f64 / 10.0 + array[6] as f64
        } else if level < 70 || (level == 70 && !ascend) {
            (level - 60) as f64 * (array[9] - array[8]) as f64 / 10.0 + array[8] as f64
        } else if level < 80 || (level == 80 && !ascend) {
            (level - 70) as f64 * (array[11] - array[10]) as f64 / 10.0 + array[10] as f64
        } else {
            (level - 80) as f64 * (array[13] - array[12]) as f64 / 10.0 + array[12] as f64
        }
    }
}

impl ChangeAttribute for CharacterCommonData {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::ATKBase, "角色基础攻击", self.base_atk);
        attribute.add_value(AttributeName::DEFBase, "角色基础防御", self.base_def);
        attribute.add_value(AttributeName::HPBase, "角色基础生命", self.base_hp);

        let sub_stat = CharacterSubStat::new(self.static_data.sub_stat, self.level, self.ascend);
        sub_stat.change_attribute(attribute);
    }
}