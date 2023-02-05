use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct FruitOfFulfillmentEffect {
    pub stack: f64
}

impl<A: Attribute> WeaponEffect<A> for FruitOfFulfillmentEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::ElementalMastery, "盈满之实被动等效", self.stack * (refine * 3.0 + 21.0));
        attribute.add_atk_percentage("盈满之实被动等效", self.stack * -0.05);
    }
}

pub struct FruitOfFulfillment;

impl WeaponTrait for FruitOfFulfillment {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FruitOfFulfillment,
        internal_name: "Catalyst_Arakalari",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "盈满之实",
            en: "Fruit of Fulfillment"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK05
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::FruitOfFulfillment { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(FruitOfFulfillmentEffect { stack }))
    }
}