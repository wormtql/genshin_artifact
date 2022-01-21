use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const POLAR_STAR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Bow,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate72,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct PolarStarEffect {
    stack: usize
}

impl PolarStarEffect {
    pub fn new(config: &WeaponConfig) -> PolarStarEffect {
        match *config {
            WeaponConfig::PolarStar { stack } => PolarStarEffect {
                stack
            },
            _ => PolarStarEffect {
                stack: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PolarStarEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "冬极白星被动", bonus1);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "冬极白星被动", bonus1);

        let atk_bonus = if self.stack == 1 {
            refine * 0.025 + 0.075
        } else if self.stack == 2 {
            refine * 0.05 + 0.15
        } else if self.stack == 3 {
            refine * 0.075 + 0.225
        } else if self.stack == 4 {
            refine * 0.12 + 0.36
        } else {
            0.0
        };

        attribute.add_atk_percentage("冬极白星被动等效", atk_bonus);
    }
}