use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct WolfsGravestoneEffect {
    rate: f64,
}

impl WolfsGravestoneEffect {
    pub fn new(config: &WeaponConfig) -> WolfsGravestoneEffect {
        match *config {
            WeaponConfig::WolfsGravestone { rate } => WolfsGravestoneEffect {
                rate
            },
            _ => WolfsGravestoneEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WolfsGravestoneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value1 = refine * 0.05 + 0.15;
        let value2 = (refine * 0.1 + 0.3) * self.rate;
        attribute.add_atk_percentage("狼的末路被动等效", value1 + value2);
    }
}

pub struct WolfsGravestone;

impl WeaponTrait for WolfsGravestone {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WolfsGravestone,
        internal_name: "Claymore_Wolfmound",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>；攻击命中生命值低于30%的敌人时，队伍中所有成员的攻击力提高<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>，持续12秒。该效果30秒只能触发一次。",
            en: "Increases ATK by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. On hit, attacks against opponents with less than 30% HP increase all party members' ATK by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> for 12s. Can only occur once every 30s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "狼的末路",
            en: "Wolf's Gravestone"
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
        Some(Box::new(WolfsGravestoneEffect::new(config)))
    }
}
