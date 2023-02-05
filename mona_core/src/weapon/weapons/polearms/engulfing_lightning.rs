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

pub struct EngulfingLightningEffect {
    rate: f64,
}

impl EngulfingLightningEffect {
    pub fn new(config: &WeaponConfig) -> EngulfingLightningEffect {
        match *config {
            WeaponConfig::EngulfingLightning { rate } => EngulfingLightningEffect {
                rate
            },
            _ => EngulfingLightningEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for EngulfingLightningEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let recharge_bonus = (refine * 0.05 + 0.25) * self.rate;
        attribute.set_value_by(AttributeName::Recharge, "薙草之稻光被动等效", recharge_bonus);

        let max_bonus = refine * 0.1 + 0.7;
        let value = refine * 0.07 + 0.21;
        attribute.add_edge2(
            AttributeName::ATKBase,
            AttributeName::Recharge,
            AttributeName::ATKPercentage,
            Box::new(move |atk_base, recharge| ((recharge - 1.0) * value).min(max_bonus) * atk_base),
            Box::new(move |grad, atk_base, recharge| {
                let grad_atk_base = grad * ((recharge - 1.0) * value).min(max_bonus);
                let grad_recharge = if (recharge - 1.0) * value <= max_bonus {
                    value * atk_base
                } else {
                    0.0
                };

                (grad_atk_base, grad_recharge)
            }),
            "薙草之稻光被动等效"
        )
    }
}

pub struct EngulfingLightning;

impl WeaponTrait for EngulfingLightning {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EngulfingLightning,
        internal_name: "Pole_Narukami",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge120),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力获得提升，提升程度相当于元素充能效率超出100%部分的<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，至多通过这种方式提升<span style=\"color: #409EFF;\">80%-90%-100%-110%-120%</span>。施放元素爆发后的12秒内，元素充能效率提升<span style=\"color: #409EFF;\">30%-35%-40%-45%-50%</span>。",
            en: "ATK increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> of Energy Recharge over the base 100%. You can gain a maximum bonus of <span style=\"color: #409EFF;\">80%-90%-100%-110%-120%</span> ATK. Gain <span style=\"color: #409EFF;\">30%-35%-40%-45%-50%</span> Energy Recharge for 12s after using an Elemental Burst."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "薙草之稻光",
            en: "Engulfing Lightning"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfig::RATE01_TYPE,
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(EngulfingLightningEffect::new(config)))
    }
}
