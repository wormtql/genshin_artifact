use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const THE_UNFORGED_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::ATK108,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct TheUnforgedEffect {
    stack: f64,
    shield_rate: f64,
}

impl TheUnforgedEffect {
    pub fn new(config: &WeaponConfig) -> TheUnforgedEffect {
        match *config {
            WeaponConfig::TheUnforged { shield_rate, stack } => TheUnforgedEffect {
                stack, shield_rate
            },
            _ => TheUnforgedEffect {
                stack: 0.0,
                shield_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheUnforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::ShieldStrength, "无工之剑被动", refine * 0.05 + 0.15);

        let atk_bonus = (refine * 0.01 + 0.03) * self.stack * (1.0 + self.shield_rate);
        attribute.add_atk_percentage("无工之剑被动等效", atk_bonus);
    }
}