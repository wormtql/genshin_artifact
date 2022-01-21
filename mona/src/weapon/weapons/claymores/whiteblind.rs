use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const WHITEBLIND_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::DEF113,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct WhiteblindEffect {
    stack: f64,
}

impl WhiteblindEffect {
    pub fn new(config: &WeaponConfig) -> WhiteblindEffect {
        match *config {
            WeaponConfig::Whiteblind { stack } => WhiteblindEffect {
                stack
            },
            _ => WhiteblindEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WhiteblindEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.015 + 0.045) * self.stack;
        attribute.add_atk_percentage("白影剑被动等效", value);
        attribute.add_def_percentage("白影剑被动等效", value);
    }
}