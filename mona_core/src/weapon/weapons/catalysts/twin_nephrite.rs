use crate::attribute::{Attribute, AttributeCommon};
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

pub struct TwinNephriteEffect {
    pub rate: f64
}

impl TwinNephriteEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for TwinNephriteEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.02 + 0.1) * self.rate;
        attribute.add_atk_percentage("甲级宝珏被动等效", value);
    }
}

pub struct TwinNephrite;

impl WeaponTrait for TwinNephrite {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TwinNephrite,
        internal_name: "Catalyst_Phoney",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate34),
        weapon_base: WeaponBaseATKFamily::ATK448,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "击败敌人后的15秒内，移动速度和攻击力提升<span style=\"color: #409EFF;\">12%-14%-16%-18%-20%</span>。",
            en: "Defeating an opponent increases Movement SPD and ATK by <span style=\"color: #409EFF;\">12%-14%-16%-18%-20%</span> for 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "甲级宝珏",
            en: "Twin Nephrite"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::TwinNephrite { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(TwinNephriteEffect::new(rate)))
    }
}
