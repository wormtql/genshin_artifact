use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const PROTOTYPE_RANCOUR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::PhysicalBonus75,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct PrototypeRancourEffect {
    stack: f64
}

impl PrototypeRancourEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeRancourEffect {
        match *config {
            WeaponConfig::PrototypeRancour { stack } => PrototypeRancourEffect {
                stack,
            },
            _ => PrototypeRancourEffect {
                stack: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeRancourEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.01 + 0.03) * self.stack;
        attribute.add_atk_percentage("试作斩岩被动等效", value);
        attribute.add_def_percentage("试作斩岩被动等效", value);
    }
}