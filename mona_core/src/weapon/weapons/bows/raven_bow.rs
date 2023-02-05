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

pub struct RavenBowEffect {
    pub rate: f64
}

impl RavenBowEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for RavenBowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.03 + 0.09) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "鸦羽弓被动等效", value);
    }
}

pub struct RavenBow;

impl WeaponTrait for RavenBow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RavenBow,
        internal_name: "Bow_Crowfeather",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM20),
        weapon_base: WeaponBaseATKFamily::ATK448,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "对处于水元素或火元素影响下的敌人，造成的伤害提高<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。",
            en: "Increases DMG against opponents affected by Hydro or Pyro by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "鸦羽弓",
            en: "Raven Bow"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::RavenBow { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(RavenBowEffect::new(rate)))
    }
}
