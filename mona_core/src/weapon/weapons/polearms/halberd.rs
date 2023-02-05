use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct Halberd;

impl WeaponTrait for Halberd {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Halberd,
        internal_name: "Pole_Halberd",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK51),
        weapon_base: WeaponBaseATKFamily::ATK448,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "对普通攻击命中的敌人造成<span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span>攻击力的额外伤害。该效果每10秒只能触发一次。",
            en: "Normal Attacks deal an additional <span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span> ATK as DMG. Can only occur once every 10s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "钺矛",
            en: "Halberd"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
