use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const ENGULFING_LIGHTNING_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::Recharge120,
    weapon_base: WeaponBaseATKFamily::ATK608,
    star: 5
};

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