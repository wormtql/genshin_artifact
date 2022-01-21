use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const PROTOTYPE_STARGLITTER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::Recharge100,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct PrototypeStarglitterEffect {
    stack: f64
}

impl PrototypeStarglitterEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeStarglitterEffect {
        match *config {
            WeaponConfig::PrototypeStarglitter { stack } => PrototypeStarglitterEffect {
                stack
            },
            _ => PrototypeStarglitterEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeStarglitterEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.02 + 0.06) * self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "试作星镰", value);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "试作星镰", value);
    }
}