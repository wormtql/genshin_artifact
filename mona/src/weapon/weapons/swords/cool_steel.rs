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

pub struct CoolSteelEffect {
    pub rate: f64
}

impl CoolSteelEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for CoolSteelEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.03 + 0.09) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "冷刃被动等效", value);
    }
}

pub struct CoolSteel;

impl WeaponTrait for CoolSteel {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CoolSteel,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("止水融冰：对处于水元素或冰元素影响下的敌人，造成的伤害提高12%/15%/18%/21%/24%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "冷刃"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::CoolSteel { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(CoolSteelEffect::new(rate)))
    }
}
