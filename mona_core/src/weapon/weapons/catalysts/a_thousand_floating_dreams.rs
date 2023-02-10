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

pub struct AThousandFloatingDreamsEffect {
    pub same_count: usize,
    pub diff_count: usize,
}

impl<A: Attribute> WeaponEffect<A> for AThousandFloatingDreamsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let em_delta = refine * 8.0 + 24.0;
        let bonus_delta = refine * 0.04 + 0.06;

        attribute.set_value_by(AttributeName::ElementalMastery, "千夜浮梦被动", em_delta * self.same_count.min(3) as f64);
        attribute.add_elemental_bonus("千夜浮梦被动", bonus_delta * self.diff_count.min(3) as f64);
    }
}

pub struct AThousandFloatingDreams;

impl WeaponTrait for AThousandFloatingDreams {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AThousandFloatingDreams,
        internal_name: "Catalyst_Ayus",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM58),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "队伍中每个其他角色，都会依据元素类型与装备者相同与否，为装备者提供提升效果。相同：元素精通提升<span style=\"color: #409EFF;\">32-40-48-56-64</span>点；不同：装备者元素类型的元素伤害加成提升<span style=\"color: #409EFF;\">10%-14%-18%-22%-26%</span>。上述提升效果每种至多叠加3层。此外，队伍中装备者以外的附近角色的元素精通提升<span style=\"color: #409EFF;\">40-42-44-46-48</span>点，多件同名武器产生的此效果可以叠加。",
            en: "Party members other than the equipping character will provide the equipping character with buffs based on whether their Elemental Type is the same as the latter or not. If their Elemental Types are the same, increase Elemental Mastery by <span style=\"color: #409EFF;\">32-40-48-56-64</span>. If not, increase the equipping character’s DMG Bonus from their Elemental Type by <span style=\"color: #409EFF;\">10%-14%-18%-22%-26%</span>. Each of the aforementioned effects can have up to 3 stacks. Additionally, all nearby party members other than the equipping character will have their Elemental Mastery increased by <span style=\"color: #409EFF;\">40-42-44-46-48</span>. Multiple such effects from multiple such weapons can stack."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "千夜浮梦",
            en: "A Thousand Floating Dreams"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "same_count",
            title: locale!(
                zh_cn: "同元素角色数",
                en: "Same Element Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 1 },
        },
        ItemConfig {
            name: "diff_count",
            title: locale!(
                zh_cn: "不同元素角色数",
                en: "Diff Element Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 2 },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (same_count, diff_count) = match *config {
            WeaponConfig::AThousandFloatingDreams { same_count, diff_count } => (same_count, diff_count),
            _ => (0, 0)
        };

        Some(Box::new(AThousandFloatingDreamsEffect {
            same_count,
            diff_count,
        }))
    }
}