use crate::attribute::{Attribute, AttributeName};
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

pub struct SharpshootersOathEffect {
    pub rate: f64,
}

impl SharpshootersOathEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for SharpshootersOathEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.06 + 0.18) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "神射手之誓被动等效", value);
    }
}

pub struct SharpshootersOath;

impl WeaponTrait for SharpshootersOath {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SharpshootersOath,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage102),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("精准：针对要害造成的伤害提升24%/30%/36%/42%/48%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "神射手之誓"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::SharpshootersOath { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(SharpshootersOathEffect::new(rate)))
    }
}