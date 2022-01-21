use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const DEATHMATCH_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate80,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct DeathmatchEffect {
    ge2: bool
}

impl DeathmatchEffect {
    pub fn new(config: &WeaponConfig) -> DeathmatchEffect {
        match *config {
            WeaponConfig::Deathmatch { ge2 } => DeathmatchEffect {
                ge2
            },
            _ => DeathmatchEffect {
                ge2: false
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DeathmatchEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        if self.ge2 {
            let value = refine * 0.04 + 0.12;
            attribute.add_atk_percentage("决斗之枪被动", value);
            attribute.add_def_percentage("决斗之枪被动", value);
        } else {
            let value = refine * 0.06 + 0.18;
            attribute.add_atk_percentage("决斗之枪被动", value);
        }
    }
}