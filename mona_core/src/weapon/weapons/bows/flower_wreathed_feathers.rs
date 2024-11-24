use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

struct FlowerWreathedFeathersEffect {
    stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for FlowerWreathedFeathersEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus = 0.045 + 0.015 * refine;
        attribute.set_value_by(AttributeName::BonusChargedAttack, "缀花之翎被动", bonus * self.stack.min(6.0));
    }
}

pub struct FlowerWreathedFeathers;

impl WeaponTrait for FlowerWreathedFeathers {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FlowerWreathedFeathers,
        internal_name: "Bow_Umpakati",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "滑翔消耗的体力降低15%。进行瞄准射击时，每0.5秒提升重击造成的伤害<span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>，该效果最多叠加6层，并将在不处于瞄准状态下10秒后移除。",
            en: "Decreases Gliding Stamina consumption by 15%. When using Aimed Shots, the DMG dealt by Charged Attacks increases by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span> every 0.5s. This effect can stack up to 6 times and will be removed 10s after leaving Aiming Mode."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "缀花之翎",
            en: "Flower-Wreathed Feathers"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "被动层数",
                en: "Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 6.0, default: 0.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::FlowerWreathedFeathers { stack } => Some(Box::new(FlowerWreathedFeathersEffect {
                stack
            })),
            _ => None
        }
    }
}
