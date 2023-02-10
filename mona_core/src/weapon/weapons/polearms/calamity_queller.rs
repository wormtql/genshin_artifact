use crate::attribute::{Attribute, AttributeCommon};
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

pub struct CalamityQuellerEffect {
    stack: f64,
    backend_rate: f64
}

impl CalamityQuellerEffect {
    pub fn new(config: &WeaponConfig) -> CalamityQuellerEffect {
        match *config {
            WeaponConfig::CalamityQueller { stack, backend_rate } => CalamityQuellerEffect {
                stack,
                backend_rate
            },
            _ => CalamityQuellerEffect {
                stack: 0.0,
                backend_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for CalamityQuellerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let elemental_bonus = refine * 0.03 + 0.09;
        attribute.add_elemental_bonus("息灾被动", elemental_bonus);

        let atk_bonus = (refine * 0.008 + 0.024) * self.stack * (1.0 + self.backend_rate);
        attribute.add_atk_percentage("息灾被动等效", atk_bonus);
    }
}

pub struct CalamityQueller;

impl WeaponTrait for CalamityQueller {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CalamityQueller,
        internal_name: "Pole_Santika",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK36),
        weapon_base: WeaponBaseATKFamily::ATK741,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "获得<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>所有元素伤害加成；施放元素战技后，获得持续20秒的「圆顿」，攻击力每1秒提升<span style=\"color: #409EFF;\">3.2%-4%-4.8%-5.6%-6.4%</span>，该攻击力提升效果至多叠加6次。当装备此武器的角色处于队伍后台时，「圆顿」的攻击力提升效果翻倍。",
            en: "Gain <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> All Elemental DMG Bonus. Obtain Consummation for 20s after using an Elemental Skill, causing ATK to increase by <span style=\"color: #409EFF;\">3.2%-4%-4.8%-5.6%-6.4%</span> per second. This ATK increase has a maximum of 6 stacks. When the character equipped with this weapon is not on the field, Consummation's ATK increase is doubled."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "息灾",
            en: "Calamity Queller"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "「圆顿」等效层数",
                en: "「Consummation」Avg Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 6.0, default: 6.0 }
        },
        ItemConfig {
            name: "backend_rate",
            title: locale!(
                zh_cn: "后台比例",
                en: "Backend Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(CalamityQuellerEffect::new(config)))
    }
}
