use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::attribute::Attribute;
use crate::character::Character;
use crate::team::team::{Team, TeamEntry};
use crate::weapon::Weapon;

pub struct TeamInterface {
    pub characters: Vec<CharacterInterface>,
    pub weapons: Vec<WeaponInterface>,
    pub target_functions: Vec<Option<TargetFunctionInterface>>
}

impl TeamInterface {
    pub fn to_team<A: Attribute>(&self) -> Team<A> {
        // let mut characters: Vec<Character<A>> = Vec::new();
        // let mut weapons: Vec<Weapon<A>> = Vec::new();
        let mut members: Vec<TeamEntry<A>> = Vec::new();

        for i in 0..self.characters.len() {
            let character = self.characters[i].to_character();
            let weapon = self.weapons[i].to_weapon(&character);
            let member = TeamEntry {
                character, weapon,
                artifacts: None
            };
            members.push(member);
        }

        Team {
            members
        }
    }
}