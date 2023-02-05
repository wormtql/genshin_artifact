use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct OtherworldlyStory;

impl WeaponTrait for OtherworldlyStory {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::OtherworldlyStory,
        internal_name: "Catalyst_Lightnov",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge85),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "获得元素微粒或元素晶球时，恢复<span style=\"color: #409EFF;\">1%-1.25%-1.5%-1.75%-2%</span>生命值。",
            en: "Picking up an Elemental Energy Orb/Particle recovers <span style=\"color: #409EFF;\">1%-1.25%-1.5%-1.75%-2%</span> HP."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "异世界行记",
            en: "Otherworldly Story"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
