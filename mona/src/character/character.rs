use super::character_common_data::CharacterCommonData;
use crate::common::ChangeAttribute;
use crate::attribute::Attribute;
use crate::character::CharacterName;
use crate::character::character_config::CharacterConfig;
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
}