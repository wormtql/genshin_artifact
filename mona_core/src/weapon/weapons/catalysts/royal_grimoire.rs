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

pub struct RoyalGrimoireEffect;

impl<A: Attribute> WeaponEffect<A> for RoyalGrimoireEffect {
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

pub struct RoyalGrimoire;

impl WeaponTrait for RoyalGrimoire {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RoyalGrimoire,
        internal_name: "Catalyst_Theocrat",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("专注：攻击造成伤害时，暴击率提升8%/10%/12%/14%/16%，最多堆叠5次。攻击造成暴击后，移除已有的专注效果。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "宗室秘法录"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(RoyalGrimoireEffect))
    }
}
