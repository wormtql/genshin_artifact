use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct MissiveWindspearEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for MissiveWindspearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("风信之锋被动等效", (0.03 * refine + 0.09) * self.rate);
        attribute.set_value_by(AttributeName::ElementalMastery, "风信之锋被动等效", (12.0 * refine + 36.0) * self.rate);
    }
}

pub struct MissiveWindspear;

impl WeaponTrait for MissiveWindspear {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MissiveWindspear,
        internal_name: "Pole_Windvane",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(""),
        #[cfg(not(target_family = "wasm"))]
        chs: ""
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::MissiveWindspear { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(MissiveWindspearEffect { rate }))
    }
}
