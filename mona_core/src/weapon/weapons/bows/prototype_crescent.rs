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

pub struct PrototypeCrescentEffect {
    rate: f64
}

impl PrototypeCrescentEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeCrescentEffect {
        match *config {
            WeaponConfig::PrototypeCrescent { rate } => PrototypeCrescentEffect {
                rate
            },
            _ => PrototypeCrescentEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeCrescentEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.09 + 0.27) * self.rate;
        attribute.add_atk_percentage("试作澹月被动等效", value);
    }
}

pub struct PrototypeCrescent;

impl WeaponTrait for PrototypeCrescent {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeCrescent,
        internal_name: "Bow_Proto",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "重击若命中要害，则提升10%移动速度与<span style=\"color: #409EFF;\">36%-45%-54%-63%-72%</span>攻击力，持续10秒。",
            en: "Charged Attack hits on weak points increase Movement SPD by 10% and ATK by <span style=\"color: #409EFF;\">36%-45%-54%-63%-72%</span> for 10s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "试作澹月",
            en: "Prototype Crescent"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PrototypeCrescentEffect::new(config)))
    }
}
