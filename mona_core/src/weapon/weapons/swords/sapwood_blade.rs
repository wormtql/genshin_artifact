use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SapwoodBladeEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for SapwoodBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::ElementalMastery, "原木刀被动等效", (15.0 * refine + 45.0) * self.rate);
    }
}

pub struct SapwoodBlade;

impl WeaponTrait for SapwoodBlade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SapwoodBlade,
        internal_name: "Sword_Arakalari",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "原木刀",
            en: "Sapwood Blade"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::SapwoodBlade { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(SapwoodBladeEffect { rate }))
    }
}