use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const WIND_AND_SONG_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::Recharge67,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct WindAndSongEffect {
    rate: f64
}

impl WindAndSongEffect {
    pub fn new(config: &WeaponConfig) -> WindAndSongEffect {
        match *config {
            WeaponConfig::WindAndSong { rate } => WindAndSongEffect {
                rate
            },
            _ => WindAndSongEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WindAndSongEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("暗巷的酒与诗被动等效", value);
    }
}