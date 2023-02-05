use crate::attribute::Attribute;
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
use crate::weapon::weapons::sapwood_blade::SapwoodBladeEffect;

pub struct ForestRegalia;

impl WeaponTrait for ForestRegalia {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ForestRegalia,
        internal_name: "Claymore_Arakalari",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "森林王器",
            en: "Forest Regalia"
        )
    };

    config_rate01!();

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::ForestRegalia { rate } => rate,
            _ => 0.0
        };

        // reuse same effect
        Some(Box::new(SapwoodBladeEffect { rate }))
    }
}