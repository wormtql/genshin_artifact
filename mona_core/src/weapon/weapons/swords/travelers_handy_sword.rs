use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct TravelersHandySword;

impl WeaponTrait for TravelersHandySword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TravelersHandySword,
        internal_name: "Sword_Traveler",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF64),
        weapon_base: WeaponBaseATKFamily::ATK448,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "获得元素晶球或元素微粒时，恢复<span style=\"color: #409EFF;\">1%-1.25%-1.5%-1.75%-2%</span>生命值。",
            en: "Each Elemental Orb or Particle collected restores <span style=\"color: #409EFF;\">1%-1.25%-1.5%-1.75%-2%</span> HP."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "旅行剑",
            en: "Traveler's Handy Sword"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
