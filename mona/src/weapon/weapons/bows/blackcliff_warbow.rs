use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const BLACKCLIFF_WARBOW_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage80,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct BlackcliffWarbowEffect {
    stack: f64
}

impl BlackcliffWarbowEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffWarbowEffect {
        match *config {
            WeaponConfig::BlackcliffWarbow { stack } => BlackcliffWarbowEffect {
                stack
            },
            _ => BlackcliffWarbowEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffWarbowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let atk_bonus = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("黑岩战弓被动等效", atk_bonus);
    }
}