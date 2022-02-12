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

pub struct DodocoTalesEffect {
    rate1: f64,
    rate2: f64
}

impl DodocoTalesEffect {
    pub fn new(config: &WeaponConfig) -> DodocoTalesEffect {
        match *config {
            WeaponConfig::DodocoTales { rate1, rate2 } => DodocoTalesEffect {
                rate1,
                rate2
            },
            _ => DodocoTalesEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DodocoTalesEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::BonusChargedAttack, "嘟嘟可故事集被动等效", (refine * 0.04 + 0.12) * self.rate1);
        attribute.add_atk_percentage("嘟嘟可故事集被动等效", (refine * 0.02 + 0.06) * self.rate2);
    }
}

pub struct DodocoTales;

impl WeaponTrait for DodocoTales {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DodocoTales,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: WeaponSubStatFamily::ATK120,
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        effect: Some("普通攻击命中敌人后的6秒内，重击造成的伤害提升16/20/24/28/32%；重击命中敌人后的6秒内，攻击力提升8/10/12/14/16%。"),
        chs: "嘟嘟可故事集"
    };

    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: "效果1比例",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "rate2",
            title: "效果2比例",
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(DodocoTalesEffect::new(config)))
    }
}
