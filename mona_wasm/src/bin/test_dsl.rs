use mona::artifacts::{Artifact, ArtifactSlotName};
use mona::attribute::AttributeUtils;
use mona::character::{Character, CharacterConfig, CharacterName};
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};

use mona_wasm::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use mona_wasm::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use mona_wasm::applications::optimize_artifacts::inter::ConstraintConfig;
use mona_wasm::target_function::dsl_tf::TargetFunctionDSL;

fn generate_artifacts() -> Vec<Artifact> {
    let mut artifacts = vec![];
    for _ in 0..25 {
        artifacts.push(Artifact::new_random(ArtifactSlotName::Flower));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Feather));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Sand));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Goblet));
        artifacts.push(Artifact::new_random(ArtifactSlotName::Head));
    }

    artifacts
}

fn main() {
    let character = Character::new(
        CharacterName::KamisatoAyaka,
        90,
        false,
        0,
        8, 8, 8,
        &CharacterConfig::NoConfig
    );

    let weapon = Weapon::new(
        WeaponName::MistsplitterReforged,
        90,
        false,
        1,
        &WeaponConfig::NoConfig,
        &character
    );

    let target_function: Box<dyn TargetFunction> = Box::new(TargetFunctionDSL::new(r#"
prop x = KamisatoAyaka.recharge
dmg d = KamisatoAyaka.Q1({ after_dash: true })
dmg a = KamisatoAyaka.Normal1({ after_dash: true })
result = 10 * a.n.e + d.n.e * min(x, 1.6)
    "#.trim()));

    let enemy = Enemy::default();

    let constraint = ConstraintConfig::default();

    let artifacts = generate_artifacts();
    let artifacts_ref: Vec<&Artifact> = artifacts.iter().collect();

    let algorithm = AStarCutoff;

    let result = algorithm.optimize(
        &artifacts_ref,
        None,
        &character,
        &weapon,
        &target_function,
        &enemy,
        &Vec::new(),
        &constraint,
        5
    );

    println!("{:?}", result);
}