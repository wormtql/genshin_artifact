use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const STAFF_OF_HOMA_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage144,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct StaffOfHomaEffect {
    be50_rate: f64
}

impl StaffOfHomaEffect {
    pub fn new(config: &WeaponConfig) -> StaffOfHomaEffect {
        match *config {
            WeaponConfig::StaffOfHoma { be50_rate } => StaffOfHomaEffect {
                be50_rate
            },
            _ => StaffOfHomaEffect {
                be50_rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for StaffOfHomaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let hp_bonus = refine * 0.05 + 0.15;
        attribute.add_hp_percentage("护摩之杖被动", hp_bonus);
        let atk_bonus_ratio = refine * 0.002 + 0.006 + (refine * 0.002 + 0.008) * self.be50_rate;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * atk_bonus_ratio),
            Box::new(move |grad, _x1, _x2| (grad * atk_bonus_ratio, 0.0)),
            "护摩之杖被动等效"
        );
    }
}