use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct AThousandFloatingDreamsEffect {
    pub same_count: usize,
    pub diff_count: usize,
}

impl<A: Attribute> WeaponEffect<A> for AThousandFloatingDreamsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let em_delta = refine * 8.0 + 24.0;
        let bonus_delta = refine * 0.04 + 0.06;

        attribute.set_value_by(AttributeName::ElementalMastery, "千夜浮梦被动", em_delta * self.same_count.min(3) as f64);
        attribute.add_elemental_bonus("千夜浮梦被动", bonus_delta * self.diff_count.min(3) as f64);
    }
}

pub struct AThousandFloatingDreams;

impl WeaponTrait for AThousandFloatingDreams {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AThousandFloatingDreams,
        internal_name: "Catalyst_Ayus",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM58),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(""),
        #[cfg(not(target_family = "wasm"))]
        chs: ""
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "same_count",
            title: "w29",
            config: ItemConfigType::Int { min: 0, max: 3, default: 1 },
        },
        ItemConfig {
            name: "diff_count",
            title: "w30",
            config: ItemConfigType::Int { min: 0, max: 3, default: 2 },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (same_count, diff_count) = match *config {
            WeaponConfig::AThousandFloatingDreams { same_count, diff_count } => (same_count, diff_count),
            _ => (0, 0)
        };

        Some(Box::new(AThousandFloatingDreamsEffect {
            same_count,
            diff_count,
        }))
    }
}