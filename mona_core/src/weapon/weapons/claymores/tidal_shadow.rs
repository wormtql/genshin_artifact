use crate::attribute::{Attribute, AttributeCommon};
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

pub struct TidalShadowEffect {
    pub rate: f64
}

impl<A: Attribute> WeaponEffect<A> for TidalShadowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = 0.06 * refine + 0.18;
        attribute.add_atk_percentage("浪影阔剑被动", value * self.rate);
    }
}

pub struct TidalShadow;

impl WeaponTrait for TidalShadow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TidalShadow,
        internal_name: "Claymore_Vorpal",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "受到治疗后，攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>，持续8秒。角色处于队伍后台也能触发。",
            en: "After the wielder is healed, ATK will be increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 8s. This can be triggered even when the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "浪影阔剑",
            en: "Tidal Shadow",
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::TidalShadow { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(TidalShadowEffect {
            rate
        }))
    }
}
