use std::time;
use mona::applications::bonus_per_stat::bonus_per_stat::{bonus_per_stat, BonusPerStatInput};

use mona::attribute::{AttributeName, AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::common::{ChangeAttribute, StatName, Element, SkillType};
use mona::weapon::{WeaponName, WeaponConfig, Weapon};
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::character::character_config::CharacterConfig;
use mona::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils};
use mona::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::applications::calculator::interface_calculator::CalculatorInterface;
use mona::applications::optimize_artifacts::algorithms::CutoffAlgorithm2;
use mona::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode};
use mona::character::{Character, CharacterName};
use mona::character::skill_config::CharacterSkillConfig;
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona::target_functions::target_functions::get_target_function;
use mona::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use mona::applications::optimize_artifacts::algorithms::cutoff_heuristic::CutoffAlgorithmHeuristic;

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

    let algo = CutoffAlgorithm2;
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

    println!("{}s", now.elapsed().unwrap().as_secs())
}
