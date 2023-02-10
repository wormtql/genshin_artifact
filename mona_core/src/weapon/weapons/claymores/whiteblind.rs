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

pub struct WhiteblindEffect {
    stack: f64,
}

impl WhiteblindEffect {
    pub fn new(config: &WeaponConfig) -> WhiteblindEffect {
        match *config {
            WeaponConfig::Whiteblind { stack } => WhiteblindEffect {
                stack
            },
            _ => WhiteblindEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WhiteblindEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.015 + 0.045) * self.stack;
        attribute.add_atk_percentage("白影剑被动等效", value);
        attribute.add_def_percentage("白影剑被动等效", value);
    }
}

pub struct Whiteblind;

impl WeaponTrait for Whiteblind {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Whiteblind,
        internal_name: "Claymore_Exotic",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF113),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击和重击命中后，攻击力和防御力提高<span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>。该效果持续6秒，最多叠加4层，每0.5秒只能触发一次。",
            en: "On hit, Normal or Charged Attacks increase ATK and DEF by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span> for 6s. Max 4 stacks. This effect can only occur once every 0.5s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "白影剑",
            en: "Whiteblind"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float {
                min: 0.0,
                max: 4.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(WhiteblindEffect::new(config)))
    }
}
