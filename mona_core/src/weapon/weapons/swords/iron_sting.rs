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

pub struct IronStingEffect {
    pub stack: f64,
}

impl IronStingEffect {
    pub fn new(config: &WeaponConfig) -> IronStingEffect {
        match *config {
            WeaponConfig::IronSting { stack } => IronStingEffect {
                stack,
            },
            _ => IronStingEffect {
                stack: 0.0,
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for IronStingEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.015 + 0.045) * self.stack;
        attribute.set_value_by(AttributeName::BonusBase, "铁蜂刺被动等效", value);
    }
}

pub struct IronSting;

impl WeaponTrait for IronSting {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::IronSting,
        internal_name: "Sword_Exotic",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "造成元素伤害后的6秒内，角色造成的伤害提高<span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>，该效果最多叠加2层。该效果每1秒可以触发一次。",
            en: "Dealing Elemental DMG increases all DMG by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span> for 6s. Max 2 stacks. Can occur once every 1s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "铁蜂刺",
            en: "Iron Sting"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float {
                min: 0.0,
                max: 2.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(IronStingEffect::new(config)))
    }
}
