use std::collections::{HashMap, HashSet};
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::artifacts::eff::ARTIFACT_EFF5;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::SimpleAttributeGraph2;
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use smallvec::{SmallVec, smallvec};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::algorithms::common::{get_artifacts_group, get_artifacts_group_without_set, get_set_names, get_super_artifacts, get_super_artifacts_without_set, ResultRecorder, ValueFunction};
use crate::applications::optimize_artifacts::algorithms::weight_heuristic::{NaiveWeightHeuristic, WeightHeuristicAlgorithm};
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};

type SimpleSlotName = usize;


pub struct CutoffAlgo2Helper<'a> {
    // make sure vec length is not 0
    pub artifacts: HashMap<(ArtifactSetName, SimpleSlotName, StatName), Vec<&'a Artifact>>,
    pub artifacts_without_set: HashMap<(SimpleSlotName, StatName), Vec<&'a Artifact>>,
    pub artifacts_all: &'a [&'a Artifact],
    pub artifact_sets: Vec<ArtifactSetName>,

    pub super_artifacts: HashMap<(ArtifactSetName, SimpleSlotName, StatName), Artifact>,
    pub super_artifacts_without_set: HashMap<(SimpleSlotName, StatName), Artifact>,

    pub sand_stats: Vec<StatName>,
    pub goblet_stats: Vec<StatName>,
    pub head_stats: Vec<StatName>,
}

#[derive(Copy, Clone)]
pub enum SlotSetName {
    Some(ArtifactSetName),
    Any,
}

