use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SacrificialJadeEffect {
    pub rate: f64
}

impl<A: Attribute> WeaponEffect<A> for SacrificialJadeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_hp_percentage("遗祀玉珑被动", self.rate * (0.08 * refine + 0.24));
        attribute.set_value_by(AttributeName::ElementalMastery, "遗祀玉珑被动", self.rate * (10.0 * refine + 30.0));
    }
}

pub struct SacrificialJade;

impl WeaponTrait for SacrificialJade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SacrificialJade,
        internal_name: "Catalyst_Yue",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate80),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "处于队伍后台超过5秒后，生命值上限提升<span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span>，元素精通提升<span style=\"color: #409EFF;\">40-50-60-70-80</span>点。装备者登场并留在场上10秒后，该效果将失效。",
            en: "When not on the field for more than 5s, Max HP will be increased by <span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span> and Elemental Mastery will be increased by <span style=\"color: #409EFF;\">40-50-60-70-80</span>. These effects will be canceled after the wielder has been on the field for 10s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "遗祀玉珑",
            en: "Sacrificial Jade"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::SacrificialJade { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(SacrificialJadeEffect {
            rate
        }))
    }
}
