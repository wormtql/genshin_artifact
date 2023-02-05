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

pub struct TheBlackSwordEffect;

impl TheBlackSwordEffect {
    pub fn new() -> TheBlackSwordEffect {
        TheBlackSwordEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for TheBlackSwordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let bonus = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "黑剑被动", bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "黑剑被动", bonus);
    }
}

pub struct TheBlackSword;

impl WeaponTrait for TheBlackSword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheBlackSword,
        internal_name: "Sword_Bloodstained",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate60),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击与重击的造成的伤害提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>；此外，普通攻击与重击暴击时，回复等同于攻击力<span style=\"color: #409EFF;\">60%-70%-80%-90%-100%</span>的生命值。该效果每5秒至多发动一次。",
            en: "Increases DMG dealt by Normal and Charged Attacks by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. Additionally, regenerates <span style=\"color: #409EFF;\">60%-70%-80%-90%-100%</span> of ATK as HP when Normal and Charged Attacks score a CRIT Hit. This effect can occur once every 5s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "黑剑",
            en: "The Black Sword"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheBlackSwordEffect::new()))
    }
}
