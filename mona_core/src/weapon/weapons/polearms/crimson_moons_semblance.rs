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

pub struct CrimsonMoonsSemblanceEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> WeaponEffect<A> for CrimsonMoonsSemblanceEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = refine * 0.04 + 0.08;
        attribute.set_value_by(AttributeName::BonusBase, "赤月之形被动", value * self.rate1 + 2.0 * value * self.rate2);
    }
}

pub struct CrimsonMoonsSemblance;

impl WeaponTrait for CrimsonMoonsSemblance {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CrimsonMoonsSemblance,
        internal_name: "",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "重击命中敌人时，赋予生命值上限25%的生命之契，该效果每14秒至多触发一次。此外，装备者具有生命之契时，造成的伤害提升<span style=\"color: #409EFF;\">12%-16%-20%-24%-28%</span>；若生命之契的数值大于等于生命值上限的30%，造成的伤害将进一步提升<span style=\"color: #409EFF;\">24%-32%-40%-48%-56%</span>。",
            en: "Grants a Bond of Life equal to 18% of Max HP when a Charged Attack hits an opponent. This effect can be triggered up to once every 14s. In addition, when the equipping character has a Bond of Life, they gain a <span style=\"color: #409EFF;\">12%-16%-20%-24%-28%</span> DMG Bonus; if the value of the Bond of Life is greater than or equal to 20% of Max HP, then gain an additional <span style=\"color: #409EFF;\">24%-32%-40%-48%-56%</span> DMG Bonus."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "赤月之形",
            en: "Crimson Moon's Semblance"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(zh_cn: "效果1比例", en: "Rate 1"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(zh_cn: "效果2比例", en: "Rate 2"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::CrimsonMoonsSemblance { rate1, rate2 } => {
                Some(Box::new(CrimsonMoonsSemblanceEffect {
                    rate1, rate2
                }))
            },
            _ => None
        }
    }
}
