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

pub struct VortexVanquisherEffect {
    stack: f64,
    shield_rate: f64
}

impl VortexVanquisherEffect {
    pub fn new(config: &WeaponConfig) -> VortexVanquisherEffect {
        match *config {
            WeaponConfig::VortexVanquisher { stack, shield_rate } => VortexVanquisherEffect {
                stack,
                shield_rate
            },
            _ => VortexVanquisherEffect {
                stack: 0.0,
                shield_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for VortexVanquisherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::ShieldStrength, "贯虹之槊被动", refine * 0.05 + 0.15);
        let atk_bonus = (refine * 0.01 + 0.03) * self.stack * (1.0 + self.shield_rate);
        attribute.add_atk_percentage("贯虹之槊被动等效", atk_bonus);
    }
}

pub struct VortexVanquisher;

impl WeaponTrait for VortexVanquisher {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::VortexVanquisher,
        internal_name: "Pole_Kunwu",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "护盾强效提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。攻击命中后的8秒内，攻击力提升<span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span>。该效果至多可叠加5层，每0.3秒只能触发一次。此外，处于护盾庇护下时，该效果的攻击力提升效果提高100%。",
            en: "Increases Shield Strength by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Scoring hits on opponents increases ATK by <span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span> for 8s. Max 5 stacks. Can only occur once every 0.3s. While protected by a shield, this ATK increase effect is increased by 100%."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "贯虹之槊",
            en: "Vortex Vanquisher"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float {
                min: 0.0,
                max: 5.0,
                default: 0.0
            }
        },
        ItemConfig {
            name: "shield_rate",
            title: locale!(
                zh_cn: "护盾覆盖率",
                en: "Shield Rate"
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(VortexVanquisherEffect::new(config)))
    }
}
