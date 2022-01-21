use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const LITHIC_BLADE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::ATK90,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct LithicBladeEffect {
    liyue_count: usize
}

impl LithicBladeEffect {
    pub fn new(config: &WeaponConfig) -> LithicBladeEffect {
        match *config {
            WeaponConfig::LithicBlade { liyue_count } => LithicBladeEffect {
                liyue_count
            },
            _ => LithicBladeEffect {
                liyue_count: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LithicBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = (refine * 0.01 + 0.06) * self.liyue_count as f64;
        attribute.add_atk_percentage("千岩古剑被动", atk_bonus);
        let crit_bonus = (refine * 0.01 + 0.02) * self.liyue_count as f64;
        attribute.set_value_by(AttributeName::CriticalBase, "千岩古剑被动", crit_bonus);
    }
}