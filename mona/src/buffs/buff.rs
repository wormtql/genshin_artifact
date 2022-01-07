use crate::enemies::Enemy;
use crate::attribute::AttributeGraph;

pub trait Buff {
    fn change_enemy(&self, enemy: &mut Enemy) {}

    fn change_attribute(&self, attribute: &mut AttributeGraph) {}
}