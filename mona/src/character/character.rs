use super::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;
use crate::attribute::AttributeGraph;
use crate::character::CharacterName;
use crate::character::character_config::CharacterConfig;
use super::characters::get_effect;

pub struct Character {
    pub common_data: CharacterCommonData,
    pub character_effect: Option<Box<dyn ChangeAttribute>>,
}

impl ChangeAttribute for Character {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        self.common_data.change_attribute(attribute);
        if let Some(ref x) = self.character_effect {
            x.change_attribute(attribute);
        }
    }
}

impl Character {
    pub fn new(name: CharacterName, level: i32, ascend: bool, constellation: i32, config: &CharacterConfig) -> Character {
        let common_data = CharacterCommonData::new(name, level, ascend, constellation);
        let effect = get_effect(name, &common_data);

        Character {
            common_data,
            character_effect: effect,
        }
    }
}