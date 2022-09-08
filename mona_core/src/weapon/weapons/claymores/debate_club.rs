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
        effect: Some("有话直说：施放元素战技后，普通攻击和重击命中时会在小范围内额外造成60%/75%/90%/105%/120%攻击力的伤害。该效果持续15秒，伤害每3秒只能触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "以理服人"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
