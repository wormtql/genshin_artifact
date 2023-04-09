use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;

pub struct BeaconOfTheReedSeaEffect {
    rate_atk: f64,
    rate_hp: f64,
}

impl BeaconOfTheReedSeaEffect {
    pub fn new(config: &WeaponConfig) -> BeaconOfTheReedSeaEffect {
        match *config {
            WeaponConfig::BeaconOfTheReedSea { rate_atk, rate_hp } => BeaconOfTheReedSeaEffect {
                rate_atk,
                rate_hp,
            },
            _ => BeaconOfTheReedSeaEffect {
                rate_atk: 0.0,
                rate_hp: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BeaconOfTheReedSeaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value1 = (refine * 0.10 + 0.30) * self.rate_atk;
        let value2 = (refine * 0.08 + 0.24) * self.rate_hp;
        attribute.add_atk_percentage("苇海信标被动等效", value1);
        attribute.add_hp_percentage("苇海信标被动等效", value2);
    }
}

pub struct BeaconOfTheReedSea;

impl WeaponTrait for BeaconOfTheReedSea {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BeaconOfTheReedSea,
        internal_name: "Claymore_Deshret",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技命中敌人后，攻击力提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>，持续8秒；受到伤害后，攻击力提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>，持续8秒。上述2种效果角色处于队伍后台时也能触发。此外，不处于护盾庇护下时，生命值上限提高<span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span>",
            en: "After the character's Elemental Skill hits an opponent, their ATK will be increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> for 8s. After the character takes DMG, their ATK will be increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> for 8s. The 2 aforementioned effects can be triggered even when the character is not on the field. Additionally, when not protected by a shield, the character's Max HP will be increased by <span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "苇海信标",
            en: "Beacon of the Reed Sea"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate_atk",
            title: crate::common::i18n::locale!(
                zh_cn: "攻击特效应用比例",
                en: "Equivalent Rate of Effect 1"
            ),
            config: ItemConfig::RATE01_TYPE,
        },
        ItemConfig {
            name: "rate_hp",
            title: crate::common::i18n::locale! {
                zh_cn: "生命特效应用比例",
                en: "Equivalent Rate of Effect 2"
            },
            config: ItemConfig::RATE01_TYPE,
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BeaconOfTheReedSeaEffect::new(config)))
    }
}
