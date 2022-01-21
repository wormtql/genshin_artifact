use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const AKUOUMARU_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::ATK90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct AkuoumaruEffect {
    energy: usize
}

impl AkuoumaruEffect {
    pub fn new(config: &WeaponConfig) -> AkuoumaruEffect {
        match *config {
            WeaponConfig::Akuoumaru { energy } => AkuoumaruEffect {
                energy
            },
            _ => AkuoumaruEffect {
                energy: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for AkuoumaruEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value = (self.energy as f64 * (refine * 0.0003 + 0.0009)).min(refine * 0.1 + 0.3);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "恶王丸被动", value);
    }
}