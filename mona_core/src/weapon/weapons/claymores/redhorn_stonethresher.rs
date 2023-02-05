use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct RedhornStonethresherEffect;

impl RedhornStonethresherEffect {
    pub fn new() -> RedhornStonethresherEffect {
        RedhornStonethresherEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for RedhornStonethresherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let def_bonus = refine * 0.07 + 0.21;
        attribute.add_def_percentage("赤角石溃杵被动", def_bonus);
        let bonus = refine * 0.1 + 0.3;
        attribute.set_value_by(AttributeName::DEFRatioNormalAttack, "赤角石溃杵被动", bonus);
        attribute.set_value_by(AttributeName::DEFRatioChargedAttack, "赤角石溃杵被动", bonus);
    }
}

pub struct RedhornStonethresher;

impl WeaponTrait for RedhornStonethresher {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RedhornStonethresher,
        internal_name: "Claymore_Itadorimaru",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "防御力提高<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>；普通攻击与重击造成的伤害值提高，提高数值相当于防御力的<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "DEF is increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>. Normal and Charged Attack DMG is increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> of DEF."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "赤角石溃杵",
            en: "Redhorn Stonethresher"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(RedhornStonethresherEffect::new()))
    }
}
