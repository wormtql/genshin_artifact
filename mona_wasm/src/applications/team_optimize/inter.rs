use serde::{Serialize, Deserialize};
use crate::applications::common::{BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizeArtifactInterface};
use crate::applications::team_optimize::hyper_param::TeamOptimizeHyperParam;
use mona::artifacts::Artifact;
use mona::attribute::Attribute;
use mona::character::Character;
use mona::team::team::{Team, TeamEntry};
// use mona::team_target::team_target_config::TeamTargetFunctionConfig;
use mona::weapon::Weapon;

#[derive(Serialize, Deserialize)]
pub struct TeamInterface {
    pub characters: Vec<CharacterInterface>,
    pub weapons: Vec<WeaponInterface>,
}

#[derive(Serialize, Deserialize)]
pub struct OptimizeTeamObject {
    // pub team: TeamInterface,
    pub weapons: Vec<WeaponInterface>,
    pub characters: Vec<CharacterInterface>,
    pub buffs: Vec<Vec<BuffInterface>>,
    pub target_functions: Vec<TargetFunctionInterface>,
    pub constraints: Vec<ConstraintConfig>,
    pub weights: Vec<f64>,
    // pub override_target_functions: Option<Vec<Option<TargetFunctionInterface>>>,
    pub artifacts: Vec<Artifact>,
    pub hyper_param: Option<TeamOptimizeHyperParam>
}

#[derive(Serialize, Deserialize)]
pub struct OptimizeTeamInterface2 {
    // pub artifacts: Vec<Artifact>,
    pub single_interfaces: Vec<OptimizeArtifactInterface>,
    pub weights: Vec<f64>,
    pub hyper_param: Option<TeamOptimizeHyperParam>
}

#[derive(Serialize, Deserialize)]
pub struct OptimizeTeamResultEntry {
    pub flower: Option<u64>,
    pub feather: Option<u64>,
    pub sand: Option<u64>,
    pub goblet: Option<u64>,
    pub head: Option<u64>,
}

impl OptimizeTeamResultEntry {
    pub fn new() -> Self {
        OptimizeTeamResultEntry {
            flower: None,
            feather: None,
            sand: None,
            goblet: None,
            head: None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OptimizeTeamResult {
    pub artifacts: Vec<Vec<OptimizeTeamResultEntry>>
}

impl TeamInterface {
    pub fn len(&self) -> usize {
        self.characters.len()
    }

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

        Team::new(members)
    }
}