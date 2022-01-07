use super::weapon_data::{WeaponData};
use super::weapon_name::WeaponName;
use crate::common::ChangeAttribute;
use crate::attribute::{AttributeGraph, AttributeName};
use super::weapon_sub_stat::{WeaponSubStat};
use crate::utils;

pub struct WeaponCommonData {
    pub level: i32,
    pub ascend: bool,
    pub refine: i32,
    pub base_atk: f64,
    pub name: WeaponName,

    pub static_data: WeaponData,
}

impl WeaponCommonData {
    pub fn new(name: WeaponName, level: i32, ascend: bool, refine: i32) -> WeaponCommonData {
        let static_data = WeaponData::weapon_data(name);
        let base_atk = static_data.weapon_base.get_base_atk(level, ascend);

        WeaponCommonData {
            level, ascend, refine, base_atk,
            static_data,
            name,
        }
    }
}

impl ChangeAttribute for WeaponCommonData {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::ATKBase, "武器基础攻击", self.base_atk);

        let sub_stat = WeaponSubStat::new(self.static_data.weapon_sub_stat, self.level, self.ascend);
        sub_stat.change_attribute(attribute);
    }
}