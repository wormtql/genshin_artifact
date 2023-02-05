use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::royal_series::royal_series_critical_bonus;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct RoyalGreatswordEffect;

impl<A: Attribute> WeaponEffect<A> for RoyalGreatswordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as usize;
        attribute.add_edge1(
            AttributeName::CriticalBase,
            AttributeName::CriticalAttacking,
            Box::new(move |x, _| royal_series_critical_bonus(refine, x)),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "宗室被动等效"
        )
    }
}

pub struct RoyalGreatsword;

impl WeaponTrait for RoyalGreatsword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RoyalGreatsword,
        internal_name: "Claymore_Theocrat",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击造成伤害时，暴击率提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，最多堆叠5次。攻击造成暴击后，移除已有的专注效果。",
            en: "Upon damaging an opponent, increases CRIT Rate by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>. Max 5 stacks. A CRIT Hit removes all stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "宗室大剑",
            en: "Royal Greatsword"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(RoyalGreatswordEffect))
    }
}
