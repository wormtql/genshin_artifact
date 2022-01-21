use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const FREEDOM_SWORN_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::EM43,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct FreedomSwornEffect {
    rate: f64,
}

impl FreedomSwornEffect {
    pub fn new(config: &WeaponConfig) -> FreedomSwornEffect {
        match *config {
            WeaponConfig::FreedomSworn { rate } => FreedomSwornEffect {
                rate,
            },
            _ => FreedomSwornEffect {
                rate: 0.0,
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for FreedomSwornEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        attribute.set_value_by(AttributeName::BonusBase, "苍古自由之誓被动", refine as f64 * 0.025 + 0.075);
        let dmg_bonus = (refine as f64 * 0.04 + 0.12) * self.rate;
        let atk_bonus = (refine as f64 * 0.05 + 0.15) * self.rate;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "苍古自由之誓被动等效", dmg_bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "苍古自由之誓被动等效", dmg_bonus);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "苍古自由之誓被动等效", dmg_bonus);
        attribute.add_atk_percentage("苍古自由之誓被动等效", atk_bonus);
    }
}