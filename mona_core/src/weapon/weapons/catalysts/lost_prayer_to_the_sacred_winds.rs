use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct LostPrayerToTheSacredWindsEffect {
    stack: f64
}

impl LostPrayerToTheSacredWindsEffect {
    pub fn new(config: &WeaponConfig) -> LostPrayerToTheSacredWindsEffect {
        match *config {
            WeaponConfig::LostPrayerToTheSacredWinds { stack } => LostPrayerToTheSacredWindsEffect {
                stack
            },
            _ => LostPrayerToTheSacredWindsEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LostPrayerToTheSacredWindsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.02 + 0.06) * self.stack;
        attribute.add_elemental_bonus("四风原典被动等效", value);
    }
}

pub struct LostPrayerToTheSacredWinds;

impl WeaponTrait for LostPrayerToTheSacredWinds {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LostPrayerToTheSacredWinds,
        internal_name: "Catalyst_Fourwinds",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "移动速度提高10%；在场上每4秒获得<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>元素伤害加成。该效果最多叠加4层，持续直到角色倒下或离场。",
            en: "Increases Movement SPD by 10%. When in battle, gain an <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> Elemental DMG Bonus every 4s. Max 4 stacks. Lasts until the character falls or leaves combat."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "四风原典",
            en: "Lost Prayer to the Sacred Winds"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK04
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LostPrayerToTheSacredWindsEffect::new(config)))
    }
}
