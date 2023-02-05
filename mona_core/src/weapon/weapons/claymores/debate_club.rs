use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct DebateClub;

impl WeaponTrait for DebateClub {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DebateClub,
        internal_name: "Claymore_Reasoning",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后，普通攻击和重击命中时会在小范围内额外造成<span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span>攻击力的伤害。该效果持续15秒，伤害每3秒只能触发一次。",
            en: "After using an Elemental Skill, on hit, Normal and Charged Attacks deal additional DMG equal to <span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span> of ATK in a small AoE. Effect lasts 15s. DMG can only occur once every 3s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "以理服人",
            en: "Debate Club"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
