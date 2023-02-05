use smallvec::SmallVec;
use crate::common::i18n::I18nLocale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::WeaponName;
use super::weapon_sub_stat::{WeaponSubStatFamily};
use super::weapon_base_atk::WeaponBaseATKFamily;

pub struct WeaponStaticData {
    pub name: WeaponName,
    pub internal_name: &'static str,
    pub weapon_type: WeaponType,
    pub weapon_sub_stat: Option<WeaponSubStatFamily>,
    pub weapon_base: WeaponBaseATKFamily,
    pub star: usize,
    #[cfg(not(target_family = "wasm"))]
    pub effect: Option<I18nLocale>,
    #[cfg(not(target_family = "wasm"))]
    pub name_locale: I18nLocale,
}
