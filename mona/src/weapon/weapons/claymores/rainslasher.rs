use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const RAINSLASHER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::EM36,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct RainslasherEffect {
    rate: f64
}

impl RainslasherEffect {
    pub fn new(config: &WeaponConfig) -> RainslasherEffect {
        match *config {
            WeaponConfig::Rainslasher { rate } => RainslasherEffect {
                rate
            },
            _ => RainslasherEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for RainslasherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "雨裁被动等效", value);
    }
}