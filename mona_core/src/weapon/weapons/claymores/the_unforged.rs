use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct TheUnforgedEffect {
    stack: f64,
    shield_rate: f64,
}

impl TheUnforgedEffect {
    pub fn new(config: &WeaponConfig) -> TheUnforgedEffect {
        match *config {
            WeaponConfig::TheUnforged { shield_rate, stack } => TheUnforgedEffect {
                stack, shield_rate
            },
            _ => TheUnforgedEffect {
                stack: 0.0,
                shield_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheUnforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::ShieldStrength, "无工之剑被动", refine * 0.05 + 0.15);

        let atk_bonus = (refine * 0.01 + 0.03) * self.stack * (1.0 + self.shield_rate);
        attribute.add_atk_percentage("无工之剑被动等效", atk_bonus);
    }
}

pub struct TheUnforged;

impl WeaponTrait for TheUnforged {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheUnforged,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("金璋皇极：护盾强效提升20%/25%/30%/35%/40%。攻击命中后的8秒内，攻击力提升4%/5%/6%/7%/8%。该效果至多可叠加5层，每0.3秒只能触发一次。此外，处于护盾庇护下，该效果的攻击力提升效果提高100%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "无工之剑"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "w1",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 5.0,
                default: 0.0
            }
        },
        ItemConfig {
            name: "shield_rate",
            title: "w15",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheUnforgedEffect::new(config)))
    }
}
