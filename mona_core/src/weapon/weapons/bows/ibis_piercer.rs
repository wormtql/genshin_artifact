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

pub struct IbisPiercerEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for IbisPiercerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let value = data.refine * 10 + 30;
        attribute.set_value_by(AttributeName::ElementalMastery, "鹮穿之喙被动等效", value as f64 * self.stack);
    }
}

pub struct IbisPiercer;

impl WeaponTrait for IbisPiercer {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::IbisPiercer,
        internal_name: "Bow_Ibis",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "重击命中敌人后的6秒内，角色元素精通提升<span style=\"color: #409EFF;\">40-50-60-70-80</span>点。该效果至多叠加2层，每0.5秒至多触发一次。",
            en: "The character's Elemental Mastery will increase by <span style=\"color: #409EFF;\">40-50-60-70-80</span> within 6s after Charged Attacks hit opponents. Max 2 stacks. This effect can be triggered once every 0.5s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "鹮穿之喙",
            en: "Ibis Piercer",
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "层数",
                en: "Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 2.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::IbisPiercer { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(IbisPiercerEffect {
            stack
        }))
    }
}
