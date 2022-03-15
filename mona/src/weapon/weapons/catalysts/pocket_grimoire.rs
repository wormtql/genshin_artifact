use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;

pub struct PocketGrimoire;

impl WeaponTrait for PocketGrimoire {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PocketGrimoire,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: None,
        weapon_base: WeaponBaseATKFamily::ATK243,
        star: 2,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        chs: "口袋魔导书"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
