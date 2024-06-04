use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct CloudforgedEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for CloudforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::ElementalMastery, "筑云被动", 30.0 + refine * 10.0);
    }
}

pub struct Cloudforged;

impl WeaponTrait for Cloudforged {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Cloudforged,
        internal_name: "Cloudforged",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素能量减少后，装备者的元素精通提升<span style=\"color: #409EFF;\">40-50-60-70-80</span>点。该效果持续18秒，至多叠加2层。",
            en: "After Elemental Energy is decreased, the equipping character's Elemental Mastery will increase by <span style=\"color: #409EFF;\">40-50-60-70-80</span> for 18s. Max 2 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "筑云",
            en: "Cloudforged",
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK02
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::Cloudforged { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(CloudforgedEffect { stack }))
    }
}
