use crate::artifacts::ArtifactList;
use crate::character::Character;
use crate::weapon::weapon::Weapon;
use crate::buffs::Buff;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::attribute::AttributeGraph;
use crate::common::ChangeAttribute;

pub struct AttributeUtils {}

impl AttributeUtils {
    pub fn create_attribute_from_big_config(
        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character,
        weapon: &Weapon,
        buffs: &Vec<Box<dyn Buff>>,
    ) -> AttributeGraph {
        let mut attribute = AttributeGraph::new();

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