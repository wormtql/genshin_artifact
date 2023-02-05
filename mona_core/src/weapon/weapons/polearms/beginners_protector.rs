use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;

pub struct BeginnersProtector;

impl WeaponTrait for BeginnersProtector {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BeginnersProtector,
        internal_name: "Pole_Gewalt",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: None,
        weapon_base: WeaponBaseATKFamily::ATK185,
        star: 1,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "新手长枪",
            en: "Beginner's Protector"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
