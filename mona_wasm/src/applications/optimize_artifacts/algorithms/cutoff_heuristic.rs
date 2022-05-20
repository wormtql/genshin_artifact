use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use serde::__private::ser::constrain;
use mona::attribute::{Attribute, AttributeCommon, AttributeName, AttributeUtils};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult};
use mona::artifacts::{Artifact, ArtifactList, ArtifactSlotName};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::SimpleAttributeGraph2;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use mona::utils;
use mona::utils::artifact::get_per_slot_artifacts;
use mona::log;

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

pub struct CutoffAlgorithmHeuristic {
    pub use_heuristic: bool,
}

impl SingleOptimizeAlgorithm for CutoffAlgorithmHeuristic {
    fn optimize(&self, artifacts: &[&Artifact], artifact_config: Option<ArtifactEffectConfig>, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, target_function: &Box<dyn TargetFunction>, enemy: &Enemy, buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>], constraint: &ConstraintConfig, count: usize) -> Vec<OptimizationResult> {
        let need_constraint = !constraint.is_any();

        let mut enemy = enemy.clone();
        // buff change enemy
        for b in buffs.iter() {
            b.change_enemy(&mut enemy);
        }

        let artifact_config = if let Some(x) = artifact_config {
            x
        } else {
            target_function.get_default_artifact_config(&Default::default())
        };

        let mut artifacts: Vec<&Artifact> = artifacts.iter().map(|x| *x).collect();
        if get_iteration_count(&artifacts) > TOO_LARGE_ITER_COUNT && !need_constraint && self.use_heuristic {
            let target_function_opt_config = target_function.get_target_function_opt_config();
            artifacts = target_function_opt_config.filter(artifacts);
        }
        let iter_count = get_iteration_count(&artifacts);
        // log!("iter count: {}", iter_count);

        let (flowers, feathers, sands, goblets, heads) = get_per_slot_artifacts(&artifacts);

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
                    if !check_artifact_set(&check_artifact_set_buffer, constraint) {
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
                        if !check_artifact_set(&check_artifact_set_buffer, constraint) {
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
                            if !check_artifact_set(&check_artifact_set_buffer, constraint) {
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
                                if !check_artifact_set(&check_artifact_set_buffer, constraint) {
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

                            if !check_attribute(&attribute, constraint) {
                                continue;
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
}
