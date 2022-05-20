use crate::artifacts::ArtifactList;
use crate::character::Character;
use crate::weapon::weapon::Weapon;
use crate::buffs::Buff;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::attribute::{Attribute, AttributeCommon};
use crate::common::ChangeAttribute;

pub struct AttributeUtils {}

impl AttributeUtils {
    pub fn create_attribute_from_c_w_bs<T: Attribute>(
        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &Vec<Box<dyn Buff<T>>>
    ) -> T {
        let mut attribute = T::new_with_base_edge();

        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute
    }

    pub fn create_attribute_from_big_config<T: Attribute>(
        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T {
        let mut attribute = T::new_with_base_edge();

        character.change_attribute(&mut attribute);
        // artifacts.apply(&mut attribute, character, config);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute
    }
}