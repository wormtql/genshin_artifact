use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapons::prospectors_drill::ProspectorsDrillEffect;

pub struct RangeGauge;

impl WeaponTrait for RangeGauge {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RangeGauge,
        internal_name: "Bow_Mechanic",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "受到治疗或进行治疗时，将赋予一枚团结标记，持续30秒，至多拥有三枚团结标记。施放元素战技或元素爆发时，将消耗所有的团结标记，产生持续10秒的「抗争」效果：每枚消耗的团结标记提高<span style=\"color: #409EFF;\">3%-4%-5%-6%-7%</span>攻击力与<span style=\"color: #409EFF;\">7%-8.5%-10%-11.5%-13%</span>所有元素伤害加成。每15秒至多触发一次抗争效果；角色处于队伍后台时也能获得团结标记。",
            en: "When the wielder is healed or heals others, they will gain a Unity's Symbol that lasts 30s, up to a maximum of 3 Symbols. When using their Elemental Skill or Burst, all Symbols will be consumed and the Struggle effect will be granted for 10s. For each Symbol consumed, gain <span style=\"color: #409EFF;\">3%-4%-5%-6%-7%</span> ATK and <span style=\"color: #409EFF;\">7%-8.5%-10%-11.5%-13%</span> All Elemental DMG Bonus. The Struggle effect can be triggered once every 15s, and Symbols can be gained even when the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "测距规",
            en: "Range Gauge"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "团结标记数量",
                en: "Unity's Symbol Amount"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::RangeGauge { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(ProspectorsDrillEffect {
            stack
        }))
    }
}
