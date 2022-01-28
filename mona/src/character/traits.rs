use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterConfig, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::ChangeAttribute;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

// pub trait CharacterEffect<A: Attribute> {
//     type EffectType: ChangeAttribute<A>;
//
//     fn new_effect(common_data: &CharacterCommonData, config: &CharacterConfig) -> Self::EffectType;
// }
//
// pub trait CharacterCommon: CharacterConstant {
//     fn get_target_function_by_role<A: Attribute>(
//         role_index: usize,
//         team: &TeamQuantization,
//         c: &CharacterCommonData,
//         w: &WeaponCommonData
//     ) -> Box<dyn TargetFunction>;
// }

pub trait CharacterTrait {
    const STATIC_DATA: CharacterStaticData;
    type SkillType;
    const SKILL: Self::SkillType;
    type DamageEnumType: Copy + Clone + Into<usize>;
    type RoleEnum;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig) -> D::Result;

    fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: Self::DamageEnumType, config: &CharacterSkillConfig) -> D::Result {
        Self::damage_internal::<D>(context, s.into(), config)
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Box<dyn ChangeAttribute<A>>;

    fn get_target_function_by_role(
        role_index: usize,
        team: &TeamQuantization,
        c: &CharacterCommonData,
        w: &WeaponCommonData
    ) -> Box<dyn TargetFunction>;
}
