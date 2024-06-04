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

pub struct SilvershowerHeartstringsEffect {
    pub stack3_rate: f64,
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for SilvershowerHeartstringsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let stack1 = 0.09 + refine * 0.03;
        let stack2 = 0.18 + refine * 0.06;
        let stack3 = 0.3 + refine * 0.1;

        let value = if self.stack < 1.0 {
            self.stack * stack1
        } else if self.stack < 2.0 {
            stack1 + (stack2 - stack1) * (self.stack - 1.0)
        } else {
            stack2 + (stack3 - stack2) * (self.stack - 2.0)
        };
        attribute.add_hp_percentage("白雨心弦被动", value);

        attribute.set_value_by(AttributeName::CriticalElementalBurst, "白雨心弦被动", 0.21 + refine * 0.07);
    }
}

pub struct SilvershowerHeartstrings;

impl WeaponTrait for SilvershowerHeartstrings {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SilvershowerHeartstrings,
        internal_name: "SilvershowerHeartstrings",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP144),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "装备者能获得「疗护」效果，持有1/2/3层疗护时，生命值上限提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>/<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>/<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。在下列情况下，装备者将各获得1层疗护：施放元素战技时，持续25秒；生命之契的数值增加时，持续25秒；进行治疗时，持续20秒，装备者处于队伍后台时依然能触发。每层疗护的持续时间独立计算。此外，处于3层疗护状态下时，元素爆发的暴击率提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，该效果将在疗护不足3层4秒后移除。",
            en: "The equipping character can gain the Remedy effect. When they possess 1/2/3 Remedy stacks, Max HP will increase by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>/<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>/<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>. 1 stack may be gained when the following conditions are met: 1 stack for 25s when using an Elemental Skill; 1 stack for 25s when the value of a Bond of Life value increased; 1 stack for 20s for performing healing. Stacks can still be triggered when the equipping character is not on the field. Each stack's duration is counted independently. In addition, when 3 stacks are active, Elemental Burst CRIT Rate will be increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>. This effect will be canceled 4s after falling under 3 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "白雨心弦",
            en: "Silvershower Heartstrings",
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 },
        },
        ItemConfig {
            name: "stack3_rate",
            title: locale!(
                zh_cn: "3层效果比例",
                en: "Stack-3 Portion"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (stack, stack3_rate) = match *config {
            WeaponConfig::SilvershowerHeartstrings { stack, stack3_rate } => (stack, stack3_rate),
            _ => (0.0, 0.0)
        };
        if stack > 0.0 && stack3_rate > 0.0 {
            Some(Box::new(SilvershowerHeartstringsEffect {
                stack, stack3_rate
            }))
        } else {
            None
        }
    }
}
