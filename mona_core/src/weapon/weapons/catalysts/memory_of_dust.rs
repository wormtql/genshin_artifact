use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct MemoryOfDustEffect {
    stack: f64,
    shield_rate: f64
}

impl MemoryOfDustEffect {
    pub fn new(config: &WeaponConfig) -> MemoryOfDustEffect {
        match *config {
            WeaponConfig::MemoryOfDust { stack, shield_rate } => MemoryOfDustEffect {
                stack,
                shield_rate
            },
            _ => MemoryOfDustEffect {
                stack: 0.0,
                shield_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MemoryOfDustEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::ShieldStrength, "尘世之锁被动", refine * 0.05 + 0.15);
        let atk_bonus = (refine * 0.01 + 0.03) * self.stack * (1.0 + self.shield_rate);
        attribute.add_atk_percentage("尘世之锁被动等效", atk_bonus);
    }
}

pub struct MemoryOfDust;

impl WeaponTrait for MemoryOfDust {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MemoryOfDust,
        internal_name: "Catalyst_Kunwu",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("金璋皇极：护盾强效提升20%/25%/30%/35%/40%。攻击命中后的8秒内，攻击力提升4%/5%/6%/7%/8%。该效果至多可叠加5层，每0.3秒只能触发一次。此外，处于护盾庇护下时，该效果的攻击力提升效果提高100%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "尘世之锁"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK05,
        ItemConfig {
            name: "shield_rate",
            title: "w15",
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MemoryOfDustEffect::new(config)))
    }
}
