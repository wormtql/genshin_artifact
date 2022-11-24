use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct ToukabouShigureEffect {
    pub rate: f64,
}

impl ToukabouShigureEffect {
    pub fn new(config: &WeaponConfig) -> ToukabouShigureEffect {
        match *config {
            WeaponConfig::ToukabouShigure { rate } => ToukabouShigureEffect {
                rate,
            },
            _ => ToukabouShigureEffect {
                rate: 0.0,
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ToukabouShigureEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.12) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "东花坊时雨被动等效", value);
    }
}

pub struct ToukabouShigure;

impl WeaponTrait for ToukabouShigure {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ToukabouShigure,
        internal_name: "", //TODO
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("攻击命中敌人后，会为命中的一名敌人施加「纸伞作祟」状态，持续10秒。该效果每15秒至多触发一次；持续期间该敌人被击败时，将清除该效果的冷却时间。装备者对处于「纸伞作祟」状态下的敌人造成的伤害提升16%-20%-24%-28%-32%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "东花坊时雨"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "w2",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(ToukabouShigureEffect::new(config)))
    }
}
