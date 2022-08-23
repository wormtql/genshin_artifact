use crate::attribute::{Attribute, ComplicatedAttributeGraph};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::Element;
use crate::damage::transformative_damage::{TransformativeDamage, transformative_damage, swirl_without_element};
use crate::enemies::Enemy;

pub struct DamageContext<'a, A> {
    pub character_common_data: &'a CharacterCommonData,
    pub attribute: &'a A,
    pub enemy: &'a Enemy,
}

impl<'a, A: Attribute> DamageContext<'a, A> {
    pub fn transformative(&self) -> TransformativeDamage {
        let level = self.character_common_data.level;

        transformative_damage::<A>(level, &self.attribute, &self.enemy)
    }

    pub fn swirl_without_element(&self) -> f64 {
        swirl_without_element::<A>(
            self.character_common_data.level, &self.attribute, 0.9
        )
    }
}
