use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const ELEGY_OF_THE_END_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::Recharge120,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct ElegyOfTheEndEffect {
    rate: f64
}

impl ElegyOfTheEndEffect {
    pub fn new(config: &WeaponConfig) -> ElegyOfTheEndEffect {
        match *config {
            WeaponConfig::ElegyOfTheEnd { rate } => ElegyOfTheEndEffect {
                rate
            },
            _ => ElegyOfTheEndEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ElegyOfTheEndEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let em_bonus = refine * 15.0 + 45.0 + (refine * 25.0 + 75.0) * self.rate;
        attribute.set_value_by(AttributeName::ElementalMastery, "终末嗟叹之诗被动等效", em_bonus);
        let atk_bonus = (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("终末嗟叹之诗被动等效", atk_bonus);
    }
}