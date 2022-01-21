use crate::attribute::Attribute;
use crate::common::{DamageResult, Element, SkillType};
use crate::damage::DamageAnalysis;
use crate::enemies::Enemy;

pub trait DamageBuilder {
    type Result;
    type AttributeType: Attribute;

    fn new() -> Self;

    fn add_atk_ratio(&mut self, key: &str, value: f64);

    fn add_def_ratio(&mut self, key: &str, value: f64);

    fn add_hp_ratio(&mut self, key: &str, value: f64);

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

    fn add_extra_res_minus(&mut self, key: &str, value: f64);

    fn build(&self, attribute: &Self::AttributeType, enemy: &Enemy, element: Element, skill_type: SkillType, is_heal: bool, character_level: usize) -> Self::Result;
}
