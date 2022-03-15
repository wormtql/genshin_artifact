use crate::attribute::Attribute;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::WeaponConfig;

pub trait WeaponTrait {
    const META_DATA: WeaponStaticData;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>>;
}
