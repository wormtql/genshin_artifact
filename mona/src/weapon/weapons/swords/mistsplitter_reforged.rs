use crate::attribute::{AttributeName, AttributeGraph};
use crate::common::{ChangeAttribute, Element};
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::Character;
use super::super::super::weapon::Weapon;
use crate::weapon::weapon_name::WeaponName;
use super::super::super::weapon_config::WeaponConfig;

pub struct MistsplitterReforgedEffect {
    pub level: i32,
    pub element: Element,
}

impl WeaponEffect for MistsplitterReforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut AttributeGraph) {
        let value1 = data.refine as f64 * 0.03 + 0.09;
        let key = "雾切之回光被动";
        attribute.add_value(AttributeName::BonusElectro, key, value1);
        attribute.add_value(AttributeName::BonusHydro, key, value1);
        attribute.add_value(AttributeName::BonusAnemo, key, value1);
        attribute.add_value(AttributeName::BonusPyro, key, value1);
        attribute.add_value(AttributeName::BonusCryo, key, value1);
        attribute.add_value(AttributeName::BonusDendro, key, value1);
        attribute.add_value(AttributeName::BonusGeo, key, value1);

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
        attribute.add_value_delta(attribute_name, key, value2);
    }
}

impl MistsplitterReforgedEffect {
    pub fn new(config: &WeaponConfig, character: &Character) -> MistsplitterReforgedEffect {
        match config {
            WeaponConfig::MistsplitterReforgedConfig(x) => MistsplitterReforgedEffect {
                level: *x,
                element: character.common_data.static_data.element,
            },
            _ => unreachable!(),
        }
    }
}
