use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct PrimordialJadeWingedSpearEffect {
    stack: f64,
    full_rate: f64
}

impl PrimordialJadeWingedSpearEffect {
    pub fn new(config: &WeaponConfig) -> PrimordialJadeWingedSpearEffect {
        match *config {
            WeaponConfig::PrimordialJadeWingedSpear { stack, full_rate } => PrimordialJadeWingedSpearEffect {
                stack,
                full_rate
            },
            _ => PrimordialJadeWingedSpearEffect {
                stack: 0.0,
                full_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrimordialJadeWingedSpearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = (refine * 0.007 + 0.025) * self.stack;
        attribute.add_atk_percentage("和璞鸢被动等效", atk_bonus);
        let bonus = (refine * 0.03 + 0.09) * self.full_rate;
        attribute.set_value_by(AttributeName::BonusBase, "和璞鸢被动等效", bonus);
    }
}

pub struct PrimordialJadeWingedSpear;

impl WeaponTrait for PrimordialJadeWingedSpear {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrimordialJadeWingedSpear,
        internal_name: "Pole_Morax",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "命中敌人时自身攻击力提高<span style=\"color: #409EFF;\">3.2%-3.9%-4.6%-5.3%-6%</span>，持续6秒，最高可以叠加7层。该效果每0.3秒最多触发一次。满层状态时伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。",
            en: "On hit, increases ATK by <span style=\"color: #409EFF;\">3.2%-3.9%-4.6%-5.3%-6%</span> for 6s. Max 7 stacks. This effect can only occur once every 0.3s. While in possession of the maximum possible stacks, DMG dealt is increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "和璞鸢",
            en: "Primordial Jade Winged-Spear"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float {
                min: 0.0, max: 7.0, default: 0.0
            }
        },
        ItemConfig {
            name: "full_rate",
            title: locale!(
                zh_cn: "满层状态比例",
                en: "Full Stack Ratio",
            ),
            config: ItemConfig::RATE01_TYPE,
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PrimordialJadeWingedSpearEffect::new(config)))
    }
}
