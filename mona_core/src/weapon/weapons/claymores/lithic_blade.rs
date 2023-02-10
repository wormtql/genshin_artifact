use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct LithicBladeEffect {
    liyue_count: usize
}

impl LithicBladeEffect {
    pub fn new(config: &WeaponConfig) -> LithicBladeEffect {
        match *config {
            WeaponConfig::LithicBlade { liyue_count } => LithicBladeEffect {
                liyue_count
            },
            _ => LithicBladeEffect {
                liyue_count: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LithicBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = (refine * 0.01 + 0.06) * self.liyue_count as f64;
        attribute.add_atk_percentage("千岩古剑被动", atk_bonus);
        let crit_bonus = (refine * 0.01 + 0.02) * self.liyue_count as f64;
        attribute.set_value_by(AttributeName::CriticalBase, "千岩古剑被动", crit_bonus);
    }
}

pub struct LithicBlade;

impl WeaponTrait for LithicBlade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LithicBlade,
        internal_name: "Claymore_Lapis",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "队伍中每有一位璃月角色，装备该武器的角色便获得<span style=\"color: #409EFF;\">7%-8%-9%-10%-11%</span>攻击力提升与<span style=\"color: #409EFF;\">3%-4%-5%-6%-7%</span>暴击率提升。至多获得4层提升效果。",
            en: "For every character in the party who hails from Liyue, the character who equips this weapon gains a <span style=\"color: #409EFF;\">7%-8%-9%-10%-11%</span> ATK increase and a <span style=\"color: #409EFF;\">3%-4%-5%-6%-7%</span> CRIT Rate increase. This effect stacks up to 4 times."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "千岩古剑",
            en: "Lithic Blade"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "liyue_count",
            title: locale!(
                zh_cn: "队伍璃月角色数量",
                en: "Team Liyue Character Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LithicBladeEffect::new(config)))
    }
}
