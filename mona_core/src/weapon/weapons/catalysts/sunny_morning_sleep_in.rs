use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

struct SunnyMorningSleepInEffect {
    pub rate1: f64,
    pub rate2: f64,
    pub rate3: f64,
}

impl<A: Attribute> WeaponEffect<A> for SunnyMorningSleepInEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus1 = 90.0 + 30.0 * refine;
        let bonus2 = 72.0 + 24.0 * refine;
        let bonus3 = 24.0 + 8.0 * refine;

        attribute.set_value_by(AttributeName::ElementalMastery, "寝正月初晴被动",
           bonus1 * self.rate1 + bonus2 * self.rate2 + bonus3 * self.rate3);
    }
}

pub struct SunnyMorningSleepIn;

impl WeaponTrait for SunnyMorningSleepIn {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SunnyMorningSleepIn,
        internal_name: "Catalyst_SakuraFan",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM58),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "触发扩散反应后的6秒内，元素精通提升<span style=\"color: #409EFF;\">120-150-180-210-240</span>点；元素战技命中敌人后的9秒内，元素精通提升<span style=\"color: #409EFF;\">96-120-144-168-192</span>点；元素爆发命中敌人后的30秒内，元素精通提升<span style=\"color: #409EFF;\">32-40-48-56-64</span>点。",
            en: "Elemental Mastery increases by <span style=\"color: #409EFF;\">120-150-180-210-240</span> for 6s after triggering Swirl. Elemental Mastery increases by <span style=\"color: #409EFF;\">96-120-144-168-192</span> for 9s after the wielder's Elemental Skill hits an opponent. Elemental Mastery increases by <span style=\"color: #409EFF;\">32-40-48-56-64</span> for 30s after the wielder's Elemental Burst hits an opponent."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "寝正月初晴",
            en: "Sunny Morning Sleep-In"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "效果1比例",
                en: "Effect 1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果2比例",
                en: "Effect 2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate3",
            title: locale!(
                zh_cn: "效果3比例",
                en: "Effect 3 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::SunnyMorningSleepIn { rate1, rate2, rate3 } => Some(
                Box::new(SunnyMorningSleepInEffect {
                    rate1, rate2, rate3
                })
            ),
            _ => None
        }
    }
}
