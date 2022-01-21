use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct MaidenBelovedEffect {}

impl MaidenBelovedEffect {
    pub fn new() -> MaidenBelovedEffect {
        MaidenBelovedEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for MaidenBelovedEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HealingBonus, "被怜爱的少女2", 0.15);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}