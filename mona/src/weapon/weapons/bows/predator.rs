use crate::attribute::{Attribute, AttributeName};
use crate::character::{Character, CharacterName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const PREDATOR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct PredatorEffect {
    stack: f64,
    is_aloy: bool
}

impl PredatorEffect {
    pub fn new<T: Attribute>(config: &WeaponConfig, character: &Character<T>) -> PredatorEffect {
        let is_aloy = character.common_data.name == CharacterName::Aloy;
        match *config {
            WeaponConfig::Predator { stack } => PredatorEffect {
                stack,
                is_aloy
            },
            _ => PredatorEffect {
                stack: 0.0,
                is_aloy
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PredatorEffect {
    fn apply(&self, _data: &WeaponCommonData, attribute: &mut T) {
        let value = 0.1 + self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "掠食者被动等效", value);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "掠食者被动等效", value);
        if self.is_aloy {
            attribute.set_value_by(AttributeName::ATKFixed, "掠食者被动", 66.0);
        }
    }
}