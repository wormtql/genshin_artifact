use super::weapon_common_data::WeaponCommonData;
use crate::attribute::{AttributeGraph};
use crate::character::Character;

pub trait WeaponEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut AttributeGraph);
}