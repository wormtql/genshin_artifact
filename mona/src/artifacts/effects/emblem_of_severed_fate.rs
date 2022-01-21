use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};

pub struct EmblemOfSeveredFataEffect {}

impl EmblemOfSeveredFataEffect {
    pub fn new() -> EmblemOfSeveredFataEffect {
        EmblemOfSeveredFataEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for EmblemOfSeveredFataEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "绝缘之旗印2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_edge1(
            AttributeName::Recharge,
            AttributeName::BonusElementalBurst,
            Box::new(|x, _| (x * 0.25).min(0.75)),
            Box::new(|grad, x, _| (if x < 3.0 { grad * 0.25 } else { 0.0 }, 0.0)),
            "绝缘之旗印4"
        );
    }
}