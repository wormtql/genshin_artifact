use std::collections::HashMap;
use crate::artifacts::Artifact;
use crate::attribute::Attribute;
use crate::character::{Character, CharacterName};
use crate::character::characters::get_target_function_by_role;
use crate::target_functions::TargetFunction;
// use crate::team::common_team::{match_team, TeamPreset};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct TeamEntry<A: Attribute> {
    pub character: Character<A>,
    pub weapon: Weapon<A>,
    pub artifacts: Option<Vec<Artifact>>
}

pub struct Team<A: Attribute> {
    pub members: Vec<TeamEntry<A>>,
    pub quant: TeamQuantization
}

impl<A: Attribute> Team<A> {
    // pub fn match_team_preset(&self) -> Option<TeamPreset> {
    //     match_team(&self)
    // }

    pub fn new(members: Vec<TeamEntry<A>>) -> Team<A> {
        Team {
            members,
            quant: Default::default()
        }
    }

    pub fn get_entry_by_name(&self, name: CharacterName) -> Option<&TeamEntry<A>> {
        for entry in self.members.iter() {
            if entry.character.common_data.name == name {
                return Some(entry)
            }
        }

        None
    }

    // fn get_target_functions_by_preset(&self, preset: &TeamPreset) -> HashMap<usize, Box<dyn TargetFunction>> {
    //     let mut result = HashMap::new();
    //
    //     for member in self.members.iter() {
    //         let character_name_usize = member.character.common_data.name as usize;
    //
    //         let mut flag = false;
    //         for &(name, role_index) in preset.name_and_role.iter() {
    //             if name as usize == character_name_usize {
    //                 let target_function = get_target_function_by_role(
    //                     role_index, &self.quant, &member.character.common_data, &member.weapon.common_data
    //                 );
    //                 result.insert(character_name_usize, target_function);
    //                 flag = true;
    //                 break;
    //             }
    //         }
    //         if !flag {
    //             let target_function = get_target_function_by_role(
    //                 0, &self.quant, &member.character.common_data, &member.weapon.common_data
    //             );
    //             result.insert(character_name_usize, target_function);
    //         }
    //     }
    //
    //     result
    // }

    // first, match existing team presets, and get target functions
    // if no matching team presets, get default target function for each character
    // pub fn get_default_target_functions(&self) -> HashMap<usize, Box<dyn TargetFunction>> {
    //     let preset = self.match_team_preset();
    //
    //     if let Some(p) = preset {
    //         self.get_target_functions_by_preset(&p)
    //     } else {
    //         let mut result = HashMap::new();
    //         for member in self.members.iter() {
    //             let character_name = member.character.common_data.name;
    //             let target_function = get_target_function_by_role(
    //                 0, &self.quant, &member.character.common_data, &member.weapon.common_data
    //             );
    //             result.insert(character_name as usize, target_function);
    //         }
    //
    //         result
    //     }
    // }
}
