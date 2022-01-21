use crate::attribute::ComplicatedAttributeGraph;
use crate::character::character_common_data::CharacterCommonData;
use crate::enemies::Enemy;

pub struct DamageContext<'a, A> {
    pub character_common_data: &'a CharacterCommonData,
    pub attribute: &'a A,
    pub enemy: &'a Enemy
}