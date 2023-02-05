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

pub struct SkyriderGreatswordEffect {
    pub stack: f64,
}

impl SkyriderGreatswordEffect {
    pub fn new(stack: f64) -> Self {
        Self { stack }
    }
}

impl<A: Attribute> WeaponEffect<A> for SkyriderGreatswordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.01 + 0.05) * self.stack;
        attribute.add_atk_percentage("飞天大御剑被动等效", value);
    }
}

pub struct SkyriderGreatsword;

impl WeaponTrait for SkyriderGreatsword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkyriderGreatsword,
        internal_name: "Claymore_Mitsurugi",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus96),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击和重击命中时，攻击力提高<span style=\"color: #409EFF;\">6%-7%-8%-9%-10%</span>，持续6秒，最多叠加4层。该效果每0.5秒只能触发一次。",
            en: "On hit, Normal or Charged Attacks increase ATK by <span style=\"color: #409EFF;\">6%-7%-8%-9%-10%</span> for 6s. Max 4 stacks. Can occur once every 0.5s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "飞天大御剑",
            en: "Skyrider Greatsword"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK04
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::SkyriderGreatsword { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(SkyriderGreatswordEffect::new(stack)))
    }
}
