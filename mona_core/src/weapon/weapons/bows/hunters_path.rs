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

pub struct HuntersPathEffect {
    rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for HuntersPathEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_elemental_bonus("猎人之径被动", refine * 0.03 + 0.09);

        let rate = self.rate;

        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::ExtraDmgChargedAttack,
            Box::new(move |em: f64, _| (0.4 * refine + 1.2) * em * rate),
            Box::new(move |grad, x1, _x2| (grad * (0.4 * refine + 1.2) * rate, 0.0)),
            "猎人之径被动等效"
        );
    }
}

pub struct HuntersPath;

impl WeaponTrait for HuntersPath {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HuntersPath,
        internal_name: "Bow_Ayus",
        weapon_type: WeaponType::Bow,
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
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::HuntersPath { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(HuntersPathEffect { rate }))
    }
}