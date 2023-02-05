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

pub struct AmosBowEffect {
    stack: f64
}

impl AmosBowEffect {
    pub fn new(config: &WeaponConfig) -> AmosBowEffect {
        match *config {
            WeaponConfig::AmosBow { stack } => AmosBowEffect {
                stack
            },
            _ => AmosBowEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for AmosBowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus = refine * 0.03 + 0.09 + (refine * 0.02 + 0.06) * self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "阿莫斯之弓", bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "阿莫斯之弓", bonus);
    }
}

pub struct AmosBow;

impl WeaponTrait for AmosBow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AmosBow,
        internal_name: "Bow_Amos",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "普通攻击与重击造成的伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>；普通攻击与重击的箭矢发射后每经过0.1秒，伤害还会提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，至多提升5次。",
            en: "Increases Normal and Charged Attack DMG by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. After a Normal or Charged Attack is fired, DMG dealt increases by a further <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> every 0.1s the arrow is in the air for up to 5 times."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "阿莫斯之弓",
            en: "Amos' Bow",
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK05
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(AmosBowEffect::new(config)))
    }
}
