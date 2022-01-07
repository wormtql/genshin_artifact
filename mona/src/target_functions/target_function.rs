use crate::character::Character;
use crate::attribute::AttributeGraph;
use crate::enemies::Enemy;

pub trait TargetFunction {
    fn target(&self, attribute: &AttributeGraph, enemy: &Enemy) -> f64;
}