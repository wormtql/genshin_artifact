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

pub struct SkyriderSwordEffect {
    pub rate: f64,
}

impl SkyriderSwordEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for SkyriderSwordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.03 + 0.09) * self.rate;
        attribute.add_atk_percentage("飞天御剑被动等效", value);
    }
}

pub struct SkyriderSword;

impl WeaponTrait for SkyriderSword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkyriderSword,
        internal_name: "Sword_Mitsurugi",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge113),
        weapon_base: WeaponBaseATKFamily::ATK354,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发后，提高<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>攻击力和移动速度，持续15秒。",
            en: "Using an Elemental Burst grants a <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> increase in ATK and Movement SPD for 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "飞天御剑",
            en: "Skyrider Sword"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::SkyriderSword { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(SkyriderSwordEffect::new(rate)))
    }
}
