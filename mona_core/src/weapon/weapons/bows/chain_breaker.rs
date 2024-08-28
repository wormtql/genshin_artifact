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

struct ChainBreakerEffect {
    pub count: usize,
}

impl<A: Attribute> WeaponEffect<A> for ChainBreakerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let step = 0.036 + 0.012 * refine;
        attribute.add_atk_percentage("碎链被动", step * self.count as f64);
        if self.count >= 3 {
            attribute.set_value_by(AttributeName::ElementalMastery, "碎链被动", 18.0 + 6.0 * refine);
        }
    }
}

pub struct ChainBreaker;

impl WeaponTrait for ChainBreaker {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ChainBreaker,
        internal_name: "Bow_Isikhulu",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "队伍中每有一名纳塔角色或与装备者元素类型不同的角色，装备者获得<span style=\"color: #409EFF;\">4.8%-6%-7.2%-8.4%-9.6%</span>攻击力提升；上述角色不少于3名时，装备者的元素精通提升<span style=\"color: #409EFF;\">24-30-36-42-48</span>点。",
            en: "For every party member from Natlan or who has a different Elemental Type from the equipping character, the equipping character gains <span style=\"color: #409EFF;\">4.8%-6%-7.2%-8.4%-9.6%</span> increased ATK. When there are no less than 3 of the aforementioned characters, the equipping character gains <span style=\"color: #409EFF;\">24-30-36-42-48</span> Elemental Mastery."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "碎链",
            en: "Chain Breaker"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "count",
            title: locale!(
                zh_cn: "纳塔角色或与装备者元素类型不同的角色数量",
                en: "Characters number from Natlan or who has a different Elemental Type from the equipping character"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 4 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::ChainBreaker { count } => Some(Box::new(ChainBreakerEffect {
                count
            })),
            _ => None
        }
    }
}
