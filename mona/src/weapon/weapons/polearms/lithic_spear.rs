use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const LITHIC_SPEAR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::ATK60,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct LithicSpearEffect {
    liyue_count: usize
}

impl LithicSpearEffect {
    pub fn new(config: &WeaponConfig) -> LithicSpearEffect {
        match *config {
            WeaponConfig::LithicSpear { liyue_count } => LithicSpearEffect {
                liyue_count
            },
            _ => LithicSpearEffect {
                liyue_count: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LithicSpearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("千岩长枪被动", (refine * 0.01 + 0.06) * self.liyue_count as f64);
        attribute.set_value_by(AttributeName::CriticalBase, "千岩长枪被动", (refine * 0.01 + 0.02) * self.liyue_count as f64);
    }
}