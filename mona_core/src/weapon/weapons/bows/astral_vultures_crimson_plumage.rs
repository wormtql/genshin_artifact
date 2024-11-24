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

struct AstralVulturesCrimsonPlumageEffect {
    rate: f64,
    different_count: usize,
}

impl<A: Attribute> WeaponEffect<A> for AstralVulturesCrimsonPlumageEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let atk_bonus = 0.18 + 0.06 * refine;
        attribute.add_atk_percentage("星鹫赤羽被动", atk_bonus * self.rate);

        let charged_bonus = if self.different_count == 0 {
            0.0
        } else if self.different_count == 1 {
            0.15 + 0.05 * refine
        } else {
            0.36 + 0.12 * refine
        };
        attribute.set_value_by(AttributeName::BonusChargedAttack, "星鹫赤羽被动", charged_bonus * self.rate);

        let q_bonus = if self.different_count == 0 {
            0.0
        } else if self.different_count == 1 {
            0.075 + 0.025 * refine
        } else {
            0.18 + 0.06 * refine
        };
        attribute.set_value_by(AttributeName::BonusElementalBurst, "星鹫赤羽被动", q_bonus * self.rate);
    }
}

pub struct AstralVulturesCrimsonPlumage;

impl WeaponTrait for AstralVulturesCrimsonPlumage {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AstralVulturesCrimsonPlumage,
        internal_name: "Bow_Qoyllorsnova",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "触发扩散反应后的12秒内，攻击力提高<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。此外，队伍中存在至少1/2名与装备者元素类型不同的角色时，装备者重击造成的伤害提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>/<span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span>，元素爆发造成的伤害提高<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>/<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。",
            en: "For 12s after triggering a Swirl reaction, ATK increases by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>. In addition, when 1/2 or more characters in the party are of a different Elemental Type from the equipping character, the DMG dealt by the equipping character's Charged Attacks is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>/<span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span> and Elemental Burst DMG dealt is increased by <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>/<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "星鹫赤羽",
            en: "Astral Vulture’s Crimson Plumage"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
        ItemConfig {
            name: "different_count",
            title: locale!(
                zh_cn: "不同元素角色数量",
                en: "# Different Element"
            ),
            config: ItemConfigType::Int { min: 0, max: 2, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::AstralVulturesCrimsonPlumage { rate, different_count } => Some(
                Box::new(AstralVulturesCrimsonPlumageEffect {
                    rate, different_count
                })
            ),
            _ => None
        }
    }
}
