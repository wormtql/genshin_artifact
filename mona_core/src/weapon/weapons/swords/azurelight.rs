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

struct AzurelightEffect {
    pub after_e: bool,
    pub effect_2: bool,
}

impl<A: Attribute> WeaponEffect<A> for AzurelightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        if self.after_e {
            let bonus = 0.18 + 0.06 * refine;
            attribute.add_atk_percentage("苍耀被动1", bonus);

            if self.effect_2 {
                attribute.add_atk_percentage("苍耀被动2", bonus);
                let crit_dmg_enhance = 0.3 + 0.1 * refine;
                attribute.set_value_by(AttributeName::CriticalDamageBase, "苍耀被动2", crit_dmg_enhance);
            }
        }
    }
}

pub struct Azurelight;

impl WeaponTrait for Azurelight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Azurelight,
        internal_name: "Azurelight",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技后的12秒内, 攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>; 持续期间, 装备者的元素能量为0时, 攻击力还会提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>, 且暴击伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
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
            name: "after_e",
            title: locale!(
                zh_cn: "释放元素战技后",
                en: "After Casting Elemental Skill"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "effect_2",
            title: locale!(
                zh_cn: "元素能量为0",
                en: "0 Energy"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match config {
            &WeaponConfig::Azurelight { after_e, effect_2 } => Some(Box::new(
                AzurelightEffect {
                    after_e, effect_2
                }
            )),
            _ => None
        }
    }
}