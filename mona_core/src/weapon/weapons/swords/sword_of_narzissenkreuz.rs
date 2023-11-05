use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SwordOfNarzissenkreuz;

impl WeaponTrait for SwordOfNarzissenkreuz {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SwordOfNarzissenkreuz,
        internal_name: "Sword_Purewill",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "装备者不具备「始基力」时：普通攻击、重击、下落攻击命中时，会释放芒性或荒性的能量冲击，造成<span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span>攻击力的伤害。该效果每12秒至多触发一次，能量冲击的类型由水仙十字之剑当前的属性决定。",
            en: "When the equipping character does not have an Arkhe: When Normal Attacks, Charged Attacks, or Plunging Attacks strike, a Pneuma or Ousia energy blast will be unleashed, dealing <span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span> of ATK as DMG. This effect can be triggered once every 12s. The energy blast type is determined by the current type of the Sword of Narzissenkreuz."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "水仙十字之剑",
            en: "Sword of Narzissenkreuz"
        )
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
