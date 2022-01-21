use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const THE_WIDSITH_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage120,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct TheWidsithEffect {
    theme: usize
}

impl TheWidsithEffect {
    pub fn new(config: &WeaponConfig) -> TheWidsithEffect {
        match *config {
            WeaponConfig::TheWidsith { theme } => TheWidsithEffect {
                theme
            },
            _ => TheWidsithEffect {
                theme: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheWidsithEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        if self.theme == 0 {
            attribute.add_atk_percentage("流浪乐章被动", refine * 0.15 + 0.45);
        } else if self.theme == 1 {
            attribute.add_elemental_bonus("流浪乐章被动", refine * 0.12 + 0.36);
        } else if self.theme == 2 {
            attribute.set_value_by(AttributeName::ElementalMastery, "流浪乐章被动", refine * 60.0 + 180.0);
        }
    }
}
