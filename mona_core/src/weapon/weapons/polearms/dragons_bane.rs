use crate::attribute::{Attribute, AttributeName};
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

pub struct DragonsBaneEffect {
    rate: f64
}

impl DragonsBaneEffect {
    pub fn new(config: &WeaponConfig) -> DragonsBaneEffect {
        match *config {
            WeaponConfig::DragonsBane { rate } => DragonsBaneEffect {
                rate
            },
            _ => DragonsBaneEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DragonsBaneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "匣里灭辰被动等效", value);
    }
}

pub struct DragonsBane;

impl WeaponTrait for DragonsBane {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DragonsBane,
        internal_name: "Pole_Stardust",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM48),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "对处于水元素或火元素影响下的敌人，造成的伤害提高<span style=\"color: #409EFF;\">20%-24%-28%-32%-36%</span>。",
            en: "Increases DMG against opponents affected by Hydro or Pyro by <span style=\"color: #409EFF;\">20%-24%-28%-32%-36%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "匣里灭辰",
            en: "Dragon's Bane"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(DragonsBaneEffect::new(config)))
    }
}
