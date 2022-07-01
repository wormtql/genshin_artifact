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

pub struct LionsRoarEffect {
    rate: f64,
}

impl LionsRoarEffect {
    pub fn new(config: &WeaponConfig) -> LionsRoarEffect {
        match *config {
            WeaponConfig::LionsRoar { rate } => LionsRoarEffect {
                rate
            },
            _ => LionsRoarEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LionsRoarEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "匣里龙吟被动等效", value);
    }
}

pub struct LionsRoar;

impl WeaponTrait for LionsRoar {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LionsRoar,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("踏火息雷：对处于火元素或雷元素影响下的敌人，造成的伤害提高20%/24%/28%/32%/36%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "匣里龙吟"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "w2",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LionsRoarEffect::new(config)))
    }
}
