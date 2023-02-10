use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct ThunderingPulseEffect {
    stack: usize
}

impl ThunderingPulseEffect {
    pub fn new(config: &WeaponConfig) -> ThunderingPulseEffect {
        match *config {
            WeaponConfig::ThunderingPulse { stack } => ThunderingPulseEffect {
                stack
            },
            _ => ThunderingPulseEffect {
                stack: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ThunderingPulseEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("飞雷之弦振被动", refine * 0.05 + 0.15);
        let bonus = if self.stack == 1 {
            refine * 0.03 + 0.09
        } else if self.stack == 2 {
            refine * 0.06 + 0.18
        } else if self.stack == 3 {
            refine * 0.1 + 0.3
        } else {
            0.0
        };
        attribute.set_value_by(AttributeName::BonusNormalAttack, "飞雷之弦振被动", bonus);
    }
}

pub struct ThunderingPulse;

impl WeaponTrait for ThunderingPulse {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ThunderingPulse,
        internal_name: "Bow_Narukami",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>，并能获得「飞雷之巴印」的威势。飞雷之巴印：持有1/2/3层飞雷之巴印时，普通攻击造成的伤害提高<span style=\"color: #409EFF;\">12/24/40%-15/30/50%-18/36/60%-21/42/70%-24/48/80%</span>。在下列情况下，角色将各获得1层飞雷之巴印：普通攻击造成伤害时，持续5秒；施放元素战技时，持续10秒；此外，角色元素能量低于100%时，将获得1层飞雷之巴印，此飞雷之巴印会在角色的元素能量充满时消失。每层飞雷之巴印的持续时间独立计算。",
            en: "Increases ATK by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> and grants the might of the Thunder Emblem. At stack levels 1/2/3, the Thunder Emblem increases Normal Attack DMG by <span style=\"color: #409EFF;\">12/24/40%-15/30/50%-18/36/60%-21/42/70%-24/48/80%</span>. The character will obtain 1 stack of Thunder Emblem in each of the following scenarios: Normal Attack deals DMG (stack lasts 5s), casting Elemental Skill (stack lasts 10s); Energy is less than 100% (stack disappears when Energy is full). Each stack's duration is calculated independently."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "飞雷之弦振",
            en: "Thundering Pulse"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "「飞雷之巴印」层数",
                en: "「Thunder Emblem」Stack"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(ThunderingPulseEffect::new(config)))
    }
}
