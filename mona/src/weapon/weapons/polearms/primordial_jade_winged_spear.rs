use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const PRIMORDIAL_JADE_WINGED_SPEAR_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate48,
    weapon_base: WeaponBaseATKFamily::ATK674,
    star: 5
};

pub struct PrimordialJadeWingedSpearEffect {
    stack: f64,
    full_rate: f64
}

impl PrimordialJadeWingedSpearEffect {
    pub fn new(config: &WeaponConfig) -> PrimordialJadeWingedSpearEffect {
        match *config {
            WeaponConfig::PrimordialJadeWingedSpear { stack, full_rate } => PrimordialJadeWingedSpearEffect {
                stack,
                full_rate
            },
            _ => PrimordialJadeWingedSpearEffect {
                stack: 0.0,
                full_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrimordialJadeWingedSpearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = (refine * 0.007 + 0.025) * self.stack;
        attribute.add_atk_percentage("和璞鸢被动等效", atk_bonus);
        let bonus = (refine * 0.03 + 0.09) * self.full_rate;
        attribute.set_value_by(AttributeName::BonusBase, "和璞鸢被动等效", bonus);
    }
}