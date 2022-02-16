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

pub struct TheBellEffect {
    rate: f64
}

impl TheBellEffect {
    pub fn new(config: &WeaponConfig) -> TheBellEffect {
        match *config {
            WeaponConfig::TheBell { rate } => TheBellEffect {
                rate
            },
            _ => TheBellEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheBellEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "钟剑被动等效", value);
    }
}

pub struct TheBell;

impl WeaponTrait for TheBell {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheBell,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: WeaponSubStatFamily::HP90,
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("叛逆的守护者：受到伤害时，生成一个伤害吸收等同于生命上限20%/23%/26%/29%/32%的护盾，持续10秒或者直到护盾失效，每45秒只能触发一次。角色处于护盾庇护下时，造成的伤害提升12%/15%/18%/21%/24%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "钟剑"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "被动应用比例",
            config: ItemConfigType::Float {
                min: 0.0, max: 1.0, default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheBellEffect::new(config)))
    }
}
