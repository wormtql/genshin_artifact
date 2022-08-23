use crate::attribute::{Attribute, AttributeCommon};
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

pub struct BlackcliffAgateEffect {
    stack: f64
}

impl BlackcliffAgateEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffAgateEffect {
        match *config {
            WeaponConfig::BlackcliffAgate { stack } => BlackcliffAgateEffect {
                stack
            },
            _ => BlackcliffAgateEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffAgateEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let atk_bonus = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("黑岩绯玉被动等效", atk_bonus);
    }
}

pub struct BlackcliffAgate;

impl WeaponTrait for BlackcliffAgate {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BlackcliffAgate,
        internal_name: "Catalyst_Blackrock",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage120),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("乘胜追击：击败敌人后，攻击力提升12%/15%/18%/21%/24%，续30秒。该效果至多叠加3层，每层持续时间独立。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "黑岩绯玉"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BlackcliffAgateEffect::new(config)))
    }
}
