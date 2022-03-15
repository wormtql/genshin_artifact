use super::weapon_common_data::WeaponCommonData;
use super::weapon_effect::WeaponEffect;
use super::weapons::get_effect;
use crate::common::ChangeAttribute;
use crate::attribute::{Attribute};
use crate::character::Character;
use crate::weapon::weapon_name::WeaponName;
use super::weapon_config::WeaponConfig;

pub struct Weapon<T> {
    pub common_data: WeaponCommonData,
    pub effect: Option<Box<dyn WeaponEffect<T>>>,
}

impl<T: Attribute> Weapon<T> {
    pub fn new(
        name: WeaponName,
        level: i32,
        ascend: bool,
        refine: i32,
        config: &WeaponConfig,
        character: &Character<T>,
    ) -> Weapon<T> {
        let common_data = WeaponCommonData::new(name, level, ascend, refine);
        let effect = get_effect(name, config, &character.common_data);

        Weapon {
            common_data,
            effect,
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for Weapon<T> {
    fn change_attribute(&self, attribute: &mut T) {
        self.common_data.change_attribute(attribute);

        if let Some(ref effect) = self.effect {
            effect.apply(&self.common_data, attribute);
        }
    }
}
