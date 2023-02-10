use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct BlackcliffLongswordEffect {
    stack: f64,
}

impl BlackcliffLongswordEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffLongswordEffect {
        match *config {
            WeaponConfig::BlackcliffLongsword { stack } => BlackcliffLongswordEffect {
                stack
            },
            _ => BlackcliffLongswordEffect {
                stack: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffLongswordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("黑岩长剑被动等效", value);
    }
}

pub struct BlackcliffLongsword;

impl WeaponTrait for BlackcliffLongsword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BlackcliffLongsword,
        internal_name: "Sword_Blackrock",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage80),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "击败敌人后，攻击力提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，持续30秒。该效果至多叠加3层，每层持续时间独立。",
            en: "After defeating an opponent, ATK is increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> for 30s. This effect has a maximum of 3 stacks, and the duration of each stack is independent of the others."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "黑岩长剑",
            en: "Blackcliff Longsword"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float {
                min: 0.0,
                max: 3.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BlackcliffLongswordEffect::new(config)))
    }
}
