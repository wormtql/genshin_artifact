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

pub struct MailedFlowerEffect {
    rate: f64,
}

impl MailedFlowerEffect {
    pub fn new(config: &WeaponConfig) -> MailedFlowerEffect {
        match *config {
            WeaponConfig::MailedFlower { rate } => MailedFlowerEffect {
                rate,
            },
            _ => MailedFlowerEffect {
                rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MailedFlowerEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value1 = (refine * 0.03 + 0.09)*self.rate;
        let value2 = (refine * 12.0 + 36.0)*self.rate;
        attribute.add_atk_percentage("饰铁之花被动等效", value1);
        attribute.set_value_by(AttributeName::ElementalMastery, "饰铁之花被动等效", value2);
    }
}

pub struct MailedFlower;

impl WeaponTrait for MailedFlower {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MailedFlower,
        internal_name: "Claymore_Fleurfair",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM24),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技命中敌人或触发元素反应后的8秒内，攻击力提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，元素精通提升<span style=\"color: #409EFF;\">48-60-72-84-96</span>点。",
            en: "Within 8s after the character's Elemental Skill hits an opponent or triggers an Elemental Reaction, the character's ATK and Elemental Mastery will be increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> and <span style=\"color: #409EFF;\">48-60-72-84-96</span> respectively."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "饰铁之花",
            en: "Mailed Flower"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "应用比例",
                en: "Equivalent Rate of Effect"
            ),
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MailedFlowerEffect::new(config)))
    }
}
