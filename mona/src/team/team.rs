use std::collections::HashMap;
use crate::artifacts::Artifact;
use crate::attribute::Attribute;
use crate::character::Character;
use crate::character::characters::get_target_function_by_role;
use crate::target_functions::TargetFunction;
use crate::team::common_team::{match_team, TeamPreset};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct TeamEntry<A: Attribute> {
    pub character: Character<A>,
    pub weapon: Weapon<A>,
    pub artifacts: Option<Vec<Artifact>>
}

pub struct Team<A: Attribute> {
    pub members: Vec<TeamEntry<A>>,
}

impl<A: Attribute> Team<A> {
    pub fn match_team_preset(&self) -> Option<&'static TeamPreset> {
        match_team(&self)
    }

    // first, match existing team presets, and get target functions
    // if no matching team presets, get default target function for each character
    pub fn get_default_target_functions(&self) -> HashMap<usize, Box<dyn TargetFunction>> {
        let preset = self.match_team_preset();

        let team_quant = TeamQuantization::default(); // todo generate team quantization from Team
        if let Some(p) = preset {
            p.get_target_functions(self, &team_quant)
        } else {
            let mut result = HashMap::new();
            for member in self.members.iter() {
                let character_name = member.character.common_data.name;
                let target_function = get_target_function_by_role(
                    0, &team_quant, &member.character.common_data, &member.weapon.common_data
                );
                result.insert(character_name as usize, target_function);
            }

            result
        }
    }
}
