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

pub struct TheDockhandsAssistantEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for TheDockhandsAssistantEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = 30.0 + refine * 10.0;
        attribute.set_value_by(AttributeName::ElementalMastery, "船坞长剑被动", value * self.stack);
    }
}

pub struct TheDockhandsAssistant;

impl WeaponTrait for TheDockhandsAssistant {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheDockhandsAssistant,
        internal_name: "Sword_Mechanic",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(
            locale!(
                zh_cn: "受到治疗或进行治疗时，将赋予一枚坚忍标记，持续30秒，至多拥有三枚坚忍标记。施放元素战技或元素爆发时，将消耗所有的坚忍标记，产生持续10秒的「奋起」效果：每枚消耗的坚忍标记提高<span style=\"color: #409EFF;\">40-50-60-70-80</span>点元素精通，并在效果产生2秒后，每枚消耗的坚忍标记为装备者恢复<span style=\"color: #409EFF;\">2-2.5-3-3.5-4</span>点元素能量。每15秒至多触发一次奋起效果；角色处于队伍后台时也能获得坚忍标记。",
                en: "When the wielder is healed or heals others, they will gain a Stoic's Symbol that lasts 30s, up to a maximum of 3 Symbols. When using their Elemental Skill or Burst, all Symbols will be consumed and the Roused effect will be granted for 10s. For each Symbol consumed, gain <span style=\"color: #409EFF;\">40-50-60-70-80</span> Elemental Mastery, and 2s after the effect occurs, <span style=\"color: #409EFF;\">2-2.5-3-3.5-4</span> Energy per Symbol consumed will be restored for said character. The Roused effect can be triggered once every 15s, and Symbols can be gained even when the character is not on the field."
            )
        ),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "船坞长剑",
            en: "The Dockhand’s Assistant"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "坚忍标记数量",
                en: "Stoic's Symbol Amount"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::TheDockhandsAssistant { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(TheDockhandsAssistantEffect {
            stack
        }))
    }
}
