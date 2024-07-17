use std::collections::HashMap;
use smallvec::SmallVec;
use wasm_bindgen::prelude::*;
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use crate::applications::optimize_artifacts::algorithms::cutoff_heuristic::CutoffAlgorithmHeuristic;
use crate::applications::team_optimize::hyper_param::TeamOptimizeHyperParam;
use crate::applications::team_optimize::inter::{OptimizeTeamInterface2, OptimizeTeamObject, OptimizeTeamResult, OptimizeTeamResultEntry, TeamInterface};
use crate::applications::team_optimize::team_optimize::{optimize_team_multi_single};
use mona::artifacts::{Artifact, ArtifactSlotName};
use mona::attribute::SimpleAttributeGraph2;
use mona::buffs::Buff;
use mona::character::Character;
use mona::character::characters::get_target_function_by_role;
use mona::target_functions::TargetFunction;
use mona::team::team::Team;
// use mona::team_target::team_target_config::TeamTargetFunctionConfig;
// use mona::team_target::team_targets::{get_default_buff, try_get_team_target_function, try_match_team};
use mona::weapon::Weapon;
use mona::utils;
use serde::Serialize;

pub struct TeamOptimizationWasm;

fn artifacts_by_id_hashmap<'a>(artifacts: &[&'a Artifact]) -> HashMap<u64, &'a Artifact> {
    let mut temp = HashMap::new();
    for &art in artifacts.iter() {
        temp.insert(art.id, art);
    }
    temp
}

