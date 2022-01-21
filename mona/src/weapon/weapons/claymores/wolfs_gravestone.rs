use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const WOLFS_GRAVESTONE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Claymore,
    weapon_sub_stat: WeaponSubStatFamily::ATK108,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

pub struct WolfsGravestoneEffect {
    rate: f64,
}

impl WolfsGravestoneEffect {
    pub fn new(config: &WeaponConfig) -> WolfsGravestoneEffect {
        match *config {
            WeaponConfig::WolfsGravestone { rate } => WolfsGravestoneEffect {
                rate
            },
            _ => WolfsGravestoneEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WolfsGravestoneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value1 = refine * 0.05 + 0.15;
        let value2 = (refine * 0.1 + 0.3) * self.rate;
        attribute.add_atk_percentage("狼的末路被动等效", value1 + value2);
    }
}