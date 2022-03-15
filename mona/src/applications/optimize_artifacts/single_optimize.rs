use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult, OptimizeArtifactInterface};
use crate::artifacts::{Artifact, ArtifactList, ArtifactSlotName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeUtils, SimpleAttributeGraph2, AttributeCommon, AttributeName, Attribute};
use crate::buffs::Buff;
use crate::character::Character;
use crate::enemies::Enemy;
use crate::target_functions::TargetFunction;
use crate::utils;
use crate::weapon::Weapon;

struct OptimizationIntermediateResult {
    flower_index: usize,
    feather_index: usize,
    sand_index: usize,
    goblet_index: usize,
    head_index: usize,
    value: f64,
}

impl PartialEq for OptimizationIntermediateResult {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for OptimizationIntermediateResult {}

impl PartialOrd for OptimizationIntermediateResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for OptimizationIntermediateResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

fn get_iteration_count(artifacts: &[&Artifact]) -> usize {
    let mut map: HashMap<ArtifactSlotName, usize> = HashMap::new();

    for art in artifacts.iter() {
        *map.entry(art.slot).or_insert(0) += 1;
    }

    map.values().fold(1, |x, y| x.max(1) * (*y).max(1))
}

fn check_artifact_set(list: &[&Artifact], constraint: &ConstraintConfig) -> bool {
    let mut set_name_count: [usize; 100] = [0; 100];

    let mut count = 0;
    match constraint.set_mode {
        None => return true,
        Some(ref mode) => {
            match *mode {
                ConstraintSetMode::Any => return true,
                ConstraintSetMode::Set2(set_name) => {
                    set_name_count[set_name as usize] = 2;
                    count = 2;
                },
                ConstraintSetMode::Set22(s1, s2) => {
                    set_name_count[s1 as usize] = 2;
                    set_name_count[s2 as usize] = 2;
                    count = 4;
                },
                ConstraintSetMode::Set4(s1) => {
                    set_name_count[s1 as usize] = 4;
                    count = 4;
                }
            }
        }
    }

    for artifact in list.iter() {
        let temp = artifact.set_name as usize;
        if set_name_count[temp] > 0 {
            set_name_count[temp] -= 1;
            count -= 1;
        }
    }

    5 - list.len() >= count
}

// #[inline]
fn check_attribute(attribute: &SimpleAttributeGraph2, constraint: &ConstraintConfig) -> bool {
    if attribute.get_atk() < constraint.atk_min.unwrap_or(0.0) {
        return false;
    }
    if attribute.get_def() < constraint.def_min.unwrap_or(0.0) {
        return false;
    }
    if attribute.get_hp() < constraint.hp_min.unwrap_or(0.0) {
        return false;
    }
    if attribute.get_value(AttributeName::ElementalMastery) < constraint.em_min.unwrap_or(0.0) {
        return false;
    }
    if attribute.get_value(AttributeName::Recharge) < constraint.recharge_min.unwrap_or(0.0) {
        return false;
    }
    let critical = attribute.get_value(AttributeName::CriticalBase).clamp(0.0, 1.0);
    if critical < constraint.crit_min.unwrap_or(0.0) {
        return false;
    }
    if attribute.get_value(AttributeName::CriticalDamageBase) < constraint.crit_dmg_min.unwrap_or(0.0) {
        return false;
    }

    true
}

const TOO_LARGE_ITER_COUNT: usize = 1000000;

pub fn optimize_single(
    artifacts: &[&Artifact],
    artifact_config: Option<ArtifactEffectConfig>,
    character: &Character<SimpleAttributeGraph2>,
    weapon: &Weapon<SimpleAttributeGraph2>,
    target_function: &Box<dyn TargetFunction>,
    constraint: Option<&ConstraintConfig>,
    buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>],
    count: usize,
    use_optim: bool
) -> Vec<OptimizationResult> {
    let need_constraint = if let Some(c) = constraint {
        !c.is_any()
    } else {
        false
    };

    let mut enemy = Enemy::default();

    // buff change enemy
    for b in buffs.iter() {
        b.change_enemy(&mut enemy);
    }

    let artifact_config = if let Some(x) = artifact_config {
        x
    } else {
        target_function.get_default_artifact_config(&Default::default())
    };

    let mut flowers: Vec<&Artifact> = vec![];
    let mut feathers: Vec<&Artifact> = vec![];
    let mut sands: Vec<&Artifact> = vec![];
    let mut goblets: Vec<&Artifact> = vec![];
    let mut heads: Vec<&Artifact> = vec![];

    let mut artifacts: Vec<&Artifact> = artifacts.iter().map(|x| *x).collect();
    if get_iteration_count(&artifacts) > TOO_LARGE_ITER_COUNT && !need_constraint && use_optim {
        let target_function_opt_config = target_function.get_target_function_opt_config();
        artifacts = target_function_opt_config.filter(artifacts);
    }
    let iter_count = get_iteration_count(&artifacts);
    utils::log!("iter count: {}", iter_count);

    // construct artifact array of each slot
    for &artifact in artifacts.iter() {
        match artifact.slot {
            ArtifactSlotName::Flower => flowers.push(artifact),
            ArtifactSlotName::Feather => feathers.push(artifact),
            ArtifactSlotName::Sand => sands.push(artifact),
            ArtifactSlotName::Goblet => goblets.push(artifact),
            ArtifactSlotName::Head => heads.push(artifact),
        }
    }

    let mut buffer_artifacts: Vec<&Artifact> = Vec::with_capacity(5);
    let mut check_artifact_set_buffer: Vec<&Artifact> = Vec::with_capacity(5);
    let mut heap = BinaryHeap::with_capacity(count + 1);
    for flower_i in 0..flowers.len().max(1) {
        for feather_i in 0..feathers.len().max(1) {
            if need_constraint {
                check_artifact_set_buffer.clear();
                if flower_i != flowers.len() {
                    check_artifact_set_buffer.push(&flowers[flower_i]);
                }
                if feather_i != feathers.len() {
                    check_artifact_set_buffer.push(&feathers[feather_i]);
                }
                if !check_artifact_set(&check_artifact_set_buffer, constraint.unwrap()) {
                    continue;
                }
            }

            for sand_i in 0..sands.len().max(1) {
                if need_constraint {
                    check_artifact_set_buffer.clear();
                    if flower_i != flowers.len() {
                        check_artifact_set_buffer.push(&flowers[flower_i]);
                    }
                    if feather_i != feathers.len() {
                        check_artifact_set_buffer.push(&feathers[feather_i]);
                    }
                    if sand_i != sands.len() {
                        check_artifact_set_buffer.push(&sands[sand_i]);
                    }
                    if !check_artifact_set(&check_artifact_set_buffer, constraint.unwrap()) {
                        continue;
                    }
                }

                for goblet_i in 0..goblets.len().max(1) {
                    if need_constraint {
                        check_artifact_set_buffer.clear();
                        if flower_i != flowers.len() {
                            check_artifact_set_buffer.push(&flowers[flower_i]);
                        }
                        if feather_i != feathers.len() {
                            check_artifact_set_buffer.push(&feathers[feather_i]);
                        }
                        if sand_i != sands.len() {
                            check_artifact_set_buffer.push(&sands[sand_i]);
                        }
                        if goblet_i != goblets.len() {
                            check_artifact_set_buffer.push(&goblets[goblet_i]);
                        }
                        if !check_artifact_set(&check_artifact_set_buffer, constraint.unwrap()) {
                            continue;
                        }
                    }

                    for head_i in 0..heads.len().max(1) {
                        if need_constraint {
                            check_artifact_set_buffer.clear();
                            if flower_i != flowers.len() {
                                check_artifact_set_buffer.push(&flowers[flower_i]);
                            }
                            if feather_i != feathers.len() {
                                check_artifact_set_buffer.push(&feathers[feather_i]);
                            }
                            if sand_i != sands.len() {
                                check_artifact_set_buffer.push(&sands[sand_i]);
                            }
                            if goblet_i != goblets.len() {
                                check_artifact_set_buffer.push(&goblets[goblet_i]);
                            }
                            if head_i != heads.len() {
                                check_artifact_set_buffer.push(&heads[head_i]);
                            }
                            if !check_artifact_set(&check_artifact_set_buffer, constraint.unwrap()) {
                                check_artifact_set_buffer.pop();
                                continue;
                            }
                        }

                        buffer_artifacts.clear();
                        if flower_i < flowers.len() {
                            buffer_artifacts.push(flowers[flower_i]);
                        }
                        if feather_i < feathers.len() {
                            buffer_artifacts.push(feathers[feather_i]);
                        }
                        if sand_i < sands.len() {
                            buffer_artifacts.push(sands[sand_i]);
                        }
                        if goblet_i < goblets.len() {
                            buffer_artifacts.push(goblets[goblet_i]);
                        }
                        if head_i < heads.len() {
                            buffer_artifacts.push(heads[head_i]);
                        }

                        let artifact_list = ArtifactList {
                            // artifacts: &artifacts
                            artifacts: &buffer_artifacts
                        };

                        let mut attribute = AttributeUtils::create_attribute_from_big_config(
                            &artifact_list,
                            &artifact_config,
                            character,
                            weapon,
                            &buffs
                        );

                        if need_constraint {
                            if !check_attribute(&attribute, constraint.unwrap()) {
                                continue;
                            }
                        }

                        let value = target_function.target(&attribute, character, weapon, &buffer_artifacts, &enemy);

                        let intermediate = OptimizationIntermediateResult {
                            flower_index: flower_i,
                            feather_index: feather_i,
                            sand_index: sand_i,
                            goblet_index: goblet_i,
                            head_index: head_i,
                            value,
                        };

                        if heap.len() < count {
                            heap.push(Reverse(intermediate));
                        } else {
                            heap.push(Reverse(intermediate));
                            heap.pop();
                        }
                    }
                }
            }
        }
    }

    let max_value = heap.iter().map(|x| x.0.value).fold(-f64::INFINITY, f64::max);

    let mut results: Vec<OptimizationResult> = vec![];
    for i in heap.iter() {
        let intermediate = &i.0;

        let flower = flowers.get(intermediate.flower_index).map(|x| x.id);
        let feather = feathers.get(intermediate.feather_index).map(|x| x.id);
        let sand = sands.get(intermediate.sand_index).map(|x| x.id);
        let goblet = goblets.get(intermediate.goblet_index).map(|x| x.id);
        let head = heads.get(intermediate.head_index).map(|x| x.id);

        let optimization_result = OptimizationResult {
            flower,
            feather,
            sand,
            goblet,
            head,
            value: intermediate.value,
            ratio: intermediate.value / max_value
        };

        results.push(optimization_result);
    }

    results.sort_by(|x, y| y.value.partial_cmp(&x.value).unwrap());

    results
}

