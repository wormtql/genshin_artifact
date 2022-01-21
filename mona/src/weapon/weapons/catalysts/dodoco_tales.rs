use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const DODOCO_TALES_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::ATK120,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct DodocoTalesEffect {
    rate1: f64,
    rate2: f64
}

impl DodocoTalesEffect {
    pub fn new(config: &WeaponConfig) -> DodocoTalesEffect {
        match *config {
            WeaponConfig::DodocoTales { rate1, rate2 } => DodocoTalesEffect {
                rate1,
                rate2
            },
            _ => DodocoTalesEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DodocoTalesEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::BonusChargedAttack, "嘟嘟可故事集被动等效", (refine * 0.04 + 0.12) * self.rate1);
        attribute.add_atk_percentage("嘟嘟可故事集被动等效", (refine * 0.02 + 0.06) * self.rate2);
    }
}