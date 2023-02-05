use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct PrototypeArchaic;

impl WeaponTrait for PrototypeArchaic {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeArchaic,
        internal_name: "Claymore_Proto",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击和重击命中时，有50%的概率对小范围内的敌人造成<span style=\"color: #409EFF;\">240%-300%-360%-420%-480%</span>攻击力的额外伤害。该效果每15秒只能触发一次。",
            en: "On hit, Normal or Charged Attacks have a 50% chance to deal an additional <span style=\"color: #409EFF;\">240%-300%-360%-420%-480%</span> ATK DMG to opponents within a small AoE. Can only occur once every 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "试作古华",
            en: "Prototype Archaic"
        )
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
