use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::macros::config_rate01;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct MoonpiercerEffect {
    rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for MoonpiercerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("贯月矢被动等效", self.rate * (refine * 0.04 + 0.12));
    }
}

pub struct Moonpiercer;

impl WeaponTrait for Moonpiercer {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Moonpiercer,
        internal_name: "Pole_Arakalari",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM24),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "贯月矢",
            en: "Moonpiercer"
        )
    };

    config_rate01!();

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::Moonpiercer { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(MoonpiercerEffect { rate }))
    }
}
