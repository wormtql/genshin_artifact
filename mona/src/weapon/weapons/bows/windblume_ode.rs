use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const WINDBLUME_ODE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::EM36,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct WindblumeOdeEffect {
    rate: f64
}

impl WindblumeOdeEffect {
    pub fn new(config: &WeaponConfig) -> WindblumeOdeEffect {
        match *config {
            WeaponConfig::WindblumeOde { rate } => WindblumeOdeEffect {
                rate
            },
            _ => WindblumeOdeEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WindblumeOdeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let atk_bonus = (data.refine as f64 * 0.04 + 0.12) * self.rate;
        attribute.add_atk_percentage("风花之颂被动等效", atk_bonus);
    }
}
