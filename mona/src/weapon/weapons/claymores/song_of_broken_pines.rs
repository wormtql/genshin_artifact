use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const SONG_OF_BROKEN_PINES_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::PhysicalBonus45,
    weapon_base: WeaponBaseATKFamily::ATK741,
    star: 5
};

pub struct SongOfBrokenPinesEffect {
    rate: f64,
}

impl SongOfBrokenPinesEffect {
    pub fn new(config: &WeaponConfig) -> SongOfBrokenPinesEffect {
        match *config {
            WeaponConfig::SongOfBrokenPines { rate } => SongOfBrokenPinesEffect {
                rate,
            },
            _ => SongOfBrokenPinesEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SongOfBrokenPinesEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value = refine * 0.04 + 0.12 + (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("松籁响起之时被动等效", value);
    }
}