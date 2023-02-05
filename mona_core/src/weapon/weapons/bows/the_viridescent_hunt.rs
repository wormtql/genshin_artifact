use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct TheViridescentHunt;

impl WeaponTrait for TheViridescentHunt {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheViridescentHunt,
        internal_name: "Bow_Viridescent",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate60),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击与重击命中时，有50%几率生成一个风之眼，持续吸引周围敌人，并对其中的敌人每0.5秒造成<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>攻击的伤害。该效果持续4秒，每<span style=\"color: #409EFF;\">14-13-12-11-10</span>秒至多触发一次。",
            en: "Upon hit, Normal and Charged Attacks have a 50% chance to generate a Cyclone, which will continuously attract surrounding opponents, dealing <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> of ATK as DMG to these opponents every 0.5s for 4s. This effect can only occur once every <span style=\"color: #409EFF;\">14-13-12-11-10</span>s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "苍翠猎弓",
            en: "The Viridescent Hunt"
        )
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
