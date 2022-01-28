use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use crate::character::CharacterName;
use lazy_static::lazy_static;
use smallvec::SmallVec;
use crate::attribute::Attribute;
use crate::character::characters::*;
use crate::character::traits::CharacterTrait;
use crate::target_functions::TargetFunction;
use crate::team::team::Team;
use crate::team::team_name::TeamName;
use crate::team::TeamQuantization;


pub struct TeamPreset {
    pub team_name: TeamName,
    pub name_and_role: Vec<(CharacterName, usize)>, // name, role as usize
}

impl TeamPreset {
    pub fn get_character_names_hash(&self) -> u64 {
        let mut names: SmallVec<[usize; 4]> = SmallVec::new();
        for &(name, _) in self.name_and_role.iter() {
            names.push(name as usize);
        }
        names.sort();

        let mut hasher = DefaultHasher::new();
        for name in names {
            name.hash(&mut hasher);
        }

        hasher.finish()
    }

    pub fn get_target_functions<A: Attribute>(&self, team: &Team<A>, team_quant: &TeamQuantization) -> HashMap<usize, Box<dyn TargetFunction>> {
        let mut result = HashMap::new();

        for member in team.members.iter() {
            let character_name_usize = member.character.common_data.name as usize;
            for &(name, role_index) in self.name_and_role.iter() {
                if name as usize == character_name_usize {
                    let target_function = get_target_function_by_role(
                        role_index, team_quant, &member.character.common_data, &member.weapon.common_data
                    );
                    result.insert(character_name_usize, target_function);
                    break;
                }
            }
        }

        result
    }
}

lazy_static! {
    static ref COMMON_TEAMS: Vec<TeamPreset> = {
        init_teams()
    };
}

fn init_teams() -> Vec<TeamPreset> {
    let mut teams = Vec::new();

    teams.push(TeamPreset {
        team_name: TeamName::Test,
        name_and_role: vec![
            (CharacterName::Albedo, <Albedo as CharacterTrait>::RoleEnum::Sub as usize),
            (CharacterName::AratakiItto, <AratakiItto as CharacterTrait>::RoleEnum::Main as usize),
            (CharacterName::Gorou, <Gorou as CharacterTrait>::RoleEnum::Default as usize),
        ]
    });

    teams
}

pub fn match_team<A: Attribute>(team: &Team<A>) -> Option<&'static TeamPreset> {
    let mut names: Vec<usize> = Vec::new();
    for member in team.members.iter() {
        names.push(member.character.common_data.name as usize);
    }
    names.sort();

    let mut hasher = DefaultHasher::new();
    for name in names.iter() {
        name.hash(&mut hasher);
    }

    let target = hasher.finish();

    for team_preset in COMMON_TEAMS.iter() {
        if team_preset.get_character_names_hash() == target {
            return Some(team_preset);
        }
    }

    None
}
