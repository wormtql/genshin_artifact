use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct XiphosMoonlightEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for XiphosMoonlightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.00009 + 0.00027) * self.rate;
        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::Recharge,
            Box::new(move |x, _| value * x),
            Box::new(move |x, y, grad| (value * grad, 0.0)),
            "西福斯的月光被动",
        );
    }
}

pub struct XiphosMoonlight;

impl WeaponTrait for XiphosMoonlight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::XiphosMoonlight,
        internal_name: "Sword_Pleroma",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(""),
        #[cfg(not(target_family = "wasm"))]
        chs: "西福斯的月光"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::XiphosMoonlight { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(XiphosMoonlightEffect { rate }))
    }
}
