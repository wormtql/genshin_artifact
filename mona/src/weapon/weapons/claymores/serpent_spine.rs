use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const SERPENT_SPINE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate60,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct  SerpentSpineEffect {
    stack: f64
}

impl SerpentSpineEffect {
    pub fn new(config: &WeaponConfig) -> SerpentSpineEffect {
        match *config {
            WeaponConfig::SerpentSpine { stack } => SerpentSpineEffect {
                stack
            },
            _ => SerpentSpineEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SerpentSpineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.01 + 0.05) * self.stack;
        attribute.set_value_by(AttributeName::BonusBase, "螭骨剑被动等效", value);
    }
}