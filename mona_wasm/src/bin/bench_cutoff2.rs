use std::time;

use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::attribute::{AttributeName, AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::character::{Character, CharacterName};
use mona::character::character_config::CharacterConfig;
use mona::character::skill_config::CharacterSkillConfig;
use mona::common::{ChangeAttribute, Element, SkillType, StatName};
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils};
use mona::target_functions::target_functions::get_target_function;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use mona_wasm::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use mona_wasm::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use mona_wasm::applications::optimize_artifacts::algorithms::cutoff_algo2::{CutoffAlgo2, CutoffAlgo2Helper};
use mona_wasm::applications::optimize_artifacts::algorithms::weight_heuristic::{NaiveWeightHeuristic, WeightHeuristicAlgorithm};
use mona_wasm::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode};

fn get_character() -> Character<SimpleAttributeGraph2> {
    Character::new(
        CharacterName::Amber,
        90,
        false,
        0,
        8,
        8,
        8,
        &CharacterConfig::NoConfig
    )
}

fn get_weapon(character: &Character<SimpleAttributeGraph2>) -> Weapon<SimpleAttributeGraph2> {
    Weapon::new(
        WeaponName::FesteringDesire,
        90,
        false,
        1,
        &WeaponConfig::NoConfig,
        &character
    )
}

fn create_target_function(character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>) -> Box<dyn TargetFunction> {
    get_target_function(
        TargetFunctionName::AmberDefault, &character, &weapon, &TargetFunctionConfig::NoConfig
    )
}

fn generate_artifacts() -> Vec<Artifact> {
    let mut artifacts = vec![];
    for _ in 0..50 {
        artifacts.push(Artifact::new_random(ArtifactSlotName::Flower));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Feather));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Sand));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Goblet));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Head));
    }

    artifacts
}

fn get_enemy() -> Enemy {
    Enemy::default()
}

fn main() {
    let character = get_character();
    let weapon = get_weapon(&character);
    let target_function = create_target_function(&character, &weapon);
    let enemy = get_enemy();

    let artifacts = generate_artifacts();
    let artifacts_ref: Vec<&Artifact> = artifacts.iter().collect();

    let now = time::SystemTime::now();

    let constraint = ConstraintConfig {
        set_mode: Some(ConstraintSetMode::Any),
        // set_mode: Some(ConstraintSetMode::Set4(ArtifactSetName::BlizzardStrayer)),
        hp_min: None,
        atk_min: None,
        def_min: None,
        recharge_min: None,
        em_min: None,
        crit_min: None,
        crit_dmg_min: None
    };

    let algo = CutoffAlgo2 { accuracy_factor: 1.0 };
    // let algo = AStarCutoff;
    // let algo = CutoffAlgorithmHeuristic {
    //     use_heuristic: true,
    // };
    let results = algo.optimize(
        &artifacts_ref,
        None,
        &character,
        &weapon,
        &target_function,
        &enemy,
        &[],
        &constraint,
        5
    );

    println!("{:?}", results);

    println!("{}ms", now.elapsed().unwrap().as_millis());

    let weight_heu_algo = NaiveWeightHeuristic {
        character: &character,
        weapon: &weapon
    };

    let weights = weight_heu_algo.generate_stat(&target_function);
    println!("{:?}", weights);
}
