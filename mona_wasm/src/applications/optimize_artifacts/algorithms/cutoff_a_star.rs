use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::collections::hash_map::{RandomState, DefaultHasher};
use std::hash::{Hash, Hasher};
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::algorithms::cutoff_heuristic::CutoffAlgorithmHeuristic;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult};
use mona::artifacts::{Artifact, ArtifactList, ArtifactSetName, ArtifactSlotName};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{SimpleAttributeGraph2, AttributeCommon, Attribute, AttributeName, AttributeUtils};
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::utils::artifact::get_per_slot_artifacts;
use mona::weapon::Weapon;

struct OptimizationIntermediateResult {
    value: f64,
    arts: [u64; 5],
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

struct ResultRecorder<'a> {
    result_heap: BinaryHeap<Reverse<OptimizationIntermediateResult>>,
    result_count: usize,
    result_set: FxHashSet<u64>,

    artifact_config: ArtifactEffectConfig,
    character: &'a Character<SimpleAttributeGraph2>,
    weapon: &'a Weapon<SimpleAttributeGraph2>,
    target_function: &'a Box<dyn TargetFunction>,
    constraint: &'a ConstraintConfig,
    buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
    enemy: Enemy,
}

impl<'a> ResultRecorder<'a> {
    pub fn new(
        artifact_config: Option<ArtifactEffectConfig>,
        character: &'a Character<SimpleAttributeGraph2>,
        weapon: &'a Weapon<SimpleAttributeGraph2>,
        target_function: &'a Box<dyn TargetFunction>,
        enemy: &'a Enemy,
        constraint: &'a ConstraintConfig,
        buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
        result_count: usize
    ) -> Self {
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

        Self {
            result_heap: BinaryHeap::with_capacity(result_count),
            result_count,
            result_set: FxHashSet::default(),

            artifact_config,
            character,
            weapon,
            target_function,
            constraint,
            buffs,
            enemy,
        }
    }

    fn calc_value(&self, arts: &[&Artifact]) -> Option<f64> {
        let artifact_list = ArtifactList {
            artifacts: arts
        };

        let attribute = AttributeUtils::create_attribute_from_big_config(
            &artifact_list,
            &self.artifact_config,
            self.character,
            self.weapon,
            &self.buffs
        );

        if !check_attribute(&attribute, &self.constraint) {
            return None;
        }

        let value = self.target_function.target(&attribute, self.character, self.weapon, &arts, &self.enemy);
        Some(value)
    }

    // arts id are sorted by slots
    fn arts_to_u64(arts_id: &[u64]) -> u64 {
        let mut s = DefaultHasher::new();
        for &id in arts_id.iter() {
            id.hash(&mut s);
        }
        // arts_id.iter().fold(0_u64, |acc, id| acc * 2000 + id)
        s.finish()
    }

    fn push_result(&mut self, arts: &[&Artifact], value: f64) {
        // utils::log!("push_result {}", value);
        let mut arts = arts.clone();

        let mut slot_ids: SmallVec<[(usize, u64); 5]> = SmallVec::new();
        for art in arts.iter() {
            slot_ids.push((art.slot as usize, art.id));
        }
        slot_ids.sort_by_key(|x| x.0);

        let mut arts_id: [u64; 5] = [0; 5];
        for (index, (_, id)) in slot_ids.iter().enumerate() {
            arts_id[index] = *id;
        }

        // arts.sort_by_key(|art| art.slot as usize);
        // let arts_id: [u64; 5] = arts.iter().map(|art| art.id).collect::<Vec<_>>().try_into().unwrap();
        let hash = Self::arts_to_u64(&arts_id);
        if self.result_set.contains(&hash) {
            return;
        }
        if self.result_heap.len() == self.result_count {
            let old_arts_id = self.result_heap.pop().unwrap().0.arts;
            let old_hash = Self::arts_to_u64(&old_arts_id);
            self.result_set.remove(&old_hash);
        }
        let intermediate = OptimizationIntermediateResult {
            arts: arts_id,
            value,
        };
        self.result_heap.push(Reverse(intermediate));
        self.result_set.insert(hash);
    }

    fn check_hope(&self, arts: &[&Artifact]) -> (bool, f64) {
        if let Some(value) = self.calc_value(&arts) {
            // utils::log!("check_hope {}", value);
            (value > self.result_heap.peek().map_or(0., |r| r.0.value), value)
        } else {
            (false, 0.)
        }
    }

