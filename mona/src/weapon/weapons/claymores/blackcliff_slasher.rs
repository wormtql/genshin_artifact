use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const BLACKCLIFF_SLASHER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage120,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct BlackcliffSlasherEffect {
    stack: f64
}

impl BlackcliffSlasherEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffSlasherEffect {
        match *config {
            WeaponConfig::BlackcliffSlasher { stack } => BlackcliffSlasherEffect {
                stack
            },
            _ => BlackcliffSlasherEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffSlasherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("黑岩斩刀被动等效", value);
    }
}