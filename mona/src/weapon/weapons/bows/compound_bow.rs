use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const COMPOUND_BOW_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::PhysicalBonus150,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};

pub struct CompoundBowEffect {
    stack: f64
}

impl CompoundBowEffect {
    pub fn new(config: &WeaponConfig) -> CompoundBowEffect {
        match *config {
            WeaponConfig::CompoundBow { stack } => CompoundBowEffect {
                stack
            },
            _ => CompoundBowEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for CompoundBowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = (refine * 0.01 + 0.03) * self.stack;
        let speed_bonus = (refine * 0.003 + 0.009) * self.stack;
        attribute.add_atk_percentage("钢轮弓被动等效", atk_bonus);
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "钢轮弓被动等效", speed_bonus);
    }
}