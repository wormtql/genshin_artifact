use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const MAPPA_MARE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Catalyst,
    weapon_sub_stat: WeaponSubStatFamily::EM24,
    weapon_base: WeaponBaseATKFamily::ATK565,
    star: 4
};

pub struct MappaMareEffect {
    stack: f64
}

impl MappaMareEffect {
    pub fn new(config: &WeaponConfig) -> MappaMareEffect {
        match *config {
            WeaponConfig::MappaMare { stack } => MappaMareEffect {
                stack
            },
            _ => MappaMareEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MappaMareEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.02 + 0.06) * self.stack;
        attribute.add_elemental_bonus("万国诸海图谱被动等效", value);
    }
}