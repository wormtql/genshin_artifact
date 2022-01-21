use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const HAMAYUMI_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK120,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct HamayumiEffect {
    rate: f64
}

impl HamayumiEffect {
    pub fn new(config: &WeaponConfig) -> HamayumiEffect {
        match *config {
            WeaponConfig::Hamayumi { rate } => HamayumiEffect {
                rate
            },
            _ => HamayumiEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for HamayumiEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus_a = (refine * 0.04 + 0.12) * (1.0 + self.rate);
        let bonus_b = bonus_a * 0.75;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "破魔之弓被动", bonus_a);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "破魔之弓被动", bonus_b);
    }
}