use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct Messenger;

impl WeaponTrait for Messenger {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Messenger,
        internal_name: "Bow_Msg",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage68),
        weapon_base: WeaponBaseATKFamily::ATK448,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "重击若命中要害，则额外造成<span style=\"color: #409EFF;\">100%-125%-150%-175%-200%</span>攻击力的伤害，该伤害必定暴击。此效果每10秒只能触发一次。",
            en: "Charged Attack hits on weak points deal an additional <span style=\"color: #409EFF;\">100%-125%-150%-175%-200%</span> ATK DMG as CRIT DMG. Can only occur once every 10s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "信使",
            en: "Messenger"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
