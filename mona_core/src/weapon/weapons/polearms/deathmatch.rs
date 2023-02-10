use crate::attribute::{Attribute, AttributeCommon};
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

pub struct DeathmatchEffect {
    ge2: bool
}

impl DeathmatchEffect {
    pub fn new(config: &WeaponConfig) -> DeathmatchEffect {
        match *config {
            WeaponConfig::Deathmatch { ge2 } => DeathmatchEffect {
                ge2
            },
            _ => DeathmatchEffect {
                ge2: false
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DeathmatchEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        if self.ge2 {
            let value = refine * 0.04 + 0.12;
            attribute.add_atk_percentage("决斗之枪被动", value);
            attribute.add_def_percentage("决斗之枪被动", value);
        } else {
            let value = refine * 0.06 + 0.18;
            attribute.add_atk_percentage("决斗之枪被动", value);
        }
    }
}

pub struct Deathmatch;

impl WeaponTrait for Deathmatch {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Deathmatch,
        internal_name: "Pole_Gladiator",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate80),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "身边至少有2个敌人时，获得<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>攻击力提升与<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>防御力提升；身边的敌人少于2个时，获得<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>攻击力提升。",
            en: "If there are at least 2 opponents nearby, ATK is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> and DEF is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. If there are fewer than 2 opponents nearby, ATK is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "决斗之枪",
            en: "Deathmatch"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "ge2",
            title: locale!(
                zh_cn: "身边至少有2个敌人",
                en: "At Least 2 Enemies Around"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(DeathmatchEffect::new(config)))
    }
}
