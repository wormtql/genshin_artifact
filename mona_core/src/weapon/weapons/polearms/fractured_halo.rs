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

pub struct FracturedHaloEffect {
    pub after_skill_burst: bool,
    pub has_shield: bool,
}

impl<A: Attribute> WeaponEffect<A> for FracturedHaloEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        
        // 施放元素战技或元素爆发后的20秒内，攻击力提升
        if self.after_skill_burst {
            let atk_bonus = refine * 0.06 + 0.18; // R1: 24%, R5: 48%
            attribute.add_atk_percentage("支离轮光被动：攻击力提升", atk_bonus);
        }
        
        // 若装备者创造了护盾，获得「流电圣敕」效果：月感电反应伤害提升
        if self.has_shield {
            let moonelectro_bonus = refine * 0.1 + 0.3; // R1: 40%, R5: 80%
            attribute.set_value_by(AttributeName::EnhanceMoonelectro, "支离轮光：流电圣敕", moonelectro_bonus);
        }
    }
}

pub struct FracturedHalo;

impl WeaponTrait for FracturedHalo {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FracturedHalo,
        internal_name: "FracturedHalo",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技或元素爆发后的20秒内，攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。持续期间内，若装备者创造了护盾，则接下来的20秒内，还会获得「流电圣敕」效果：队伍中附近所有角色触发的月感电反应造成的伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "After using an Elemental Skill or Elemental Burst, ATK is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 20s. During this period, if the wielder creates a shield, all nearby party members will gain the \"Lightning Commandment\" effect for the next 20s: all Lunar-Charged Reaction DMG dealt by nearby party members is increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "支离轮光",
            en: "Fractured Halo"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_skill_burst",
            title: locale!(zh_cn: "施放元素战技或元素爆发后", en: "After Elemental Skill/Burst"),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "has_shield",
            title: locale!(zh_cn: "装备者创造了护盾", en: "Wielder Creates Shield"),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::FracturedHalo { after_skill_burst, has_shield } => {
                Some(Box::new(FracturedHaloEffect {
                    after_skill_burst, 
                    has_shield
                }))
            },
            _ => None
        }
    }
}
