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

struct VividNotionsEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> WeaponEffect<A> for VividNotionsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let atk_bonus = 0.07 * refine + 0.21;
        attribute.add_atk_percentage("溢彩心念被动", atk_bonus);

        // happened to be same
        let bonus1 = atk_bonus;
        attribute.set_value_by(AttributeName::CriticalDamagePlungingAttack, "溢彩心念被动1", bonus1 * self.rate1);

        let bonus2 = 0.3 + 0.1 * refine;
        attribute.set_value_by(AttributeName::CriticalDamagePlungingAttack, "溢彩心念被动2", bonus2 * self.rate2);
    }
}

pub struct VividNotions;

impl WeaponTrait for VividNotions {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::VividNotions,
        internal_name: "Catalyst_VaresaTransformer",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击力提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>；进行下落攻击时，获得「初霞之彩」效果：下落攻击造成的暴击伤害提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>；施放元素战技或元素爆发时，获得「苍暮之辉」效果：下落攻击造成的暴击伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>；上述两种效果各自持续15秒，并将在坠地冲击命中后的0.1秒后解除。",
            en: "ATK is increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>. When you use a Plunging Attack, you will gain the \"Dawn's First Hue\" effect: Plunging Attack CRIT DMG is increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>. When you use an Elemental Skill or Burst, you will gain the \"Twilight's Splendor\" effect: Plunging Attack CRIT DMG is increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>. The two effects above each last for 15s, and will be canceled 0.1s after the ground impact hits a target."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "溢彩心念",
            en: "Vivid Notions"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "被动1比例",
                en: "Effect 1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "被动2比例",
                en: "Effect 2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match config {
            &WeaponConfig::VividNotions { rate1, rate2 } => Some(Box::new(
                VividNotionsEffect {
                    rate1, rate2
                }
            )),
            _ => None
        }
    }
}
