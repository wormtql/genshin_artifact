pub mod team_target_raiden_kujou_kazuha_bennett;
pub mod team_target_ayaka_rosaria_kokomi_kazuha;

use std::collections::{HashMap, HashSet};
pub use team_target_raiden_kujou_kazuha_bennett::TeamTargetRaidenKujouKazuhaBennett;
pub use team_target_ayaka_rosaria_kokomi_kazuha::TeamTargetAyakaRosariaKokomiKazuha;

use crate::attribute::Attribute;
use crate::buffs::Buff;
use crate::character::CharacterName;
use crate::team::team::Team;
use crate::team_target::team_name::TeamName;
use crate::team_target::team_target_config::TeamTargetFunctionConfig;
use crate::team_target::team_target_function::TeamTargetFunction;

pub fn get_members(name: TeamName) -> &'static [CharacterName] {
    name.get_members()
}

pub fn get_default_buff<A: Attribute>(name: TeamName, team: &Team<A>) -> HashMap<usize, Vec<Box<dyn Buff<A>>>> {
    name.get_default_buffs(team)
}

pub fn try_match_team<A: Attribute>(team: &Team<A>) -> Option<TeamName> {
    let len = TeamName::LEN;

    for i in 0_usize..len {
        let e: TeamName = num::FromPrimitive::from_usize(i).unwrap();
        let members = get_members(e);
        let mut set = HashSet::new();
        for &member in members.iter() {
            set.insert(member as usize);
        }

        for team_entry in team.members.iter() {
            let name = team_entry.character.common_data.name;
            set.remove(&(name as usize));
        }

        if set.is_empty() {
            return Some(e)
        }
    }

    None
}

pub fn try_get_team_target_function<A: Attribute>(team: &Team<A>, config: &TeamTargetFunctionConfig) -> Option<Box<dyn TeamTargetFunction>> {
    let team_name = try_match_team(team);
    if team_name.is_none() {
        None
    } else {
        Some(team_name.unwrap().create(config, team))
    }
}
