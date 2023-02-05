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

pub struct FerrousShadowEffect {
    pub rate: f64
}

impl FerrousShadowEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for FerrousShadowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.05 + 0.25) * self.rate;
        attribute.set_value_by(AttributeName::BonusChargedAttack, "铁影阔剑被动等效", value);
    }
}

pub struct FerrousShadow;

impl WeaponTrait for FerrousShadow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FerrousShadow,
        internal_name: "Claymore_Glaive",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "生命值低于<span style=\"color: #409EFF;\">70%-75%-80%-85%-90%</span>时，重击不会轻易被打断，并提高<span style=\"color: #409EFF;\">30%-35%-40%-45%-50%</span>重击伤害。",
            en: "When HP falls below <span style=\"color: #409EFF;\">70%-75%-80%-85%-90%</span>, increases Charged Attack DMG by <span style=\"color: #409EFF;\">30%-35%-40%-45%-50%</span> and Charged Attacks become harder to interrupt."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "铁影阔剑",
            en: "Ferrous Shadow"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::FerrousShadow { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(FerrousShadowEffect::new(rate)))
    }
}
