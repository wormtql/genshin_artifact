use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::attribute::AttributeName::BonusHydro;
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

pub struct FlowingPurityEffect {
    pub rate1: f64,
    pub rate2: f64
}

impl<A: Attribute> WeaponEffect<A> for FlowingPurityEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_elemental_bonus("纯水流华被动", self.rate1 * (0.02 * refine + 0.06));

        let bonus_elements = [
            AttributeName::BonusHydro,
            AttributeName::BonusPhysical,
            AttributeName::BonusElectro,
            AttributeName::BonusDendro,
            AttributeName::BonusPyro,
            AttributeName::BonusGeo,
            AttributeName::BonusCryo,
            AttributeName::BonusAnemo
        ];

        let rate2 = self.rate2;
        for target in bonus_elements {
            attribute.add_edge1(
                AttributeName::HP,
                target,
                Box::new(move |hp, _| {
                    let max = 0.03 * refine + 0.09;
                    let step = 0.005 * refine + 0.015;
                    let value = max.min((hp / 1000.0).floor() * step);
                    value * rate2
                }),
                Box::new(|gx, gy, gz| (0.0, 0.0)),
                "纯水流华被动"
            )
        }
    }
}

pub struct FlowingPurity;

impl WeaponTrait for FlowingPurity {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FlowingPurity,
        internal_name: "Catalyst_Vorpal",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技时，所有元素伤害加成提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，持续15秒，并赋予生命值上限24%的生命之契，该效果每10秒至多触发一次。生命之契清除时，每清除1000点将会提供<span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span>所有元素伤害加成，至多通过这种方式获得<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>所有元素伤害加成，持续15秒。",
            en: "When using an Elemental Skill, All Elemental DMG Bonus will be increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> for 15s, and a Bond of Life worth 24% of Max HP will be granted. This effect can be triggered once every 10s. When the Bond of Life is cleared, every 1,000 HP cleared in the process will provide <span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span> All Elemental DMG Bonus, up to a maximum of <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. This effect lasts 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "纯水流华",
            en: "Flowing Purity"
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
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 },
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果②比例",
                en: "Effect 2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2) = match *config {
            WeaponConfig::FlowingPurity { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };
        Some(Box::new(FlowingPurityEffect {
            rate1, rate2
        }))
    }
}
