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

pub struct WindAndSongEffect {
    rate: f64
}

impl WindAndSongEffect {
    pub fn new(config: &WeaponConfig) -> WindAndSongEffect {
        match *config {
            WeaponConfig::WindAndSong { rate } => WindAndSongEffect {
                rate
            },
            _ => WindAndSongEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WindAndSongEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("暗巷的酒与诗被动等效", value);
    }
}

pub struct WindAndSong;

impl WeaponTrait for WindAndSong {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WindAndSong,
        internal_name: "Catalyst_Outlaw",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("变化万端：普通攻击命中敌人后，冲刺或替代冲刺的能力消耗的体力降低14%/16%/18%/20%/22%，持续5秒。此外，使用冲刺或替代冲刺的能力后，攻击力提升20%/25%/30%/35%/40%，持续5秒。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "暗巷的酒与诗"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(WindAndSongEffect::new(config)))
    }
}
