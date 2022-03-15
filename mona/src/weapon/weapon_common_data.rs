use super::weapon_static_data::{WeaponStaticData};
use super::weapon_name::WeaponName;
use crate::common::ChangeAttribute;
use crate::attribute::{Attribute, AttributeName};
use crate::weapon::weapons::get_static_data;
use super::weapon_sub_stat::{WeaponSubStat};

pub struct WeaponCommonData {
    pub level: i32,
    pub ascend: bool,
    pub refine: i32,
    pub base_atk: f64,
    pub name: WeaponName,

    pub static_data: WeaponStaticData,
}

impl WeaponCommonData {
    pub fn new(name: WeaponName, level: i32, ascend: bool, refine: i32) -> WeaponCommonData {
        let static_data = get_static_data(name);
        let base_atk = static_data.weapon_base.get_base_atk(level, ascend);

        WeaponCommonData {
            level, ascend, refine, base_atk,
            static_data,
            name,
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for WeaponCommonData {
    fn change_attribute(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ATKBase, "武器基础攻击", self.base_atk);

        if let Some(s) = self.static_data.weapon_sub_stat {
            let sub_stat = WeaponSubStat::new(s, self.level, self.ascend);
            sub_stat.change_attribute(attribute);
        }
    }
}