    fn check_hope_option(&self, arts: &[Option<&Artifact>]) -> bool {
        // utils::log!("check_hope_option {}", arts.len());
        if arts.iter().any(|op| op.is_none()) {
            return false;
        }
        let arts: SmallVec<[&Artifact; 5]> = arts.iter().map(|op| op.unwrap()).collect();
        self.check_hope(&arts).0
    }
}

fn merge_art_stat(dst: &mut Artifact, src: &Artifact) {
    dst.main_stat.1 = dst.main_stat.1.max(src.main_stat.1);
    for (stat, value) in src.sub_stats.iter() {
        let mut exists = false;
        for (dstat, dvalue) in dst.sub_stats.iter_mut() {
            if dstat == stat {
                exists = true;
                *dvalue = dvalue.max(*value);
                break;
            }
        }
        if !exists {
            dst.sub_stats.push((*stat, *value));
        }
    }
}

struct SingleOptimizer<'a> {
    artifacts: Vec<&'a Artifact>,
    group_arts: Vec<(Vec<&'a Artifact>, Artifact)>,
    group_map: HashMap<ArtifactSlotName, HashMap<StatName, HashMap<ArtifactSetName, usize>>>,
    constraint: &'a ConstraintConfig,
}

impl<'a> SingleOptimizer<'a> {
    pub fn new(
        artifacts: &[&'a Artifact],
        constraint: &'a ConstraintConfig,
    ) -> Self {
        // group arts by slot, main stat, set; and make an fake super art for each set
        let mut group_arts = Vec::<(Vec<&Artifact>, Artifact)>::new();
        let mut group_map = HashMap::<ArtifactSlotName, HashMap<StatName, HashMap<ArtifactSetName, usize>>>::new();
        for &art in artifacts.iter() {
            let slot_res = group_map.entry(art.slot).or_default();
            let mstat_res = slot_res.entry(art.main_stat.0).or_default();
            let (set_res, super_art) = if let Some(&gi) = mstat_res.get(&art.set_name) {
                &mut group_arts[gi]
            } else {
                group_arts.push((Vec::new(), Artifact::new(
                    art.set_name,
                    art.slot,
                    0,
                    5,
                    Vec::new(),
                    (art.main_stat.0, 0.),
                )));
                let gi = group_arts.len() - 1;
                mstat_res.insert(art.set_name, gi);
                &mut group_arts[gi]
            };
            set_res.push(art);
            merge_art_stat(super_art, &art);
        }
        for (&slot, slot_group) in group_map.iter_mut() {
            for (&mstat, stat_group) in slot_group.iter_mut() {
                let mut super_art = Artifact::new(
                    ArtifactSetName::Empty,
                    slot,
                    0,
                    5,
                    Vec::new(),
                    (mstat, 0.),
                );
                for &gi in stat_group.values() {
                    let (_, set_super_art) = &mut group_arts[gi];
                    merge_art_stat(&mut super_art, set_super_art);
                }
                group_arts.push((Vec::new(), super_art));
                let gi = group_arts.len() - 1;
                stat_group.insert(ArtifactSetName::Empty, gi);
            }
        }

        Self {
            artifacts: artifacts.iter().map(|x| *x).collect(),
            group_arts,
            group_map,
            constraint,
        }
    }

    fn do_enumerate_recursive<'b>(
        &self,
        slot_index: usize,
        group_arts: &Vec<&HashMap<ArtifactSetName, (&Vec<&'b Artifact>, &'b Artifact)>>,
        arts: &mut Vec<&'b Artifact>,
        upper_arts: &mut Vec<&'b Artifact>,
        res_rec: &mut ResultRecorder,
    ) {
        let slot_upper_art = upper_arts[slot_index];
        let slot_group = group_arts[slot_index];
        for (set_arts, set_super_art) in slot_group.values() {
            if set_arts.is_empty() {
                continue;
            }
            upper_arts[slot_index] = set_super_art;
            if !res_rec.check_hope(upper_arts).0 {
                continue;
            }
            for art in set_arts.iter() {
                upper_arts[slot_index] = art;
                let (hope, value) = res_rec.check_hope(upper_arts);
                if !hope {
                    continue;
                }
                arts[slot_index] = art;
                if slot_index == 4 {
                    res_rec.push_result(arts, value);
                } else {
                    self.do_enumerate_recursive(slot_index + 1, group_arts, arts, upper_arts, res_rec);
                }
            }
        }
        upper_arts[slot_index] = slot_upper_art;
    }

