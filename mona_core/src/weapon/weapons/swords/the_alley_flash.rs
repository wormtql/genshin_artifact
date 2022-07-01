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

pub struct TheAlleyFlashEffect {
    rate: f64,
}

impl TheAlleyFlashEffect {
    pub fn new(config: &WeaponConfig) -> TheAlleyFlashEffect {
        match *config {
            WeaponConfig::TheAlleyFlash { rate } => TheAlleyFlashEffect {
                rate,
            },
            _ => TheAlleyFlashEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheAlleyFlashEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusBase, "暗巷闪光被动", value * self.rate);
    }
}

pub struct TheAlleyFlash;

impl WeaponTrait for TheAlleyFlash {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheAlleyFlash,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM12),
        weapon_base: WeaponBaseATKFamily::ATK620,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("街巷游侠：造成的伤害提升12%/15%/18%/21%/24%，受到伤害后的5秒内失效。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "暗巷闪光"
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
        Some(Box::new(TheAlleyFlashEffect::new(config)))
    }
}
