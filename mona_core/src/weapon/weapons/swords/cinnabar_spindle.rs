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
        internal_name: "Sword_Opus",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF150),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技造成的伤害值提高，提高数值相当于防御力的<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。该效果每1.5秒最多触发一次，并将在元素战技造成伤害后的0.1秒后清除效果。",
            en: "Elemental Skill DMG is increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> of DEF. The effect will be triggered no more than once every 1.5s and will be cleared 0.1s after the Elemental Skill deals DMG."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "辰砂之纺锤",
            en: "Cinnabar Spindle"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
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
