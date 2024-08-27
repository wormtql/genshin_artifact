use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct EarthShakerEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for EarthShakerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus = 0.12 + 0.04 * refine;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "撼地者被动", bonus * self.rate);
    }
}

pub struct EarthShaker;

impl WeaponTrait for EarthShaker {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EarthShaker,
        internal_name: "Claymore_Isikhulu",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "队伍中的角色触发火元素相关反应后，装备者元素战技造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，持续8秒。该效果队伍中的角色处于队伍后台时也能触发。",
            en: "After a party member triggers a Pyro-related reaction, the equipping character's Elemental Skill DMG is increased by After a party member triggers a Pyro-related reaction, the equipping character's Elemental Skill DMG is increased by 16% for 8s. This effect can be triggered even when the triggering party member is not on the field. for 8s. This effect can be triggered even when the triggering party member is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "撼地者",
            en: "Earth Shaker"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::EarthShaker { rate } => Some(Box::new(EarthShakerEffect {
                rate
            })),
            _ => None
        }
    }
}
