use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const PROTOTYPE_CRESCENT_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct PrototypeCrescentEffect {
    rate: f64
}

impl PrototypeCrescentEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeCrescentEffect {
        match *config {
            WeaponConfig::PrototypeCrescent { rate } => PrototypeCrescentEffect {
                rate
            },
            _ => PrototypeCrescentEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeCrescentEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.09 + 0.27) * self.rate;
        attribute.add_atk_percentage("试作澹月被动等效", value);
    }
}