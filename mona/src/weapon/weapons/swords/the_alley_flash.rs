use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const THE_ALLEY_FLASH_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::EM12,
    weapon_base: WeaponBaseATKFamily::ATK620,
    star: 4
};

pub struct TheAlleyFlashEffect {
    rate: f64,
}

impl TheAlleyFlashEffect {
    pub fn new(config: &WeaponConfig) -> TheAlleyFlashEffect {
        match *config {
            WeaponConfig::TheAlleyFlash { rate } => TheAlleyFlashEffect {
                rate,
            },
            _ => TheAlleyFlashEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheAlleyFlashEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusBase, "暗巷闪光被动", value * self.rate);
    }
}