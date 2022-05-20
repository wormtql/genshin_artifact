use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult, OptimizeArtifactInterface};
use mona::artifacts::{Artifact, ArtifactList, ArtifactSlotName};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{AttributeUtils, SimpleAttributeGraph2, AttributeCommon, AttributeName, Attribute};
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::utils;
use mona::weapon::Weapon;

pub fn optimize_single_interface_wasm(input: &OptimizeArtifactInterface, artifacts: &[&Artifact], algo: &Box<dyn SingleOptimizeAlgorithm>, count: usize) -> Vec<OptimizationResult> {
    let character = input.character.to_character();
    let weapon = input.weapon.to_weapon(&character);
    let target_function = input.target_function.to_target_function(&character, &weapon);
    // let constraint_ref = input.constraint.as_ref();
    let buffs: Vec<Box<dyn Buff<SimpleAttributeGraph2>>> = input.buffs.iter().map(|x| x.to_buff()).collect();
    let artifact_config = input.artifact_config.as_ref().map(|x| x.clone().to_config());

    let default_constraint = ConstraintConfig::default();
    let constraint = input.constraint.as_ref().unwrap_or(&default_constraint);

    let filtered_artifacts = input.filter.as_ref().map(|x| x.filter_artifact(artifacts));
    let artifacts = match filtered_artifacts {
        Some(ref a) => a.as_slice(),
        None => artifacts
    };

    let result = algo.optimize(
        &artifacts,
        artifact_config,
        &character,
        &weapon,
        &target_function,
        &Default::default(),
        &buffs,
        &constraint,
        count
    );

    result
}
