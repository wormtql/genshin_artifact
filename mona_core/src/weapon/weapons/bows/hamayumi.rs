use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct HamayumiEffect {
    rate: f64
}

impl HamayumiEffect {
    pub fn new(config: &WeaponConfig) -> HamayumiEffect {
        match *config {
            WeaponConfig::Hamayumi { rate } => HamayumiEffect {
                rate
            },
            _ => HamayumiEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for HamayumiEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus_a = (refine * 0.04 + 0.12) * (1.0 + self.rate);
        let bonus_b = bonus_a * 0.75;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "破魔之弓被动", bonus_a);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "破魔之弓被动", bonus_b);
    }
}

pub struct Hamayumi;

impl WeaponTrait for Hamayumi {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Hamayumi,
        internal_name: "Bow_Bakufu",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "普通攻击造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，重击造成的伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>。当装备该武器的角色元素能量等于100%时，这个效果提升100%。",
            en: "Increases Normal Attack DMG by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> and Charged Attack DMG by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. When the equipping character's Energy reaches 100%, this effect is increased by 100%."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "破魔之弓",
            en: "Hamayumi"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(HamayumiEffect::new(config)))
    }
}
