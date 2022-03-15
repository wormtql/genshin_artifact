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
use crate::team_target::team_name::TeamName;
use crate::team::TeamQuantization;


#[derive(Clone)]
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
}

lazy_static! {
    static ref COMMON_TEAMS: HashMap<u64, TeamPreset> = {
        init_teams()
    };
}

fn init_teams() -> HashMap<u64, TeamPreset> {
    let mut teams = HashMap::new();

    let mut add = |preset: TeamPreset| {
        let hash = preset.get_character_names_hash();
        teams.insert(hash, preset);
    };

    add(TeamPreset {
        team_name: TeamName::Test,
        name_and_role: vec![
            (CharacterName::Albedo, <Albedo as CharacterTrait>::RoleEnum::Sub as usize),
            (CharacterName::AratakiItto, <AratakiItto as CharacterTrait>::RoleEnum::Main as usize),
            (CharacterName::Gorou, <Gorou as CharacterTrait>::RoleEnum::Default as usize),
        ]
    });

    teams
}

pub fn match_team<A: Attribute>(team: &Team<A>) -> Option<TeamPreset> {
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

    if let Some(x) = COMMON_TEAMS.get(&target) {
        Some(x.clone())
    } else {
        None
    }
}
