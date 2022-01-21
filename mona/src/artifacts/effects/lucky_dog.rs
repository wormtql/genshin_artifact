use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};

pub struct LuckyDogEffect {}

impl LuckyDogEffect {
    pub fn new() -> LuckyDogEffect {
        LuckyDogEffect {}
    }
}

impl<T: Attribute> ArtifactEffect<T> for LuckyDogEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::DEFFixed, "幸运儿2", 100.0);
    }
}