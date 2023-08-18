use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct TheFirstGreatMagicEffect {
    pub diff_count: f64,
    pub same_count: f64,
}

impl<A: Attribute> WeaponEffect<A> for TheFirstGreatMagicEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::BonusChargedAttack, "最初的大魔术被动", 0.04 * refine + 0.12);
        if self.diff_count > 0.0 {
            let interval = 0.04 * refine + 0.12;
            attribute.add_atk_percentage("最初的大魔术被动", self.same_count * interval);
        }
    }
}

pub struct TheFirstGreatMagic;

impl WeaponTrait for TheFirstGreatMagic {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheFirstGreatMagic,
        internal_name: "Bow_Pledge",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "重击造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>；队伍中每存在一位与装备者元素类型相同的角色（包括装备者自身），将获得1层「手法」效果；每存在一位元素类型不同的角色，将获得1层「演技」效果。处于1/2/3层及以上「手法」效果下时，攻击力提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>/<span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span>/<span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span>；处于1/2/3层及以上「演技」效果下时，移动速度提升<span style=\"color: #409EFF;\">4%-6%-8%-10%-12%</span>/<span style=\"color: #409EFF;\">7%-9%-11%-13%-15%</span>/<span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>。",
            en: "DMG dealt by Charged Attacks increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. For every party member with the same Elemental Type as the wielder (including the wielder themselves), gain 1 Gimmick stack. For every party member with a different Elemental Type from the wielder, gain 1 Theatrics stack. When the wielder has 1/2/3 or more Gimmick stacks, ATK will be increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>/<span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span>/<span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span>. When the wielder has 1/2/3 or more Theatrics stacks, Movement SPD will be increased by <span style=\"color: #409EFF;\">4%-6%-8%-10%-12%</span>/<span style=\"color: #409EFF;\">7%-9%-11%-13%-15%</span>/<span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "最初的大魔术",
            en: "The First Great Magic"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "same_count",
            title: locale!(
                zh_cn: "队伍同元素数量",
                en: "Same Element Number"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 1.0 }
        },
        ItemConfig {
            name: "diff_count",
            title: locale!(
                zh_cn: "队伍不同元素数量",
                en: "Diff Element Number"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (a, b) = match *config {
            WeaponConfig::TheFirstGreatMagic { diff_count, same_count } => (diff_count, same_count),
            _ => (0.0, 0.0)
        };
        Some(Box::new(TheFirstGreatMagicEffect {
            diff_count: a,
            same_count: b
        }))
    }
}
