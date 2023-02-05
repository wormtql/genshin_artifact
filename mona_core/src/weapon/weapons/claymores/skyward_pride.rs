use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct  SkywardPrideEffect;

impl SkywardPrideEffect {
    pub fn new() -> SkywardPrideEffect {
        SkywardPrideEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardPrideEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.02 + 0.06;
        attribute.set_value_by(AttributeName::BonusBase, "天空之傲被动", value);
    }
}

pub struct SkywardPride;

impl WeaponTrait for SkywardPride {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardPride,
        internal_name: "Claymore_Dvalin",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge80),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "造成的伤害提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>；施放元素爆发后：普通攻击和重击命中时会发出真空刃，对路径上的敌人造成<span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span>攻击力的伤害，持续20秒或直至发出8次真空刃。",
            en: "Increases all DMG by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>. After using an Elemental Burst, Normal or Charged Attack, on hit, creates a vacuum blade that does <span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span> of ATK as DMG to opponents along its path. Lasts for 20s or 8 vacuum blades."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "天空之傲",
            en: "Skyward Pride"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardPrideEffect::new()))
    }
}
