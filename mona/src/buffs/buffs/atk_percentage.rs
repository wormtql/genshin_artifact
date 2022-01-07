use super::super::buff::Buff;
use crate::enemies::Enemy;
use crate::attribute::{AttributeGraph, AttributeName};

pub struct BuffAtkPercentage {
    pub value: f64,
}

impl Buff for BuffAtkPercentage {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        let value = self.value;
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(move |n| (String::from("buff"), value * n.value())),
        )
    }
}