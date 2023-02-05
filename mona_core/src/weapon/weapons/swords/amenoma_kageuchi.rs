use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{WeaponType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct AmenomaKageuchi;

impl WeaponTrait for AmenomaKageuchi {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AmenomaKageuchi,
        internal_name: "Sword_Bakufu",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后，获得1个胤种，该效果每5秒至多触发一次。胤种持续30秒，至多同时存在3个。施放元素爆发后，会清除持有的所有胤种，并在2秒之后，基于消耗的胤种数量，每个为该角色恢复<span style=\"color: #409EFF;\">6-7.5-9-10.5-12</span>点元素能量。",
            en: "After casting an Elemental Skill, gain 1 Succession Seed. This effect can be triggered once every 5s. The Succession Seed lasts for 30s. Up to 3 Succession Seeds may exist simultaneously. After using an Elemental Burst, all Succession Seeds are consumed and after 2s, the character regenerates <span style=\"color: #409EFF;\">6-7.5-9-10.5-12</span> Energy for each seed consumed."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "天目影打刀",
            en: "Amenoma Kageuchi"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
