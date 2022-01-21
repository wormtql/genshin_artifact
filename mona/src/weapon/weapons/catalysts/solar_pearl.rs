use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const SOLAR_PEARL_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate60,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct SolarPearlEffect {
    rate1: f64,
    rate2: f64
}

impl SolarPearlEffect {
    pub fn new(config: &WeaponConfig) -> SolarPearlEffect {
        match *config {
            WeaponConfig::SolarPearl { rate1, rate2 } => SolarPearlEffect {
                rate1,
                rate2
            },
            _ => SolarPearlEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SolarPearlEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "匣里日月被动等效", value * self.rate1);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "匣里日月被动等效", value * self.rate1);
        attribute.set_value_by(AttributeName::BonusNormalAttack, "匣里日月被动等效", value * self.rate2);
    }
}