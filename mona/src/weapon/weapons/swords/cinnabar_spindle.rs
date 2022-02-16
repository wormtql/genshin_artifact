use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct CinnabarSpindleEffect {
    rate: f64,
}

impl CinnabarSpindleEffect {
    pub fn new(config: &WeaponConfig) -> CinnabarSpindleEffect {
        match *config {
            WeaponConfig::CinnabarSpindle { rate } => CinnabarSpindleEffect {
                rate,
            },
            _ => CinnabarSpindleEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for CinnabarSpindleEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.1 + 0.3) * self.rate;
        attribute.set_value_by(AttributeName::DEFRatioElementalSkill, "辰砂之纺锤被动等效", value);
    }
}

pub struct CinnabarSpindle;

impl WeaponTrait for CinnabarSpindle {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CinnabarSpindle,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: WeaponSubStatFamily::DEF150,
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("元素战技造成的伤害值提高，提高数值相当于防御力的40/50/60/70/80%。该效果每1.5秒最多触发一次，并将在元素战技造成伤害后的0.1秒后清除效果。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "辰砂之纺锤"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "被动应用比例",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(CinnabarSpindleEffect::new(config)))
    }
}
