use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const AMOS_BOW_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK108,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct AmosBowEffect {
    stack: f64
}

impl AmosBowEffect {
    pub fn new(config: &WeaponConfig) -> AmosBowEffect {
        match *config {
            WeaponConfig::AmosBow { stack } => AmosBowEffect {
                stack
            },
            _ => AmosBowEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for AmosBowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus = refine * 0.03 + 0.09 + (refine * 0.02 + 0.06) * self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "阿莫斯之弓", bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "阿莫斯之弓", bonus);
    }
}