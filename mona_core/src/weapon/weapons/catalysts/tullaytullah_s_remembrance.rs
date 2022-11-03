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

pub struct TullaytullahsRemembranceEffect {
    pub stack:f64,
}

impl<A: Attribute> WeaponEffect<A> for TullaytullahsRemembranceEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let interval = 0.036 + 0.012*refine;

        attribute.set_value_by(AttributeName::BonusNormalAttack, "图莱杜拉的回忆被动", interval*self.stack);
    }
}

pub struct TullaytullahsRemembrance;

impl WeaponTrait for TullaytullahsRemembrance {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TullaytullahsRemembrance,
        internal_name: "Alaya", // todo
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(""),
        #[cfg(not(target_family = "wasm"))]
        chs: ""
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "w1",
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 7.0 },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::TullaytullahsRemembrance { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(TullaytullahsRemembranceEffect {
            stack
        }))
    }
}