use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const SUMMIT_SHAPER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::ATK108,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct SummitShaperEffect {
    stack: f64,
    shield_rate: f64,
}

impl SummitShaperEffect {
    pub fn new(config: &WeaponConfig) -> SummitShaperEffect {
        match *config {
            WeaponConfig::SummitShaper { stack, shield_rate } => SummitShaperEffect {
                stack, shield_rate
            },
            _ => SummitShaperEffect {
                stack: 0.0,
                shield_rate: 0.0
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SummitShaperEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        attribute.set_value_by(AttributeName::ShieldStrength, "斫峰之刃被动", refine as f64 * 0.05 + 0.15);

        let atk_bonus = (refine as f64 * 0.01 + 0.03) * (1.0 + self.shield_rate);
        attribute.add_atk_percentage("斫峰之刃被动等效", atk_bonus);
    }
}