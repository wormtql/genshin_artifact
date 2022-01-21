use crate::attribute::{Attribute, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const IRON_STING_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::EM36,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct IronStingEffect {
    pub stack: f64,
}

impl IronStingEffect {
    pub fn new(config: &WeaponConfig) -> IronStingEffect {
        match *config {
            WeaponConfig::IronSting { stack } => IronStingEffect {
                stack,
            },
            _ => IronStingEffect {
                stack: 0.0,
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for IronStingEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.015 + 0.045) * self.stack;
        attribute.set_value_by(AttributeName::BonusBase, "铁蜂刺被动等效", value);
    }
}