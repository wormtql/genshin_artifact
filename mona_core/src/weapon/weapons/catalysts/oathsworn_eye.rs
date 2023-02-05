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

pub struct OathswornEyeEffect {
    pub rate: f64
}

impl<A: Attribute> WeaponEffect<A> for OathswornEyeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let recharge_bonus = (refine * 0.06 + 0.18) * self.rate;
        attribute.set_value_by(AttributeName::Recharge, "证誓之明瞳被动等效", recharge_bonus);
    }
}

pub struct OathswornEye;

impl WeaponTrait for OathswornEye {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::OathswornEye,
        internal_name: "Catalyst_Jyanome",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技后，元素充能效率提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>，持续10秒。",
            en: "Increases Energy Recharge by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 10s after using an Elemental Skill."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "证誓之明瞳",
            en: "Oathsworn Eye"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::OathswornEye { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(OathswornEyeEffect {
            rate
        }))
    }
}
