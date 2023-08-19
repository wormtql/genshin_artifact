use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct FinaleOfTheDeepEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> WeaponEffect<A> for FinaleOfTheDeepEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let rate2 = self.rate2;

        attribute.add_atk_percentage("海渊终曲被动", (0.03 * refine + 0.09) * self.rate1);
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |hp, _| {
                let max = 37.5 * refine + 112.5;
                max.min(hp * 0.25 * (0.006 * refine + 0.018)) * rate2
            }),
            Box::new(|x, y, v| (0.0, 0.0)),
            "海渊终曲被动"
        );
    }
}

pub struct FinaleOfTheDeep;

impl WeaponTrait for FinaleOfTheDeep {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FinaleOfTheDeep,
        internal_name: "Sword_Vorpal",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技时，攻击力提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，持续15秒，并赋予生命值上限25%的生命之契，该效果每10秒至多触发一次。生命之契清除时，基于清除值的<span style=\"color: #409EFF;\">2.4%-3%-3.6%-4.2%-4.8%</span>提升至多<span style=\"color: #409EFF;\">150-187.5-225-262.5-300</span>点攻击力，持续15秒。",
            en: "When using an Elemental Skill, ATK will be increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> for 15s, and a Bond of Life worth 25% of Max HP will be granted. This effect can be triggered once every 10s. When the Bond of Life is cleared, a maximum of <span style=\"color: #409EFF;\">150-187.5-225-262.5-300</span> ATK will be gained based on <span style=\"color: #409EFF;\">2.4%-3%-3.6%-4.2%-4.8%</span> of the total amount of the Life Bond cleared, lasting for 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "海渊终曲",
            en: "Finale of the Deep"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "效果①比例",
                en: "Effect 1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果②比例",
                en: "Effect 2 Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2) = match *config {
            WeaponConfig::FinaleOfTheDeep { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };
        Some(Box::new(FinaleOfTheDeepEffect {
            rate1, rate2
        }))
    }
}