fn smallvec_to_optimize_entry(v: &SmallVec<[u64; 5]>, artifacts_by_id: &HashMap<u64, &Artifact>) -> OptimizeTeamResultEntry {
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
    pub fn optimize_team2(val: JsValue, artifacts: JsValue) -> JsValue {
        utils::set_panic_hook();

        let mut input: OptimizeTeamInterface2 = serde_wasm_bindgen::from_value(val).unwrap();

        let artifacts: Vec<Artifact> = serde_wasm_bindgen::from_value(artifacts).unwrap();

        let artifacts_ref: Vec<&Artifact> = artifacts.iter().collect();
        let hyper_param = match input.hyper_param {
            Some(x) => x,
            None => TeamOptimizeHyperParam::default()
        };

        // let single_algo: Box<dyn SingleOptimizeAlgorithm> = Box::new(CutoffAlgorithmHeuristic { use_heuristic: true });
        let single_algo: Box<dyn SingleOptimizeAlgorithm> = Box::new(AStarCutoff);
        let result_raw = optimize_team_multi_single(
            &artifacts_ref,
            &input.single_interfaces,
            &input.weights,
            &hyper_param,
            &single_algo
        );

        let artifacts_by_id = artifacts_by_id_hashmap(&artifacts_ref);
        let mut results: Vec<Vec<OptimizeTeamResultEntry>> = Vec::new();
        for entry in result_raw.iter() {
            let mut temp = Vec::new();
            for member_index in 0..entry.len() {
                temp.push(smallvec_to_optimize_entry(&entry[member_index], &artifacts_by_id));
            }
            results.push(temp);
        }

        let ret = OptimizeTeamResult {
            artifacts: results
        };

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        ret.serialize(&s).unwrap()
    }

    // pub fn optimize_team(val: &JsValue) -> JsValue {
    //     utils::set_panic_hook();
    //     let input: OptimizeTeamObject = val.into_serde().unwrap();
    //
    //     let mut characters: Vec<Character<SimpleAttributeGraph2>> = Vec::new();
    //     let mut weapons: Vec<Weapon<SimpleAttributeGraph2>> = Vec::new();
    //     let mut target_functions = Vec::new();
    //     let mut buffs: Vec<Vec<Box<dyn Buff<SimpleAttributeGraph2>>>> = Vec::new();
    //
    //     for i in 0..input.characters.len() {
    //         let character = input.characters[i].to_character();
    //         let weapon = input.weapons[i].to_weapon(&character);
    //         let target_function = input.target_functions[i].to_target_function(&character, &weapon);
    //
    //         characters.push(character);
    //         weapons.push(weapon);
    //         target_functions.push(target_function);
    //
    //         buffs.push(input.buffs[i].iter().map(|x| x.to_buff()).collect());
    //     }
    //
    //     // let team: Team<SimpleAttributeGraph2> = input.team.to_team();
    //
    //     // let maybe_team_name = try_match_team(&team);
    //     // let is_team_preset = maybe_team_name.is_some();
    //
    //     // let team_name = maybe_team_name.unwrap();
    //     // let team_target_function_config = input.team_target_function_config.unwrap_or(TeamTargetFunctionConfig::NoConfig);
    //
    //     // let team_target_function = team_name.create(&team_target_function_config, &team);
    //     // let mut buffs_map = get_default_buff(team_name, &team);
    //     // let mut buffs: Vec<Vec<Box<dyn Buff<SimpleAttributeGraph2>>>> = Vec::new();
    //     // for i in 0..characters.len() {
    //     //     let name_usize = characters[i].common_data.name as usize;
    //     //     let has_buff = buffs_map.contains_key(&name_usize);
    //     //     if has_buff {
    //     //         buffs.push(buffs_map.remove(&name_usize).unwrap());
    //     //     } else {
    //     //         buffs.push(Vec::new());
    //     //     }
    //     // }
    //
    //     // let mut default_target_functions = team.get_default_target_functions();
    //     // if let Some(ref tfs) = input.override_target_functions {
    //     //     for (index, maybe_target_function) in tfs.iter().enumerate() {
    //     //         if let Some(tf) = maybe_target_function {
    //     //             let character_name = input.team.characters[index].name;
    //     //             let target_function = tf.to_target_function(&characters[index], &weapons[index]);
    //     //             default_target_functions.insert(character_name as usize, target_function);
    //     //         }
    //     //     }
    //     // }
    //
    //     // let mut individual_targets = team_target_function.get_default_individual_targets();
    //     // let mut target_functions: Vec<Box<dyn TargetFunction>> = Vec::new();
    //     // for i in 0..characters.len() {
    //     //     // let name = characters[i].common_data.name;
    //     //     // let tf = default_target_functions.remove(&(name as usize)).unwrap();
    //     //     // target_functions.push(tf);
    //     //     let name_usize = characters[i].common_data.name as usize;
    //     //     let has_individual_target = individual_targets.contains_key(&name_usize);
    //     //     if has_individual_target {
    //     //         target_functions.push(individual_targets.remove(&name_usize).unwrap());
    //     //     } else {
    //     //         todo!() // todo default or user input target function
    //     //         // let tf =
    //     //         // get_target_function_by_role(0, )
    //     //     }
    //     // }
    //
    //     let artifacts_ref: Vec<&Artifact> = input.artifacts.iter().collect();
    //     // let characters_ref: Vec<&Character<SimpleAttributeGraph2>> = characters.iter().collect();
    //     // let weapons_ref: Vec<&Weapon<SimpleAttributeGraph2>> = weapons.iter().collect();
    //     // let target_functions_ref: Vec<&Box<dyn TargetFunction>> = target_functions.iter().collect();
    //     let hyper_param = match input.hyper_param {
    //         Some(x) => x,
    //         None => TeamOptimizeHyperParam::default()
    //     };
    //     let result_raw = optimize_team(
    //         &artifacts_ref,
    //         &characters,
    //         &weapons,
    //         &buffs,
    //         &target_functions,
    //         &input.constraints,
    //         &input.weights,
    //         &hyper_param
    //     );
    //
    //     let artifacts_by_id = artifacts_by_id_hashmap(&artifacts_ref);
    //     let mut results: Vec<Vec<OptimizeTeamResultEntry>> = Vec::new();
    //     for entry in result_raw.iter() {
    //         let mut temp = Vec::new();
    //         for member_index in 0..entry.len() {
    //             temp.push(smallvec_to_optimize_entry(&entry[member_index], &artifacts_by_id));
    //         }
    //         results.push(temp);
    //     }
    //
    //     let ret = OptimizeTeamResult {
    //         artifacts: results
    //     };
    //
    //     JsValue::from_serde(&ret).unwrap()
    // }

    // pub fn match_name(val: &JsValue) -> JsValue {
    //     utils::set_panic_hook();
    //     let input: OptimizeTeamObject = val.into_serde().unwrap();
    //
    //     let team: Team<SimpleAttributeGraph2> = input.team.to_team();
    //
    //     let maybe_team_name = try_match_team(&team);
    //     let maybe_chs = maybe_team_name.map(|x| x.get_meta().chs);
    //
    //     JsValue::from_serde(&maybe_chs).unwrap()
    // }
}
