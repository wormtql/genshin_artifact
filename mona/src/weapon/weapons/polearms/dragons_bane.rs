use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const DRAGONS_BANE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::EM48,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct DragonsBaneEffect {
    rate: f64
}

impl DragonsBaneEffect {
    pub fn new(config: &WeaponConfig) -> DragonsBaneEffect {
        match *config {
            WeaponConfig::DragonsBane { rate } => DragonsBaneEffect {
                rate
            },
            _ => DragonsBaneEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DragonsBaneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "匣里灭辰被动等效", value);
    }
}