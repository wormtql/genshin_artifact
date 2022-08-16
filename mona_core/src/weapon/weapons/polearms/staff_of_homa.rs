use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct StaffOfHomaEffect {
    be50_rate: f64
}

impl StaffOfHomaEffect {
    pub fn new(config: &WeaponConfig) -> StaffOfHomaEffect {
        match *config {
            WeaponConfig::StaffOfHoma { be50_rate } => StaffOfHomaEffect {
                be50_rate
            },
            _ => StaffOfHomaEffect {
                be50_rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for StaffOfHomaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let hp_bonus = refine * 0.05 + 0.15;
        attribute.add_hp_percentage("护摩之杖被动", hp_bonus);
        let atk_bonus_ratio = refine * 0.002 + 0.006 + (refine * 0.002 + 0.008) * self.be50_rate;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * atk_bonus_ratio),
            Box::new(move |grad, _x1, _x2| (grad * atk_bonus_ratio, 0.0)),
            "护摩之杖被动等效"
        );
    }
}

pub struct StaffOfHoma;

impl WeaponTrait for StaffOfHoma {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StaffOfHoma,
        internal_name: "Pole_Homa",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("无羁的朱赤之蝶：生命值提升20%/25%/30%/35%/40%。此外，提供基于装备该武器的角色生命值上限的0.8%/1%/1.2%/1.4%/1.6%，获得攻击力加成。当装备该武器的角色生命值低于50%时，进一步获得1%/1.2%/1.4%/1.6%/1.8%最大生命值上限的攻击力提升。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "护摩之杖"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "be50_rate",
            title: "w24",
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(StaffOfHomaEffect::new(config)))
    }
}
