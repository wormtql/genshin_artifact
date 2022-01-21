use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const BLACKCLIFF_LONGSWORD_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate80,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct BlackcliffLongswordEffect {
    stack: f64,
}

impl BlackcliffLongswordEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffLongswordEffect {
        match *config {
            WeaponConfig::BlackcliffLongsword { stack } => BlackcliffLongswordEffect {
                stack
            },
            _ => BlackcliffLongswordEffect {
                stack: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffLongswordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("黑岩长剑被动等效", value);
    }
}