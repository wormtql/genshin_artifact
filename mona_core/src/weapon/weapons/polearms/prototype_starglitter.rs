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

pub struct PrototypeStarglitterEffect {
    stack: f64
}

impl PrototypeStarglitterEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeStarglitterEffect {
        match *config {
            WeaponConfig::PrototypeStarglitter { stack } => PrototypeStarglitterEffect {
                stack
            },
            _ => PrototypeStarglitterEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeStarglitterEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.02 + 0.06) * self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "试作星镰", value);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "试作星镰", value);
    }
}

pub struct PrototypeStarglitter;

impl WeaponTrait for PrototypeStarglitter {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeStarglitter,
        internal_name: "Pole_Proto",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后，普通攻击和重击造成的伤害提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>。该效果持续12秒，最多叠加2层。",
            en: "After using an Elemental Skill, increases Normal and Charged Attack DMG by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> for 12s. Max 2 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "试作星镰",
            en: "Prototype Starglitter"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 },
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PrototypeStarglitterEffect::new(config)))
    }
}
