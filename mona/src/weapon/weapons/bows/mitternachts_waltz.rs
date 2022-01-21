use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const MITTERNACHTS_WALTZ_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::PhysicalBonus113,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct MitternachtsWaltzEffect {
    rate1: f64,
    rate2: f64
}

impl MitternachtsWaltzEffect {
    pub fn new(config: &WeaponConfig) -> MitternachtsWaltzEffect {
        match *config {
            WeaponConfig::MitternachtsWaltz { rate1, rate2 } => MitternachtsWaltzEffect {
                rate1,
                rate2
            },
            _ => MitternachtsWaltzEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MitternachtsWaltzEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "幽夜华尔兹被动等效", value * self.rate1);
        attribute.set_value_by(AttributeName::BonusNormalAttack, "幽夜华尔兹被动等效", value * self.rate2);
    }
}
