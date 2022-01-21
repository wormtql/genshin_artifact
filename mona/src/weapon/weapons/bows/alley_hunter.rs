use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const ALLEY_HUNTER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK60,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct AlleyHunterEffect {
    stack: f64
}

impl AlleyHunterEffect {
    pub fn new(config: &WeaponConfig) -> AlleyHunterEffect {
        match *config {
            WeaponConfig::AlleyHunter { stack } => AlleyHunterEffect {
                stack
            },
            _ => AlleyHunterEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for AlleyHunterEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.005 + 0.015) * self.stack;
        attribute.set_value_by(AttributeName::BonusBase, "暗巷猎手被动等效", value);
    }
}