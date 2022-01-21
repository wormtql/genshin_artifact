use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const LIONS_ROAR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::ATK90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct LionsRoarEffect {
    rate: f64,
}

impl LionsRoarEffect {
    pub fn new(config: &WeaponConfig) -> LionsRoarEffect {
        match *config {
            WeaponConfig::LionsRoar { rate } => LionsRoarEffect {
                rate
            },
            _ => LionsRoarEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LionsRoarEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "匣里龙吟被动等效", value);
    }
}