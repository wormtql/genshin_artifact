use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct PrototypeRancourEffect {
    stack: f64
}

impl PrototypeRancourEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeRancourEffect {
        match *config {
            WeaponConfig::PrototypeRancour { stack } => PrototypeRancourEffect {
                stack,
            },
            _ => PrototypeRancourEffect {
                stack: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeRancourEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.01 + 0.03) * self.stack;
        attribute.add_atk_percentage("试作斩岩被动等效", value);
        attribute.add_def_percentage("试作斩岩被动等效", value);
    }
}

pub struct PrototypeRancour;

impl WeaponTrait for PrototypeRancour {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeRancour,
        internal_name: "Sword_Proto",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus75),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击或重击命中时，攻击力和防御力提高<span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span>，持续6秒，最多叠加4层。该效果每0.3秒只能触发一次。",
            en: "On hit, Normal or Charged Attacks increase ATK and DEF by <span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span> for 6s. Max 4 stacks. This effect can only occur once every 0.3s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "试作斩岩",
            en: "Prototype Rancour"
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
        Some(Box::new(PrototypeRancourEffect::new(config)))
    }
}
