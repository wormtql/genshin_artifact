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

pub struct SkywardHarpEffect;

impl SkywardHarpEffect {
    pub fn new() -> SkywardHarpEffect {
        SkywardHarpEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardHarpEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalDamageBase, "天空之翼被动", data.refine as f64 * 0.05 + 0.15);
    }
}

pub struct SkywardHarp;

impl WeaponTrait for SkywardHarp {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardHarp,
        internal_name: "Bow_Dvalin",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "暴击伤害提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>；攻击命中时有<span style=\"color: #409EFF;\">60%-70%-80%-90%-100%</span>概率造成125%攻击力的小范围物理伤害，该效果每<span style=\"color: #409EFF;\">4-3.5-3-2.5-2</span>秒只能触发一次。",
            en: "Increases CRIT DMG by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Hits have a <span style=\"color: #409EFF;\">60%-70%-80%-90%-100%</span> chance to inflict a small AoE attack, dealing 125% Physical ATK DMG. Can only occur once every <span style=\"color: #409EFF;\">4-3.5-3-2.5-2</span>s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "天空之翼",
            en: "Skyward Harp"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardHarpEffect::new()))
    }
}
