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

pub struct StaffOfTheScarletSandsEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for StaffOfTheScarletSandsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let value = (0.13 * refine + 0.39) + (0.07 * refine + 0.21) * self.stack;
        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::ATKFixed,
            Box::new(move |em, _| em * value),
            Box::new(move |em, _, grad| (value * grad, 0.0)),
            "赤沙之杖被动等效",
        );
    }
}

pub struct StaffOfTheScarletSands;

impl WeaponTrait for StaffOfTheScarletSands {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StaffOfTheScarletSands,
        internal_name: "Pole_Deshret",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate96),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(""),
        #[cfg(not(target_family = "wasm"))]
        chs: ""
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::StaffOfTheScarletSands { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(StaffOfTheScarletSandsEffect { stack }))
    }
}