pub fn optimize_single_interface(
    artifacts: &[&Artifact],
    artifact_config: Option<ArtifactEffectConfig>,
    character_interface: &CharacterInterface,
    weapon_interface: &WeaponInterface,
    target_function_interface: &TargetFunctionInterface,
    constraint: Option<&ConstraintConfig>,
    buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>],
    count: usize
) -> Vec<OptimizationResult> {
    let character = character_interface.to_character();
    let weapon = weapon_interface.to_weapon(&character);
    let tf = target_function_interface.to_target_function(&character, &weapon);

    optimize_single(
        artifacts, artifact_config, &character, &weapon, &tf, constraint, buffs, count, true
    )
}

pub fn optimize_single_interface_wasm(input: &OptimizeArtifactInterface, artifacts: &[&Artifact]) -> Vec<OptimizationResult> {
    let character = input.character.to_character();
    let weapon = input.weapon.to_weapon(&character);
    let target_function = input.target_function.to_target_function(&character, &weapon);
    // let artifacts_ref: Vec<&Artifact> = input.artifacts.iter().collect();
    let constraint_ref = input.constraint.as_ref();
    let buffs: Vec<Box<dyn Buff<SimpleAttributeGraph2>>> = input.buffs.iter().map(|x| x.to_buff()).collect();
    let artifact_config = input.artifact_config.as_ref().map(|x| x.clone().to_config());

    let filtered_artifacts = input.filter.as_ref().map(|x| x.filter_artifact(artifacts));
    let artifacts = match filtered_artifacts {
        Some(ref a) => a.as_slice(),
        None => artifacts
    };

    let results = optimize_single(
        // &artifacts_ref,
        &artifacts,
        artifact_config,
        &character,
        &weapon,
        &target_function,
        constraint_ref,
        &buffs,
        100,
        input.use_optim
    );

    results
}
