use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const CINNABAR_SPINDLE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::DEF150,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct CinnabarSpindleEffect {
    rate: f64,
}

impl CinnabarSpindleEffect {
    pub fn new(config: &WeaponConfig) -> CinnabarSpindleEffect {
        match *config {
            WeaponConfig::CinnabarSpindle { rate } => CinnabarSpindleEffect {
                rate,
            },
            _ => CinnabarSpindleEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for CinnabarSpindleEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.1 + 0.3) * self.rate;
        attribute.set_value_by(AttributeName::DEFRatioElementalSkill, "辰砂之纺锤被动等效", value);
    }
}