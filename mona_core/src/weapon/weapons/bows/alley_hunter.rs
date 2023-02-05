use smallvec::{smallvec, SmallVec};
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{WeaponType};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct AlleyHunterEffect {
    pub stack: f64
}

impl AlleyHunterEffect {
    pub fn new(config: &WeaponConfig) -> AlleyHunterEffect {
        match *config {
            WeaponConfig::AlleyHunter { stack } => AlleyHunterEffect {
                stack
            },
            _ => AlleyHunterEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for AlleyHunterEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.005 + 0.015) * self.stack;
        attribute.set_value_by(AttributeName::BonusBase, "暗巷猎手被动等效", value);
    }
}

pub struct AlleyHunter;

impl WeaponTrait for AlleyHunter {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AlleyHunter,
        internal_name: "Bow_Outlaw",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "街巷伏击：装备该武器的角色处于队伍后台时，每1秒角色造成的伤害提升<span style=\"color: #409EFF;\">2%/2.5%/3%/3.5%/4%</span>,最多通过这种方式获得<span style=\"color: #409EFF;\">20%/25%/30%/35%/40%</span>的伤害提升；在场上超过4秒后，上述伤害提升效果每1秒会流失<span style=\"color: #409EFF;\">4%/5%/6%/7%/8%</span>，直到降低至0%。",
            en: "While the character equipped with this weapon is in the party but not on the field, their DMG increases by <span style=\"color: #409EFF;\">2%-2.5%-3%-3.5%-4%</span> every second up to a max of <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. When the character is on the field for more than 4s, the aforementioned DMG buff decreases by <span style=\"color: #409EFF;\">4%-5%-6%-7%-8%</span> per second until it reaches 0%.",
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "暗巷猎手",
            en: "Alley Hunter",
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            config: ItemConfigType::Float {
                min: 0.0,
                max: 10.0,
                default: 0.0
            },
            title: ItemConfig::DEFAULT_STACK_TITLE,
            name: "stack"
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(AlleyHunterEffect::new(config)))
    }
}
