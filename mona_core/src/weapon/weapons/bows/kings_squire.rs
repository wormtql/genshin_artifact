use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::macros::config_rate01;

pub struct KingsSquireEffect {
    rate: f64
}

impl<A: Attribute> WeaponEffect<A> for KingsSquireEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::ElementalMastery, "王下近侍被动等效", self.rate * (refine * 20.0 + 40.0));
    }
}

pub struct KingsSquire;

impl WeaponTrait for KingsSquire {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KingsSquire,
        internal_name: "Bow_Arakalari",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "王下近侍",
            en: "King's Squire"
        )
    };

    config_rate01!();

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::KingsSquire { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(KingsSquireEffect { rate }))
    }
}