    fn do_enumerate(&self, mut group_arts: Vec<&HashMap<ArtifactSetName, (&Vec<&Artifact>, &Artifact)>>, res_rec: &mut ResultRecorder) {
        group_arts.sort_by_cached_key(|hm| {
            hm.values().map(|v| v.0.len()).sum::<usize>()
        });
        let mut upper_arts = group_arts.iter().map(|hm| hm[&ArtifactSetName::Empty].1).collect::<Vec<_>>();
        let mut arts = upper_arts.clone();
        self.do_enumerate_recursive(0, &group_arts, &mut arts, &mut upper_arts, res_rec);
    }

    fn do4(&self, art_sets: &Vec<ArtifactSetName>, res_rec: &mut ResultRecorder) {
        // utils::log!("hello from do4");
        for flower_group in self.group_map[&ArtifactSlotName::Flower].values() {
            for feather_group in self.group_map[&ArtifactSlotName::Feather].values() {
                for sand_group in self.group_map[&ArtifactSlotName::Sand].values() {
                    for goblet_group in self.group_map[&ArtifactSlotName::Goblet].values() {
                        for head_group in self.group_map[&ArtifactSlotName::Head].values() {
                            let groups: Vec<_> = vec![
                                flower_group,
                                feather_group,
                                sand_group,
                                goblet_group,
                                head_group,
                            ].iter().map(|hm| {
                                let new_hm: HashMap<_, _> = hm.iter().map(|(&k, &gi)| {
                                    let (g, sa) = &self.group_arts[gi];
                                    (k, (g, sa))
                                }).collect();
                                new_hm
                            }).collect();
                            for &set in art_sets.iter() {
                                let set_group_arts: Vec<_> = groups.iter().map(|g| {
                                    match g.get(&set) {
                                        Some(x) => HashMap::from([
                                            (set, (x.0, x.1)),
                                            (ArtifactSetName::Empty, (g[&ArtifactSetName::Empty].0, x.1))
                                        ]),
                                        None => HashMap::new(),
                                    }
                                }).collect();
                                let set_super_arts: Vec<_> = set_group_arts.iter().map(|g| {
                                    g.get(&set).map(|x| x.1)
                                }).collect();
                                for free_posi in 0..5 {
                                    let mut super_arts = set_super_arts.clone();
                                    super_arts[free_posi] = Option::Some(groups[free_posi][&ArtifactSetName::Empty].1);
                                    if !res_rec.check_hope_option(&super_arts) {
                                        continue;
                                    }
                                    let mut group_arts: Vec<_> = set_group_arts.iter().map(|hm| hm).collect();
                                    group_arts[free_posi] = &groups[free_posi];
                                    self.do_enumerate(group_arts, res_rec);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn do22(&self, art_sets: &Vec<ArtifactSetName>, fixed_set: Option<ArtifactSetName>, res_rec: &mut ResultRecorder) {
        // utils::log!("hello from do22");
        for flower_group in self.group_map[&ArtifactSlotName::Flower].values() {
            for feather_group in self.group_map[&ArtifactSlotName::Feather].values() {
                for sand_group in self.group_map[&ArtifactSlotName::Sand].values() {
                    for goblet_group in self.group_map[&ArtifactSlotName::Goblet].values() {
                        for head_group in self.group_map[&ArtifactSlotName::Head].values() {
                            let groups: Vec<_> = vec![
                                flower_group,
                                feather_group,
                                sand_group,
                                goblet_group,
                                head_group,
                            ].iter().map(|hm| {
                                let new_hm: HashMap<_, _> = hm.iter().map(|(&k, &gi)| {
                                    let (g, sa) = &self.group_arts[gi];
                                    (k, (g, sa))
                                }).collect();
                                new_hm
                            }).collect();
                            let slot_super_arts: Vec<_> = groups.iter().map(|hm| Some(hm[&ArtifactSetName::Empty].1)).collect();

                            for (i, &set1) in art_sets.iter().enumerate() {
                                if let Some(ref set) = fixed_set {
                                    if set != &set1 {
                                        continue;
                                    }
                                }
                                let set1_group_arts: Vec<_> = groups.iter().map(|g| {
                                    match g.get(&set1) {
                                        Some(x) => HashMap::from([
                                            (set1, (x.0, x.1)),
                                            (ArtifactSetName::Empty, (g[&ArtifactSetName::Empty].0, x.1))
                                        ]),
                                        None => HashMap::new(),
                                    }
                                }).collect();
                                let set1_super_arts: Vec<_> = set1_group_arts.iter().map(|g| {
                                    g.get(&set1).map(|x| x.1)
                                }).collect();

                                for &set2 in art_sets[(i + 1)..].iter() {
                                    let set2_group_arts: Vec<_> = groups.iter().map(|g| {
                                        match g.get(&set2) {
                                            Some(x) => HashMap::from([
                                                (set2, (x.0, x.1)),
                                                (ArtifactSetName::Empty, (g[&ArtifactSetName::Empty].0, x.1))
                                            ]),
                                            None => HashMap::new(),
                                        }
                                    }).collect();
                                    let set2_super_arts: Vec<_> = set2_group_arts.iter().map(|g| {
                                        g.get(&set2).map(|x| x.1)
                                    }).collect();

                                    for pos1 in 0..4 {
                                        for pos2 in (pos1 + 1)..5 {
                                            for pos3 in 0..4 {
                                                if pos3 == pos1 || pos3 == pos2 {
                                                    continue;
                                                }
                                                for pos4 in (pos3 + 1)..5 {
                                                    if pos4 == pos1 || pos4 == pos2 {
                                                        continue;
                                                    }
                                                    let mut super_arts = slot_super_arts.clone();
                                                    super_arts[pos1] = set1_super_arts[pos1];
                                                    super_arts[pos2] = set1_super_arts[pos2];
                                                    super_arts[pos3] = set2_super_arts[pos3];
                                                    super_arts[pos4] = set2_super_arts[pos4];
                                                    if !res_rec.check_hope_option(&super_arts) {
                                                        continue;
                                                    }

                                                    let mut group_arts: Vec<_> = groups.iter().map(|hm| hm).collect();
                                                    group_arts[pos1] = &set1_group_arts[pos1];
                                                    group_arts[pos2] = &set1_group_arts[pos2];
                                                    group_arts[pos3] = &set2_group_arts[pos3];
                                                    group_arts[pos4] = &set2_group_arts[pos4];
                                                    self.do_enumerate(group_arts, res_rec);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn do2(&self, art_sets: &Vec<ArtifactSetName>, res_rec: &mut ResultRecorder) {
        // utils::log!("hello from do2");
        for flower_group in self.group_map[&ArtifactSlotName::Flower].values() {
            for feather_group in self.group_map[&ArtifactSlotName::Feather].values() {
                for sand_group in self.group_map[&ArtifactSlotName::Sand].values() {
                    for goblet_group in self.group_map[&ArtifactSlotName::Goblet].values() {
                        for head_group in self.group_map[&ArtifactSlotName::Head].values() {
                            let groups: Vec<_> = vec![
                                flower_group,
                                feather_group,
                                sand_group,
                                goblet_group,
                                head_group,
                            ].iter().map(|hm| {
                                let new_hm: HashMap<_, _> = hm.iter().map(|(&k, &gi)| {
                                    let (g, sa) = &self.group_arts[gi];
                                    (k, (g, sa))
                                }).collect();
                                new_hm
                            }).collect();
                            let slot_super_arts: Vec<_> = groups.iter().map(|hm| Some(hm[&ArtifactSetName::Empty].1)).collect();

                            for &set in art_sets.iter() {
                                let set_group_arts: Vec<_> = groups.iter().map(|g| {
                                    match g.get(&set) {
                                        Some(x) => HashMap::from([
                                            (set, (x.0, x.1)),
                                            (ArtifactSetName::Empty, (g[&ArtifactSetName::Empty].0, x.1))
                                        ]),
                                        None => HashMap::new(),
                                    }
                                }).collect();
                                let set_super_arts: Vec<_> = set_group_arts.iter().map(|g| {
                                    g.get(&set).map(|x| x.1)
                                }).collect();
                                for free_posi_1 in 0..4 {
                                    for free_posi_2 in (free_posi_1 + 1)..5 {
                                        let mut super_arts = slot_super_arts.clone();
                                        super_arts[free_posi_1] = set_super_arts[free_posi_1];
                                        super_arts[free_posi_2] = set_super_arts[free_posi_2];
                                        if !res_rec.check_hope_option(&super_arts) {
                                            continue;
                                        }

                                        let mut group_arts: Vec<_> = groups.iter().map(|hm| hm).collect();
                                        group_arts[free_posi_1] = &set_group_arts[free_posi_1];
                                        group_arts[free_posi_2] = &set_group_arts[free_posi_2];
                                        self.do_enumerate(group_arts, res_rec);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn do_any(&self, res_rec: &mut ResultRecorder) {
        // utils::log!("hello from do_any");
        for flower_group in self.group_map[&ArtifactSlotName::Flower].values() {
            for feather_group in self.group_map[&ArtifactSlotName::Feather].values() {
                for sand_group in self.group_map[&ArtifactSlotName::Sand].values() {
                    for goblet_group in self.group_map[&ArtifactSlotName::Goblet].values() {
                        for head_group in self.group_map[&ArtifactSlotName::Head].values() {
                            let groups: Vec<_> = vec![
                                flower_group,
                                feather_group,
                                sand_group,
                                goblet_group,
                                head_group,
                            ].iter().map(|hm| {
                                let new_hm: HashMap<_, _> = hm.iter().map(|(&k, &gi)| {
                                    let (g, sa) = &self.group_arts[gi];
                                    (k, (g, sa))
                                }).collect();
                                new_hm
                            }).collect();
                            let group_arts: Vec<_> = groups.iter().map(|hm| hm).collect();
                            self.do_enumerate(group_arts, res_rec);
                        }
                    }
                }
            }
        }
    }

    fn optimize(&self, res_rec: &mut ResultRecorder) -> Vec<OptimizationResult> {
        let art_sets = HashSet::<_, RandomState>::from_iter(self.artifacts.iter().map(|&a| a.set_name));
        let art_sets = Vec::from_iter(art_sets.iter().map(|&a| a));
        match self.constraint.set_mode.as_ref().unwrap_or(&ConstraintSetMode::Any) {
            ConstraintSetMode::Any => {
                self.do4(&art_sets, res_rec);
                self.do22(&art_sets, None, res_rec);
                self.do2(&art_sets, res_rec);
                self.do_any(res_rec);
            },
            ConstraintSetMode::Set2(set_name) => {
                let limit_art_sets = vec![*set_name];
                self.do4(&limit_art_sets, res_rec);
                self.do22(&art_sets, Some(*set_name), res_rec);
                self.do2(&limit_art_sets, res_rec);
            },
            ConstraintSetMode::Set22(s1, s2) => {
                let limit_art_sets = vec![*s1, *s2];
                self.do22(&limit_art_sets, None, res_rec);
            },
            ConstraintSetMode::Set4(s1) => {
                self.do4(&vec![*s1], res_rec);
            }
        }

        let max_value = res_rec.result_heap.iter().map(|x| x.0.value).fold(-f64::INFINITY, f64::max);
        let mut results: Vec<OptimizationResult> = vec![];
        for i in res_rec.result_heap.iter() {
            let OptimizationIntermediateResult { value, arts } = &i.0;

            let optimization_result = OptimizationResult {
                flower: Some(arts[0]),
                feather: Some(arts[1]),
                sand: Some(arts[2]),
                goblet: Some(arts[3]),
                head: Some(arts[4]),
                value: *value,
                ratio: value / max_value,
            };

            results.push(optimization_result);
        }

        results.sort_by(|x, y| y.value.partial_cmp(&x.value).unwrap());
        results
    }
}

pub struct AStarCutoff;

impl SingleOptimizeAlgorithm for AStarCutoff {
    fn optimize(&self, artifacts: &[&Artifact], artifact_config: Option<ArtifactEffectConfig>, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, target_function: &Box<dyn TargetFunction>, enemy: &Enemy, buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>], constraint: &ConstraintConfig, count: usize) -> Vec<OptimizationResult> {
        let (flowers, feathers, sands, goblets, heads) = get_per_slot_artifacts(&artifacts);

        let any_zero = vec![flowers, feathers, sands, goblets, heads].iter().any(|x| x.len() == 0);
        if any_zero {
            let naive_algo = CutoffAlgorithmHeuristic { use_heuristic: false };
            return naive_algo.optimize(artifacts, artifact_config, character, weapon, target_function, enemy, buffs, constraint, count);
        }

        let single_optimizer = SingleOptimizer::new(artifacts, constraint);
        let mut res_rec = ResultRecorder::new(
            artifact_config,
            character,
            weapon,
            target_function,
            &enemy,
            constraint,
            buffs,
            count,
        );
        single_optimizer.optimize(&mut res_rec)
    }
}
