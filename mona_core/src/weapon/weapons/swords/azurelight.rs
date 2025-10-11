use crate::attribute::{AttributeName, Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::WeaponName;
use super::super::super::weapon_config::WeaponConfig;

pub struct AzurelightEffect {
    pub after_skill: bool,
    pub zero_energy: bool,
}

impl<T: Attribute> WeaponEffect<T> for AzurelightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        
        // 施放元素战技后12秒内，攻击力提升
        if self.after_skill {
            let mut total_atk_bonus = refine * 0.06 + 0.18; // R1: 24%, R5: 48%
            
            // 装备者的元素能量为0时，攻击力还会提升，且暴击伤害提升
            if self.zero_energy {
                total_atk_bonus += refine * 0.06 + 0.18; // R1: +24%, R5: +48% (额外)
                let crit_dmg_bonus = refine * 0.1 + 0.3; // R1: 40%, R5: 80%
                
                attribute.set_value_by(AttributeName::CriticalDamageBase, "苍耀被动：零能量暴击伤害", crit_dmg_bonus);
            }
            
            attribute.add_atk_percentage("苍耀被动：白山的馈赐", total_atk_bonus);
        }
    }
}

impl AzurelightEffect {
    pub fn new(config: &WeaponConfig) -> AzurelightEffect {
        let (after_skill, zero_energy) = match *config {
            WeaponConfig::Azurelight { after_skill, zero_energy } => (after_skill, zero_energy),
            _ => (false, false)
        };

        AzurelightEffect {
            after_skill,
            zero_energy,
        }
    }
}

pub struct Azurelight;

impl WeaponTrait for Azurelight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Azurelight,
        internal_name: "Sword_Azurelight",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技后的12秒内，攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。持续期间，装备者的元素能量为0时，攻击力还会提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>，且暴击伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "Within 12s after an Elemental Skill is used, ATK is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>. During this time, when the equipping character has 0 Energy, ATK will be further increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>, and CRIT DMG will be increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "苍耀",
            en: "Azurelight"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_skill",
            title: locale!(
                zh_cn: "施放元素战技后",
                en: "After Elemental Skill"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "zero_energy",
            title: locale!(
                zh_cn: "元素能量为0",
                en: "Zero Energy"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(AzurelightEffect::new(config)))
    }
}
