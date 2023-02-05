use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct WhiteTasselEffect;

impl<A: Attribute> WeaponEffect<A> for WhiteTasselEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = refine * 0.06 + 0.18;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "白缨枪被动", value);
    }
}

pub struct WhiteTassel;

impl WeaponTrait for WhiteTassel {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WhiteTassel,
        internal_name: "Pole_Ruby",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate51),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击造成的伤害提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。",
            en: "Increases Normal Attack DMG by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "白缨枪",
            en: "White Tassel"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(WhiteTasselEffect))
    }
}
