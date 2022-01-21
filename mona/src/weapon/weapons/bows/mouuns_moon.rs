use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const MOUUNS_MOON_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::ATK60,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct MouunsMoonEffect {
    energy: usize
}

impl MouunsMoonEffect {
    pub fn new(config: &WeaponConfig) -> MouunsMoonEffect {
        match *config {
            WeaponConfig::MouunsMoon { energy } => MouunsMoonEffect {
                energy
            },
            _ => MouunsMoonEffect {
                energy: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MouunsMoonEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value = ((refine * 0.0003 + 0.0009) * self.energy as f64).min(refine * 0.1 + 0.3);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "曚云之月被动", value);
    }
}