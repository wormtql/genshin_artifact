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

pub struct WolfFangEffect {
    pub e_stack: f64,
    pub q_stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for WolfFangEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = 0.04 * refine + 0.12;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "狼牙被动", value);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "狼牙被动", value);

        if self.e_stack > 0.0 {
            let step = 0.005 * refine + 0.015;
            attribute.set_value_by(AttributeName::CriticalElementalSkill, "狼牙被动", step * self.e_stack);
        }
        if self.q_stack > 0.0 {
            let step = 0.005 * refine + 0.015;
            attribute.set_value_by(AttributeName::CriticalElementalBurst, "狼牙被动", step * self.q_stack);
        }
    }
}

pub struct WolfFang;

impl WeaponTrait for WolfFang {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WolfFang,
        internal_name: "Sword_Boreas",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate60),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素战技与元素爆发造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。元素战技命中敌人时，元素战技的暴击率提升<span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span>；元素爆发命中敌人时，元素爆发的暴击率提升<span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span>。上述两种效果各自持续10秒，至多叠加4次，每0.1秒至多触发一次。",
            en: "DMG dealt by Elemental Skill and Elemental Burst is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. When an Elemental Skill hits an opponent, its CRIT Rate will be increased by <span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span>. When an Elemental Burst hits an opponent, its CRIT Rate will be increased by <span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span>. Both of these effects last 10s separately, have 4 max stacks, and can be triggered once every 0.1s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "狼牙",
            en: "Wolf-Fang"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_stack",
            title: locale!(
                zh_cn: "元素战技命中层数",
                en: "E Hit Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        },
        ItemConfig {
            name: "q_stack",
            title: locale!(
                zh_cn: "元素爆发命中层数",
                en: "Q Hit Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (e_stack, q_stack) = match *config {
            WeaponConfig::WolfFang { e_stack, q_stack } => (e_stack, q_stack),
            _ => (0.0, 0.0)
        };
        Some(Box::new(WolfFangEffect {
            e_stack, q_stack
        }))
    }
}
