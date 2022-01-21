use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const LOST_PRAYER_TO_THE_SACRED_WINDS_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate72,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct LostPrayerToTheSacredWindsEffect {
    stack: f64
}

impl LostPrayerToTheSacredWindsEffect {
    pub fn new(config: &WeaponConfig) -> LostPrayerToTheSacredWindsEffect {
        match *config {
            WeaponConfig::LostPrayerToTheSacredWinds { stack } => LostPrayerToTheSacredWindsEffect {
                stack
            },
            _ => LostPrayerToTheSacredWindsEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LostPrayerToTheSacredWindsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.02 + 0.06) * self.stack;
        attribute.add_elemental_bonus("四风原典被动等效", value);
    }
}