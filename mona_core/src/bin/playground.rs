use mona::character::CharacterName;
use strum::*;

fn main() {
    for c in CharacterName::iter() {
        let meta = c.get_static_data();
        println!("{:?}", meta.name_locale);
    }
}