impl<'a> CutoffAlgo2Helper<'a> {
    pub fn new(artifacts: &'a [&'a Artifact], weight_heuristic: Option<HashMap<StatName, f64>>, set_heuristics: Option<HashMap<ArtifactSetName, f64>>) -> CutoffAlgo2Helper<'a> {
        let mut sand_stats = HashSet::new();
        let mut goblet_stats = HashSet::new();
        let mut head_stats = HashSet::new();

        for art in artifacts.iter() {
            let slot = art.slot;
            let main_stat = art.main_stat.0;
            if slot == ArtifactSlotName::Sand {
                sand_stats.insert(main_stat);
            } else if slot == ArtifactSlotName::Goblet {
                goblet_stats.insert(main_stat);
            } else if slot == ArtifactSlotName::Head {
                head_stats.insert(main_stat);
            }
        }

        let mut sand_stats: Vec<StatName> = sand_stats.into_iter().collect();
        let mut goblet_stats: Vec<StatName> = goblet_stats.into_iter().collect();
        let mut head_stats: Vec<StatName> = head_stats.into_iter().collect();

        let mut artifacts_group = get_artifacts_group(artifacts);
        let mut artifacts_group_without_set = get_artifacts_group_without_set(artifacts);

        if let Some(ref weights) = weight_heuristic {
            sand_stats.sort_by_cached_key(|x| {
                -(weights.get(x).cloned().unwrap_or(0.0) * 100.0) as usize
            });
            goblet_stats.sort_by_cached_key(|x| {
                -(weights.get(x).cloned().unwrap_or(0.0) * 100.0) as usize
            });
            head_stats.sort_by_cached_key(|x| {
                -(weights.get(x).cloned().unwrap_or(0.0) * 100.0) as usize
            });
            // println!("{:?}", sand_stats);

            for (_, arts) in artifacts_group.iter_mut() {
                arts.sort_by_cached_key(|x| {
                    let mut score = 0.0;
                    for &(stat_name, value) in x.sub_stats.iter() {
                        let weight = weights.get(&stat_name).cloned().unwrap_or(0.0);
                        let eff = value / ARTIFACT_EFF5.get_value(stat_name, 3);
                        score += weight * eff;
                    }
                    -(score * 100.0) as usize
                })
            }
            for (_, arts) in artifacts_group_without_set.iter_mut() {
                arts.sort_by_cached_key(|x| {
                    let mut score = 0.0;
                    for &(stat_name, value) in x.sub_stats.iter() {
                        let weight = weights.get(&stat_name).cloned().unwrap_or(0.0);
                        let eff = value / ARTIFACT_EFF5.get_value(stat_name, 3);
                        score += weight * eff;
                    }
                    -(score * 100.0) as usize
                })
            }
        }

        let mut sets = get_set_names(artifacts);
        if let Some(ref h) = set_heuristics {
            sets.sort_by_key(|x| {
                let v = h.get(x).cloned().unwrap_or(0.0);
                -(v * 100.0) as usize
            })
        }

        CutoffAlgo2Helper {
            artifacts: artifacts_group,
            artifacts_without_set: artifacts_group_without_set,
            artifacts_all: artifacts,
            artifact_sets: sets,
            super_artifacts: get_super_artifacts(artifacts),
            super_artifacts_without_set: get_super_artifacts_without_set(artifacts),

            sand_stats,
            goblet_stats,
            head_stats,
        }
    }

    pub fn get_arts(&self, set_name: SlotSetName, main_stat: StatName, slot: SimpleSlotName) -> Option<&[&Artifact]> {
        match set_name {
            SlotSetName::Some(x) => {
                if let Some(arts) = self.artifacts.get(&(x, slot, main_stat)) {
                    Some(&arts)
                } else {
                    None
                }
            },
            SlotSetName::Any => {
                if let Some(arts) = self.artifacts_without_set.get(&(slot, main_stat)) {
                    Some(&arts)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_super_art(&self, set_name: SlotSetName, main_stat: StatName, slot: SimpleSlotName) -> Option<&Artifact> {
        match set_name {
            SlotSetName::Some(x) => self.super_artifacts.get(&(x, slot, main_stat)),
            SlotSetName::Any => self.super_artifacts_without_set.get(&(slot, main_stat)),
        }
    }

    pub fn is_better_than_current_least(&self, arts: &[&Artifact], value_fn: &ValueFunction, rc: &ResultRecorder) -> bool {
        let score = value_fn.score(&arts);
        let current_least = rc.current_least();
        score > current_least * 1.2
    }

    pub fn update_artifacts(&self, arts: &[&Artifact], value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        let score = value_fn.score(&arts);

        let art_ids = [
            arts[0].id,
            arts[1].id,
            arts[2].id,
            arts[3].id,
            arts[4].id,
        ];
        rc.push_result(art_ids, score);
    }

    pub fn do_iter(&self, set_names: &[SlotSetName], main_stats: &[StatName], value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        let arts: SmallVec<[&[&Artifact]; 5]> = {
            let mut temp: SmallVec<[&[&Artifact]; 5]> = SmallVec::new();
            for i in 0..5 {
                let maybe_arts = self.get_arts(set_names[i], main_stats[i], i);
                if maybe_arts.is_none() {
                    return;
                }
                temp.push(maybe_arts.unwrap());
            }
            // temp.sort_by_cached_key(|x| {
            //     x.len()
            // });
            temp
        };

        for i0 in 0..arts[0].len() {
            {
                let mut super_arts: SmallVec<[&Artifact; 5]> = SmallVec::new();
                super_arts.push(arts[0][i0]);

                for i in 1..5 {
                    if let Some(super_art) = self.get_super_art(set_names[i], main_stats[i], i) {
                        super_arts.push(super_art);
                    } else {
                        return;
                    }
                }

                if !self.is_better_than_current_least(&super_arts, value_fn, rc) {
                    continue;
                }
            }

            for i1 in 0..arts[1].len() {
                {
                    let mut super_arts: SmallVec<[&Artifact; 5]> = SmallVec::new();
                    super_arts.push(arts[0][i0]);
                    super_arts.push(arts[1][i1]);

                    for i in 2..5 {
                        if let Some(super_art) = self.get_super_art(set_names[i], main_stats[i], i) {
                            super_arts.push(super_art);
                        } else {
                            return;
                        }
                    }

                    if !self.is_better_than_current_least(&super_arts, value_fn, rc) {
                        continue;
                    }
                }

                for i2 in 0..arts[2].len() {
                    {
                        let mut super_arts: SmallVec<[&Artifact; 5]> = smallvec![
                            arts[0][i0],
                            arts[1][i1],
                            arts[2][i2],
                        ];

                        for i in 3..5 {
                            if let Some(super_art) = self.get_super_art(set_names[i], main_stats[i], i) {
                                super_arts.push(super_art);
                            } else {
                                return;
                            }
                        }

                        if !self.is_better_than_current_least(&super_arts, value_fn, rc) {
                            continue;
                        }
                    }

                    for i3 in 0..arts[3].len() {
                        {
                            let mut super_arts: SmallVec<[&Artifact; 5]> = smallvec![
                                arts[0][i0],
                                arts[1][i1],
                                arts[2][i2],
                                arts[3][i3],
                            ];

                            for i in 4..5 {
                                if let Some(super_art) = self.get_super_art(set_names[i], main_stats[i], i) {
                                    super_arts.push(super_art);
                                } else {
                                    return;
                                }
                            }

                            if !self.is_better_than_current_least(&super_arts, value_fn, rc) {
                                continue;
                            }
                        }

                        for i4 in 0..arts[4].len() {
                            let artifacts: SmallVec<[&Artifact; 5]> = smallvec![
                                arts[0][i0],
                                arts[1][i1],
                                arts[2][i2],
                                arts[3][i3],
                                arts[4][i4],
                            ];

                            self.update_artifacts(&artifacts, value_fn, rc);
                        }
                    }
                }
            }
        }
    }

    pub fn iter_set(&mut self, set_mask: &[[i32; 5]], s1: ArtifactSetName, s2: ArtifactSetName, value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        for &sand in self.sand_stats.iter() {
            for &goblet in self.goblet_stats.iter() {
                for &head in self.head_stats.iter() {
                    let main_stats = [StatName::HPFixed, StatName::ATKFixed, sand, goblet, head];
                    'outer: for set_composition in set_mask.iter() {
                        {
                            let mut super_artifacts: SmallVec<[&Artifact; 5]> = SmallVec::new();
                            for (index, &mask) in set_composition.iter().enumerate() {
                                let slot_set_name = if mask == 0 {
                                    SlotSetName::Any
                                } else if mask == 1 {
                                    SlotSetName::Some(s1)
                                } else {
                                    SlotSetName::Some(s2)
                                };

                                let super_art = if let Some(x) = self.get_super_art(slot_set_name, main_stats[index], index) {
                                    x
                                } else {
                                    continue 'outer;
                                };

                                super_artifacts.push(super_art);
                            }

                            if !self.is_better_than_current_least(&super_artifacts, value_fn, rc) {
                                continue 'outer;
                            }
                        }

                        // do iter
                        let set_names = {
                            let mut temp: SmallVec<[SlotSetName; 5]> = SmallVec::new();
                            for &mask in set_composition.iter() {
                                if mask == 0 {
                                    temp.push(SlotSetName::Any);
                                } else if mask == 1 {
                                    temp.push(SlotSetName::Some(s1));
                                } else {
                                    temp.push(SlotSetName::Some(s2));
                                }
                            }
                            temp
                        };

                        self.do_iter(&set_names, &main_stats, value_fn, rc);
                    }
                }
            }
        }

    }

    pub fn iter_set4(&mut self, set_name: ArtifactSetName, value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        let set_masks = [
            [0, 1, 1, 1, 1], [1, 0, 1, 1, 1], [1, 1, 0, 1, 1], [1, 1, 1, 0, 1], [1, 1, 1, 1, 0]
        ];

        // self.iter_set(&set_masks, set_name, ArtifactSetName::Empty, main_stats, value_fn, rc);
        self.iter_set(&set_masks, set_name, ArtifactSetName::Empty, value_fn, rc);
    }

    pub fn iter_set22(&mut self, s1: ArtifactSetName, s2: ArtifactSetName, value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        let set_mask = [
            [0, 1, 1, 2, 2], [0, 1, 2, 1, 2], [0, 1, 2, 2, 1], [0, 2, 1, 1, 2], [0, 2, 1, 2, 1], [0, 2, 2, 1, 1],
            [1, 0, 1, 2, 2], [1, 0, 2, 1, 2], [1, 0, 2, 2, 1], [2, 0, 1, 1, 2], [2, 0, 1, 2, 1], [2, 0, 2, 1, 1],
            [1, 1, 0, 2, 2], [1, 2, 0, 1, 2], [1, 2, 0, 2, 1], [2, 1, 0, 1, 2], [2, 1, 0, 2, 1], [2, 2, 0, 1, 1],
            [1, 1, 2, 0, 2], [1, 2, 1, 0, 2], [1, 2, 2, 0, 1], [2, 1, 1, 0, 2], [2, 1, 2, 0, 1], [2, 2, 1, 0, 1],
            [1, 1, 2, 2, 0], [1, 2, 1, 2, 0], [1, 2, 2, 1, 0], [2, 1, 1, 2, 0], [2, 1, 2, 1, 0], [2, 2, 1, 1, 0],
        ];

        // self.iter_set(&set_mask, s1, s2, main_stats, value_fn, rc);
        self.iter_set(&set_mask, s1, s2, value_fn, rc);
    }

    pub fn iter_set2(&mut self, set_name: ArtifactSetName, value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        let set_masks = [
            [1, 1, 0, 0, 0], [1, 0, 1, 0, 0], [1, 0, 0, 1, 0], [1, 0, 0, 0, 1],
            [0, 1, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 0, 0, 1],
            [0, 0, 1, 1, 0], [0, 0, 1, 0, 1],
            [0, 0, 0, 1, 1]
        ];

        // self.iter_set(&set_masks, set_name, ArtifactSetName::Empty, main_stats, value_fn, rc);
        self.iter_set(&set_masks, set_name, ArtifactSetName::Empty, value_fn, rc);
    }

    pub fn iter_any(&mut self, value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        // let mut set_count = [0; ArtifactSetName::LEN];

        // self.iter_set(&[[0, 0, 0, 0, 0]], ArtifactSetName::Empty, ArtifactSetName::Empty, main_stats, value_fn, rc);
        self.iter_set(&[[0, 0, 0, 0, 0]], ArtifactSetName::Empty, ArtifactSetName::Empty, value_fn, rc);
    }

    pub fn do_calculation(&mut self, value_fn: &ValueFunction, rc: &mut ResultRecorder) {
        // let set_names = get_set_names(&self.artifacts_all);
        let set_names = self.artifact_sets.clone();

        // for &sand in stats_sand.iter() {
        //     for &goblet in stats_goblet.iter() {
        //         for &head in stats_head.iter() {
        //             let stats = [StatName::HPFixed, StatName::ATKFixed, sand, goblet, head];

                    for &set_name in set_names.iter() {
                        // self.iter_set4(set_name, &stats, value_fn, rc);
                        self.iter_set4(set_name, value_fn, rc);
                    }

                    for i in 0..set_names.len() {
                        let s1 = set_names[i];

                        // self.iter_set4(s1, value_fn, rc);

                        for j in i..set_names.len() {
                            let s2 = set_names[j];
                            // self.iter_set22(s1, s2, &stats, value_fn, rc);
                            self.iter_set22(s1, s2, value_fn, rc);
                        }
                    }

                    for &s1 in set_names.iter() {
                        // self.iter_set2(s1, &stats, value_fn, rc);
                        self.iter_set2(s1, value_fn, rc);
                    }

                    // self.iter_any(&stats, value_fn, rc);
                    self.iter_any(value_fn, rc);
                // }
            // }
        // }
    }
}

pub struct CutoffAlgo2;

impl SingleOptimizeAlgorithm for CutoffAlgo2 {
    fn optimize(&self, artifacts: &[&Artifact], artifact_config: Option<ArtifactEffectConfig>, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, target_function: &Box<dyn TargetFunction>, enemy: &Enemy, buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>], constraint: &ConstraintConfig, count: usize) -> Vec<OptimizationResult> {
        let mut result_recorder = ResultRecorder::new(count);

        let mut default_effect_config: ArtifactEffectConfig;
        let effect_config = if let Some(ref x) = artifact_config {
            x
        } else {
            default_effect_config = target_function.get_default_artifact_config(&Default::default());
            &default_effect_config
        };

        let value_function = ValueFunction {
            artifact_effect_config: &effect_config,
            character: &character,
            weapon: &weapon,
            target_function: &target_function,
            buffs: &buffs,
            enemy: &enemy,
            constraint: &constraint
        };

        let weight_heuristic_algo = NaiveWeightHeuristic {
            character,
            weapon
        };
        let weight_heuristic = weight_heuristic_algo.generate_stat(&target_function);
        let set_heuristic = weight_heuristic_algo.generate_set(&target_function);

        let mut algo = CutoffAlgo2Helper::new(
            artifacts,
            Some(weight_heuristic),
            Some(set_heuristic)
            // None,
            // None,
        );
        algo.do_calculation(&value_function, &mut result_recorder);

        let intermediate_results = result_recorder.get_results_descend();
        let max_score = intermediate_results.iter().map(|x| x.score)
            .fold(f64::NEG_INFINITY, f64::max);

        let mut result = Vec::new();
        for item in intermediate_results.iter() {
            result.push(item.to_result(max_score));
        }

        result
    }
}