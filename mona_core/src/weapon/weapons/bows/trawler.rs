use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct Trawler;

impl WeaponTrait for Trawler {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Trawler,
        internal_name: "Bow_Fin",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后，将触发「沿洄」效果，在攻击命中敌人时造成<span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span>攻击力的范围伤害，该效果将在15秒或触发3次范围伤害后移除。每2秒至多通过这种方式造成一次范围伤害，每12秒至多触发一次沿洄。",
            en: "Triggers the Flowrider effect after using an Elemental Skill, dealing <span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span> ATK as AoE DMG upon hitting an opponent with an attack. Flowrider will be removed after 15s or after causing 3 instances of AoE DMG. Only 1 instance of AoE DMG can be caused every 2s in this way. Flowrider can be triggered once every 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "竭泽",
            en: "Trawler"
        )
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
