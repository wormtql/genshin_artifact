use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct DefendersWillEffect {}

impl DefendersWillEffect {
    pub fn new() -> DefendersWillEffect {
        DefendersWillEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for DefendersWillEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("守护之心2", 0.3);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}