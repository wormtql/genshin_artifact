use crate::attribute::{Attribute, AttributeName};
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

pub struct TheBellEffect {
    rate: f64
}

impl TheBellEffect {
    pub fn new(config: &WeaponConfig) -> TheBellEffect {
        match *config {
            WeaponConfig::TheBell { rate } => TheBellEffect {
                rate
            },
            _ => TheBellEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheBellEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "钟剑被动等效", value);
    }
}

pub struct TheBell;

impl WeaponTrait for TheBell {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheBell,
        internal_name: "Claymore_Troupe",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "受到伤害时，生成一个伤害吸收量等同于生命值上限<span style=\"color: #409EFF;\">20%-23%-26%-29%-32%</span>的护盾，持续10秒或直到护盾失效,每<span style=\"color: #409EFF;\">45-45-45-45-45</span>秒只能触发一次。角色处于护盾庇护下时，造成的伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。",
            en: "Taking DMG generates a shield which absorbs DMG up to <span style=\"color: #409EFF;\">20%-23%-26%-29%-32%</span> of Max HP. This shield lasts for 10s or until broken, and can only be triggered once every <span style=\"color: #409EFF;\">45-45-45-45-45</span>s. While protected by a shield, the character gains <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> increased DMG."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "钟剑",
            en: "The Bell"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfigType::Float {
                min: 0.0, max: 1.0, default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheBellEffect::new(config)))
    }
}
