use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::ChangeAttribute;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;

pub trait CharacterConstant {
    const STATIC_DATA: CharacterStaticData;
    type SkillType;
    const SKILL: Self::SkillType;
    type DamageEnumType: Copy + Clone + Into<usize>;
}

pub trait CharacterDamage<D: DamageBuilder>: CharacterConstant {
    fn damage_internal(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig) -> D::Result;

    fn damage(context: &DamageContext<'_, D::AttributeType>, s: Self::DamageEnumType, config: &CharacterSkillConfig) -> D::Result {
        Self::damage_internal(context, s.into(), config)
    }
}

pub trait CharacterEffect<A: Attribute> {
    type EffectType: ChangeAttribute<A>;

    fn new_effect(common_data: &CharacterCommonData, config: &CharacterConfig) -> Self::EffectType;
}

pub trait CharacterTrait<A: Attribute, D: DamageBuilder>: CharacterConstant + CharacterDamage<D> + CharacterEffect<A> {

}
