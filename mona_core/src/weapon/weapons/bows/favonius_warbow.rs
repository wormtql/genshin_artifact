use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct FavoniusWarbow;

impl WeaponTrait for FavoniusWarbow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FavoniusWarbow,
        internal_name: "Bow_Zephyrus",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge133),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击造成暴击时，有<span style=\"color: #409EFF;\">60%-70%-80%-90%-100%</span>的几率产生少量元素微粒，能为角色恢复6点元素能量。该效果每<span style=\"color: #409EFF;\">12-10.5-9-7.5-6</span>秒只能触发一次。",
            en: "CRIT Hits have a <span style=\"color: #409EFF;\">60%-70%-80%-90%-100%</span> chance to generate a small amount of Elemental Particles, which will regenerate 6 Energy for the character. Can only occur once every <span style=\"color: #409EFF;\">12-10.5-9-7.5-6</span>s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "西风猎弓",
            en: "Favonius Warbow"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
