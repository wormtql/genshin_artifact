use crate::common::{Element, SkillType};
use super::super::target_function::TargetFunction;
use crate::attribute::AttributeGraph;
use super::super::target_function_config::TargetFunctionConfig;
use crate::enemies::Enemy;

pub struct ExpectationTargetFunction {
    pub element: Element,
    pub skill_type: SkillType,
}

impl ExpectationTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> ExpectationTargetFunction {
        match config {
            TargetFunctionConfig::ExpectationConfig(e, s) => ExpectationTargetFunction {
                element: *e,
                skill_type: *s,
            },
            _ => unreachable!(),
        }
    }
}

impl TargetFunction for ExpectationTargetFunction {
    fn target(&self, attribute: &AttributeGraph, enemy: &Enemy) -> f64 {
        let atk = attribute.get_atk();
        // virtual skill ratio of 300%
        attribute.damage_without_enemy(atk * 3.0, self.element, self.skill_type).expectation
    }
}
