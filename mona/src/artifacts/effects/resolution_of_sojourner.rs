use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct ResolutionOfSojournerEffect {}

impl ResolutionOfSojournerEffect {
    pub fn new() -> ResolutionOfSojournerEffect {
        ResolutionOfSojournerEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for ResolutionOfSojournerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("行者之心2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalChargedAttack, "行者之心4", 0.3);
    }
}