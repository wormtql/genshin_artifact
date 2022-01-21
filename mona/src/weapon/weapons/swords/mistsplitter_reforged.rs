use crate::attribute::{AttributeName, Attribute};
use crate::common::{Element, WeaponType};
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::Character;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use super::super::super::weapon_config::WeaponConfig;


pub const MISTSPLITTER_REFORGED_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage96,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5,
};

pub struct MistsplitterReforgedEffect {
    pub level: i32,
    pub element: Element,
}

impl<T: Attribute> WeaponEffect<T> for MistsplitterReforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value1 = data.refine as f64 * 0.03 + 0.09;
        let key = "雾切之回光被动";
        attribute.set_value_by(AttributeName::BonusElectro, key, value1);
        attribute.set_value_by(AttributeName::BonusHydro, key, value1);
        attribute.set_value_by(AttributeName::BonusAnemo, key, value1);
        attribute.set_value_by(AttributeName::BonusPyro, key, value1);
        attribute.set_value_by(AttributeName::BonusCryo, key, value1);
        attribute.set_value_by(AttributeName::BonusDendro, key, value1);
        attribute.set_value_by(AttributeName::BonusGeo, key, value1);

        let value2 = if self.level == 1 {
            0.02 * data.refine as f64 + 0.06
        } else if self.level == 2 {
            0.04 * data.refine as f64 + 0.12
        } else if self.level == 3 {
            0.07 * data.refine as f64 + 0.21
        } else {
            0.0
        };

        let attribute_name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(attribute_name, key, value2);
    }
}

impl MistsplitterReforgedEffect {
    pub fn new<T>(config: &WeaponConfig, character: &Character<T>) -> MistsplitterReforgedEffect {
        match config {
            WeaponConfig::MistsplitterReforged { emblem_level } => MistsplitterReforgedEffect {
                level: *emblem_level as i32,
                element: character.common_data.static_data.element,
            },
            _ => unreachable!(),
        }
    }
}
