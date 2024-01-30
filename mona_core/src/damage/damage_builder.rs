use crate::attribute::Attribute;
use crate::common::{DamageResult, Element, SkillType};
use crate::common::reaction_type::TransformativeType;
use crate::damage::DamageAnalysis;
use crate::enemies::Enemy;

pub trait DamageBuilder {
    type Result;
    type AttributeType: Attribute;

    fn new() -> Self;

    fn add_atk_ratio(&mut self, key: &str, value: f64);

    fn add_def_ratio(&mut self, key: &str, value: f64);

    fn add_hp_ratio(&mut self, key: &str, value: f64);

    fn add_em_ratio(&mut self, key: &str, value: f64);

    fn add_extra_em(&mut self, key: &str, value: f64);

    fn add_extra_atk(&mut self, key: &str, value: f64);

    fn add_extra_def(&mut self, key: &str, value: f64);

    fn add_extra_hp(&mut self, key: &str, value: f64);

    fn add_extra_damage(&mut self, key: &str, value: f64);

    fn add_extra_critical(&mut self, key: &str, value: f64);

    fn add_extra_critical_damage(&mut self, key: &str, value: f64);

    fn add_extra_bonus(&mut self, key: &str, value: f64);

    fn add_extra_enhance_melt(&mut self, key: &str, value: f64);

    fn add_extra_enhance_vaporize(&mut self, key: &str, value: f64);

    fn add_extra_def_minus(&mut self, key: &str, value: f64);

    fn add_extra_def_penetration(&mut self, key: &str, value: f64);

    fn add_extra_res_minus(&mut self, key: &str, value: f64);

    fn damage(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        skill_type: SkillType,
        character_level: usize,
        fumo: Option<Element>
    ) -> Self::Result;

    fn heal(&self, attribute: &Self::AttributeType) -> Self::Result;

    fn shield(&self, attribute: &Self::AttributeType, element: Element) -> Self::Result;
}
