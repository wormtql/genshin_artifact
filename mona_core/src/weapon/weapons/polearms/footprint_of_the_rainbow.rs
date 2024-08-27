use crate::attribute::{Attribute, AttributeCommon};
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

struct FootprintOfTheRainbowEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for FootprintOfTheRainbowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus = 0.12 + 0.04 * refine;
        attribute.add_def_percentage("虹的行迹被动", bonus * self.rate);
    }
}

pub struct FootprintOfTheRainbow;

impl WeaponTrait for FootprintOfTheRainbow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FootprintOfTheRainbow,
        internal_name: "Pole_Isikhulu",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF113),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技时，防御力提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，持续15秒。",
            en: "Using an Elemental Skill increases DEF by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "虹的行迹",
            en: "Footprint of the Rainbow"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::FootprintOfTheRainbow { rate } => Some(Box::new(FootprintOfTheRainbowEffect {
                rate
            })),
            _ => None
        }
    }
}
