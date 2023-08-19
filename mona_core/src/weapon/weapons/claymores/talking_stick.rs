use crate::attribute::{Attribute, AttributeCommon};
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

pub struct TalkingStickEffect {
    pub rate1: f64,
    pub rate2: f64
}

impl<A: Attribute> WeaponEffect<A> for TalkingStickEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("聊聊棒被动", (0.04 * refine + 0.12) * self.rate1);
        attribute.add_elemental_bonus("聊聊棒被动", (0.03 * refine + 0.09) * self.rate2);
    }
}

pub struct TalkingStick;

impl WeaponTrait for TalkingStick {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TalkingStick,
        internal_name: "Claymore_BeastTamer",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate40),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "承受火元素附着后的15秒内，攻击力提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，每12秒至多触发一次；承受水元素、冰元素、雷元素或草元素附着后的15秒内，所有元素伤害加成提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，每12秒至多触发一次。",
            en: "ATK will be increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 15s after being affected by Pyro. This effect can be triggered once every 12s. All Elemental DMG Bonus will be increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> for 15s after being affected by Hydro, Cryo, Electro, or Dendro. This effect can be triggered once every 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "聊聊棒",
            en: "Talking Stick"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "被动①比例",
                en: "Effect 1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "被动②比例",
                en: "Effect 2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2) = match *config {
            WeaponConfig::TalkingStick { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };
        Some(Box::new(TalkingStickEffect {
            rate1,
            rate2
        }))
    }
}
