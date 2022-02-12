use std::collections::HashMap;
use smallvec::SmallVec;
use wasm_bindgen::prelude::*;
use crate::applications::team_optimize::hyper_param::TeamOptimizeHyperParam;
use crate::applications::team_optimize::inter::{OptimizeTeamObject, OptimizeTeamResult, OptimizeTeamResultEntry, TeamInterface};
use crate::applications::team_optimize::team_optimize::optimize_team;
use crate::artifacts::{Artifact, ArtifactSlotName};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::target_functions::TargetFunction;
use crate::team::team::Team;
use crate::weapon::Weapon;

pub struct TeamOptimizationWasm;

fn artifacts_by_id_hashmap<'a>(artifacts: &[&'a Artifact]) -> HashMap<usize, &'a Artifact> {
    let mut temp = HashMap::new();
    for &art in artifacts.iter() {
        temp.insert(art.id, art);
    }
    temp
}

fn smallvec_to_optimize_entry(v: &SmallVec<[usize; 5]>, artifacts_by_id: &HashMap<usize, &Artifact>) -> OptimizeTeamResultEntry {
    let mut temp = OptimizeTeamResultEntry::new();
    for &art_id in v.iter() {
        let art = *artifacts_by_id.get(&art_id).unwrap();
        match art.slot {
            ArtifactSlotName::Flower => {
                temp.flower = Some(art_id);
            },
            ArtifactSlotName::Feather => {
                temp.feather = Some(art_id);
            },
            ArtifactSlotName::Sand => {
                temp.sand = Some(art_id);
            },
            ArtifactSlotName::Goblet => {
                temp.goblet = Some(art_id);
            },
            ArtifactSlotName::Head => {
                temp.head = Some(art_id);
            }
        }
    }

    temp
}

#[wasm_bindgen]
impl TeamOptimizationWasm {
    pub fn optimize_team(val: &JsValue) -> JsValue {
        let input: OptimizeTeamObject = val.into_serde().unwrap();

        let mut characters: Vec<Character<SimpleAttributeGraph2>> = Vec::new();
        let mut weapons: Vec<Weapon<SimpleAttributeGraph2>> = Vec::new();

        for i in 0..input.team.len() {
            let character = input.team.characters[i].to_character();
            let weapon = input.team.weapons[i].to_weapon(&character);
            characters.push(character);
            weapons.push(weapon);
        }

        let team: Team<SimpleAttributeGraph2> = input.team.to_team();
        let mut default_target_functions = team.get_default_target_functions();
        if let Some(ref tfs) = input.override_target_functions {
            for (index, maybe_target_function) in tfs.iter().enumerate() {
                if let Some(tf) = maybe_target_function {
                    let character_name = input.team.characters[index].name;
                    let target_function = tf.to_target_function(&characters[index], &weapons[index]);
                    default_target_functions.insert(character_name as usize, target_function);
                }
            }
        }

        let mut target_functions: Vec<Box<dyn TargetFunction>> = Vec::new();
        for i in 0..characters.len() {
            let name = characters[i].common_data.name;
            let tf = default_target_functions.remove(&(name as usize)).unwrap();
            target_functions.push(tf);
        }

        let artifacts_ref: Vec<&Artifact> = input.artifacts.iter().collect();
        let characters_ref: Vec<&Character<SimpleAttributeGraph2>> = characters.iter().collect();
        let weapons_ref: Vec<&Weapon<SimpleAttributeGraph2>> = weapons.iter().collect();
        let target_functions_ref: Vec<&Box<dyn TargetFunction>> = target_functions.iter().collect();

        let hyper_param = match input.hyper_param {
            Some(x) => x,
            None => TeamOptimizeHyperParam::default()
        };
        let result_raw = optimize_team(
            &artifacts_ref,
            &characters_ref,
            &weapons_ref,
            &target_functions_ref,
            &hyper_param
        );

        let artifacts_by_id = artifacts_by_id_hashmap(&artifacts_ref);
        let mut results: Vec<OptimizeTeamResultEntry> = Vec::new();
        for entry in result_raw.iter() {
            results.push(smallvec_to_optimize_entry(entry, &artifacts_by_id));
        }

        let ret = OptimizeTeamResult {
            artifacts: results
        };

        JsValue::from_serde(&ret).unwrap()
    }
}
