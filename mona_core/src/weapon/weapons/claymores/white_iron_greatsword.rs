use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct WhiteIronGreatsword;

impl WeaponTrait for WhiteIronGreatsword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WhiteIronGreatsword,
        internal_name: "Claymore_Tin",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF96),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "击败敌人时，恢复<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>生命值。",
            en: "Defeating an opponent restores <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> HP."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "白铁大剑",
            en: "White Iron Greatsword"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
