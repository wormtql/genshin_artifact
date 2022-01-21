use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const THUNDERING_PULSE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage144,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct ThunderingPulseEffect {
    stack: usize
}

impl ThunderingPulseEffect {
    pub fn new(config: &WeaponConfig) -> ThunderingPulseEffect {
        match *config {
            WeaponConfig::ThunderingPulse { stack } => ThunderingPulseEffect {
                stack
            },
            _ => ThunderingPulseEffect {
                stack: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ThunderingPulseEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("飞雷之弦振被动", refine * 0.05 + 0.15);
        let bonus = if self.stack == 1 {
            refine * 0.03 + 0.09
        } else if self.stack == 2 {
            refine * 0.06 + 0.18
        } else if self.stack == 3 {
            refine * 0.1 + 0.3
        } else {
            0.0
        };
        attribute.set_value_by(AttributeName::BonusNormalAttack, "飞雷之弦振被动", bonus);
    }
}