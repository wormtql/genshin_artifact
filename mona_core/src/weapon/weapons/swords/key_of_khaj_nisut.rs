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

pub struct KeyOfKhajNisutEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for KeyOfKhajNisutEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_hp_percentage("圣显之钥被动", 0.05 * refine + 0.15);
        let em_bonus = (0.0003 * refine + 0.0009) * self.stack;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ElementalMastery,
            Box::new(move |hp, _| hp * em_bonus),
            Box::new(move |hp, _, grad| (em_bonus, 0.0)),
            "圣显之钥被动等效"
        );
    }
}

pub struct KeyOfKhajNisut;

impl WeaponTrait for KeyOfKhajNisut {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KeyOfKhajNisut,
        internal_name: "Sword_Deshret",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP144),
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
            WeaponConfig::KeyOfKhajNisut { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(KeyOfKhajNisutEffect { stack }))
    }
}
