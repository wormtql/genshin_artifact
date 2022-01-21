use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const THE_BELL_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::HP90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct TheBellEffect {
    rate: f64
}

impl TheBellEffect {
    pub fn new(config: &WeaponConfig) -> TheBellEffect {
        match *config {
            WeaponConfig::TheBell { rate } => TheBellEffect {
                rate
            },
            _ => TheBellEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheBellEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "钟剑被动等效", value);
    }
}