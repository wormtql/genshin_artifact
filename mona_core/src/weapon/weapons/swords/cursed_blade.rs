use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct CursedBladeEffect {
    rate: f64
}

impl<A: Attribute> WeaponEffect<A> for CursedBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        attribute.add_atk_percentage("笼钓瓶一心被动", 0.15 * self.rate);
    }
}

pub struct CursedBlade;

impl WeaponTrait for CursedBlade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CursedBlade,
        internal_name: "Sword_Youtou",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击、重击或下落攻击命中敌人时，将卷起切落风，造成<span style=\"color: #409EFF;\">180%</span>攻击力的范围伤害，并且使攻击力提升<span style=\"color: #409EFF;\">15%</span>，持续8秒。该效果每8秒至多触发一次。",
            en: "When a Normal, Charged, or Plunging Attack hits an opponent, it will whip up a Hewing Gale, dealing AoE DMG equal to <span style=\"color: #409EFF;\">180%</span> of ATK and increasing ATK by <span style=\"color: #409EFF;\">15%</span> for 8s. This effect can be triggered once every 8s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "笼钓瓶一心",
            en: "Kagotsurube Isshin"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::CursedBlade { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(CursedBladeEffect {
            rate
        }))
    }
}
