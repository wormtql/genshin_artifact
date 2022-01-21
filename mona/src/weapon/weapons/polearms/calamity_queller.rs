use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const CALAMITY_QUELLER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::ATK36,
    weapon_base: WeaponBaseATKFamily::ATK741,
    star: 5
};

pub struct CalamityQuellerEffect {
    stack: f64,
    backend_rate: f64
}

impl CalamityQuellerEffect {
    pub fn new(config: &WeaponConfig) -> CalamityQuellerEffect {
        match *config {
            WeaponConfig::CalamityQueller { stack, backend_rate } => CalamityQuellerEffect {
                stack,
                backend_rate
            },
            _ => CalamityQuellerEffect {
                stack: 0.0,
                backend_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for CalamityQuellerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let elemental_bonus = refine * 0.03 + 0.09;
        attribute.add_elemental_bonus("息灾被动", elemental_bonus);

        let atk_bonus = (refine * 0.008 + 0.024) * self.stack * (1.0 + self.backend_rate);
        attribute.add_atk_percentage("息灾被动等效", atk_bonus);
    }
}
