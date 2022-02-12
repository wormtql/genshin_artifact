use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct RainslasherEffect {
    rate: f64
}

impl RainslasherEffect {
    pub fn new(config: &WeaponConfig) -> RainslasherEffect {
        match *config {
            WeaponConfig::Rainslasher { rate } => RainslasherEffect {
                rate
            },
            _ => RainslasherEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for RainslasherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "雨裁被动等效", value);
    }
}

pub struct Rainslasher;

impl WeaponTrait for Rainslasher {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Rainslasher,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: WeaponSubStatFamily::EM36,
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        effect: Some("止水息雷：对处于水元素或雷元素影响下的敌人，造成的伤害提高20％/24%/28%/32%/36%。"),
        chs: "雨裁"
    };

    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "被动应用比例",
            config: ItemConfigType::Float { min: 0.0, max: 0.0, default: 0.0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(RainslasherEffect::new(config)))
    }
}
