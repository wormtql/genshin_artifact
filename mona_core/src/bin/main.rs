use mona::attribute::SimpleAttributeGraph2;
use mona::character::{Character, CharacterConfig, CharacterName};

fn main() {
    let character: Character<SimpleAttributeGraph2> = Character::new(CharacterName::Amber, 90, false, 0, 8, 8, 8, &CharacterConfig::NoConfig);

    println!("{}", character.common_data.base_atk);
}