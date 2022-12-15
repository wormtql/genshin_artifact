use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;

pub struct TulaytullahsRemembranceEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for TulaytullahsRemembranceEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let interval = 0.036 + 0.012 * refine;
        let value = interval * 10.0_f64.min(self.stack);

        attribute.set_value_by(AttributeName::BonusNormalAttack, "图莱杜拉的回忆被动", value);
    }
}

pub struct TulaytullahsRemembrance;

impl WeaponTrait for TulaytullahsRemembrance {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TulaytullahsRemembrance,
        internal_name: "Catalyst_Alaya",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(""),
        #[cfg(not(target_family = "wasm"))]
        chs: "",
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
            WeaponConfig::TulaytullahsRemembrance { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(TulaytullahsRemembranceEffect {
            stack
        }))
    }
}