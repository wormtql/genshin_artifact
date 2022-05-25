use super::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::CharacterName;
use crate::character::character_config::CharacterConfig;
use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::damage_result::SimpleDamageResult;
use crate::damage::DamageContext;
use super::characters::get_effect;

pub struct Character<T> {
    pub common_data: CharacterCommonData,
    pub character_effect: Option<Box<dyn ChangeAttribute<T>>>,
}

impl<T: Attribute> ChangeAttribute<T> for Character<T> {
    fn change_attribute(&self, attribute: &mut T) {
        self.common_data.change_attribute(attribute);
        if let Some(ref x) = self.character_effect {
            x.change_attribute(attribute);
        }
    }
}

impl<T: Attribute> Character<T> {
    pub fn new(
        name: CharacterName,
        level: usize,
        ascend: bool,
        constellation: i32,
        skill1: usize,
        skill2: usize,
        skill3: usize,
        config: &CharacterConfig
    ) -> Character<T> {
        let common_data = CharacterCommonData::new(
            name,
            level,
            ascend,
            constellation,
            skill1,
            skill2,
            skill3
        );
        let effect = get_effect(name, &common_data, config);

        Character {
            common_data,
            character_effect: effect,
        }
    }

    // pub fn damage(&self, ctx: &DamageContext<'_, SimpleAttributeGraph2>, skill_index: usize, skill_config: &CharacterSkillConfig) -> SimpleDamageResult {
    //     self.common_data.name.damage::<SimpleAttributeGraph2>(ctx, skill_index, skill_config)
    // }
}