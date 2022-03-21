use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::collections::hash_map::RandomState;
// use std::iter::zip;
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult, OptimizeArtifactInterface};
use crate::applications::optimize_artifacts::single_optimize;
use crate::artifacts::{Artifact, ArtifactList, ArtifactSetName, ArtifactSlotName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeUtils, SimpleAttributeGraph2, AttributeCommon, AttributeName, Attribute};
use crate::buffs::Buff;
use crate::character::Character;
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::TargetFunction;
use crate::utils;
use crate::weapon::Weapon;

struct OptimizationIntermediateResult {
    value: f64,
    arts: Vec<u64>,
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
    result_set: HashSet<u64>,

    artifact_config: ArtifactEffectConfig,
    character: &'a Character<SimpleAttributeGraph2>,
    weapon: &'a Weapon<SimpleAttributeGraph2>,
    target_function: &'a Box<dyn TargetFunction>,
    constraint: Option<&'a ConstraintConfig>,
    buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
    enemy: Enemy,
}

impl<'a> ResultRecorder<'a> {
    pub fn new(
        artifact_config: Option<ArtifactEffectConfig>,
        character: &'a Character<SimpleAttributeGraph2>,
        weapon: &'a Weapon<SimpleAttributeGraph2>,
        target_function: &'a Box<dyn TargetFunction>,
        constraint: Option<&'a ConstraintConfig>,
        buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
        result_count: usize
    ) -> Self {
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

        Self {
            result_heap: BinaryHeap::with_capacity(result_count),
            result_count,
            result_set: HashSet::new(),

            artifact_config,
            character: character,
            weapon,
            target_function,
            constraint,
            buffs,
            enemy,
        }
    }

    fn calc_value(&self, arts: &Vec<&Artifact>) -> Option<f64> {
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

        if let Some(c) = self.constraint {
            if !check_attribute(&attribute, c) {
                return None;
            }
        }

        let value = self.target_function.target(&attribute, self.character, self.weapon, &arts, &self.enemy);
        Some(value)
    }

    fn arts_to_u64(arts: &Vec<&Artifact>) -> u64 {
        arts.iter().fold(0_u64, |acc, art| acc * 2000 + art.id)
    }

    fn push_result(&mut self, arts: &Vec<&Artifact>) {
        let hash = Self::arts_to_u64(arts);
        if self.result_set.contains(&hash) {
            return;
        }
        if let Some(value) = self.calc_value(&arts) {
            // utils::log!("push_result {}", value);
            let intermediate = OptimizationIntermediateResult {
                arts: arts.iter().map(|a| a.id).collect(),
                value,
            };
            if value > self.result_heap.peek().map_or(0., |r| r.0.value) {
                if self.result_heap.len() == self.result_count {
                    self.result_heap.pop();
                }
                self.result_heap.push(Reverse(intermediate));
                self.result_set.insert(hash);
            }
        }
    }

    fn check_hope(&self, arts: &Vec<&Artifact>) -> bool {
        if let Some(value) = self.calc_value(&arts) {
            // utils::log!("check_hope {}", value);
            value > self.result_heap.peek().map_or(0., |r| r.0.value)
        } else {
            false
        }
    }

    fn check_hope_option(&self, arts: &Vec<Option<&Artifact>>) -> bool {
        // utils::log!("check_hope_option {}", arts.len());
        if arts.iter().any(|op| op.is_none()) {
            return false;
        }
        let arts: Vec<_> = arts.iter().map(|op| op.unwrap()).collect();
        self.check_hope(&arts)
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
    // ideal_arts: HashMap<ArtifactSlotName, HashMap<StatName, HashMap<ArtifactSetName, Artifact>>>,
    // stat_groups: Vec<[(&'a HashMap<ArtifactSetName, Vec<usize>>, &'a HashMap<ArtifactSetName, Artifact>); 5]>,
    // stat_groups_temp: [(&'a HashMap<ArtifactSetName, Vec<usize>>, &'a HashMap<ArtifactSetName, Artifact>); 5],
    constraint: Option<&'a ConstraintConfig>,
}

impl<'a> SingleOptimizer<'a> {
    pub fn new(
        artifacts: &[&'a Artifact],
        constraint: Option<&'a ConstraintConfig>,
    ) -> Self {
        // group arts by slot, main stat, set; and make an fake super art for each set
        let mut group_arts = Vec::<(Vec<&Artifact>, Artifact)>::new();
        // let mut group_arts = HashMap::<ArtifactSlotName, HashMap<StatName, HashMap<ArtifactSetName, (Vec<&Artifact>, Artifact)>>>::new();
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
            // let (set_res, super_art) = mstat_res.entry(art.set_name).or_insert_with(|| (Vec::new(), Artifact::new(
            //     art.set_name,
            //     art.slot,
            //     0,
            //     5,
            //     Vec::new(),
            //     (art.main_stat.0, 0.),
            // )));
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
                // for (_, set_super_art) in stat_group.values() {
                //     merge_art_stat(&mut super_art, set_super_art);
                // }
                for &gi in stat_group.values() {
                    let (_, set_super_art) = &mut group_arts[gi];
                    merge_art_stat(&mut super_art, set_super_art);
                }
                group_arts.push((Vec::new(), super_art));
                let gi = group_arts.len() - 1;
                stat_group.insert(ArtifactSetName::Empty, gi);
            }
        }

        // get fake ideal arts
        // let mut ideal_arts = HashMap::<ArtifactSlotName, HashMap<StatName, HashMap<ArtifactSetName, Artifact>>>::new();
        // for (&slot, slot_arts) in group_arts.iter() {
        //     let slot_res = ideal_arts.entry(slot).or_default();
        //     for (&mstat, stat_arts) in slot_arts.iter() {
        //         let mstat_res = slot_res.entry(mstat).or_default();
        //         for (&set, set_arts) in stat_arts.iter() {
        //             let mut set_ideal_art = Artifact::new(
        //                 set,
        //                 slot,
        //                 0,
        //                 5,
        //                 Vec::new(),
        //                 (mstat, 0.),
        //             );
        //             for art in set_arts.iter() {
        //                 merge_art_stat(&mut set_ideal_art, art);
        //             }
        //             mstat_res.insert(set, set_ideal_art);
        //         }
        //         // over all set
        //         let mut empty_ideal_art = Artifact::new(
        //             ArtifactSetName::Empty,
        //             slot,
        //             0,
        //             5,
        //             Vec::new(),
        //             (mstat, 0.),
        //         );
        //         for art in mstat_res.values() {
        //             merge_art_stat(&mut empty_ideal_art, art);
        //         }
        //         mstat_res.insert(ArtifactSetName::Empty, empty_ideal_art);
        //     }
        // }

        let mut _self = Self {
            artifacts: artifacts.iter().map(|x| *x).collect(),
            group_arts,
            group_map,
            // ideal_arts,
            // stat_groups: Vec::new(),
            // stat_groups_temp: [],
            constraint,
        };
        // let mut temp: [(&HashMap<ArtifactSetName, Vec<usize>>, &HashMap<ArtifactSetName, Artifact>); 5];
        // _self.gen_stat_group(0, &mut temp);

        _self
    }

    // fn gen_stat_group(&mut self, slot_index: usize, temp: &mut [(&'a HashMap<ArtifactSetName, Vec<usize>>, &'a HashMap<ArtifactSetName, Artifact>); 5]) {
    //     if slot_index == ArtifactSlotName::LEN {
    //         let _temp: [(&HashMap<ArtifactSetName, Vec<usize>>, &HashMap<ArtifactSetName, Artifact>); 5];
    //         _temp.clone_from_slice(temp);
    //         // _temp.clone_from_slice(&self.stat_groups_temp);
    //         self.stat_groups.push(_temp);
    //         return;
    //     }
    //     let slot: ArtifactSlotName = num::FromPrimitive::from_usize(slot_index).unwrap();
    //     // for group in self.group_arts[&slot].values() {
    //     //     self.stat_groups_temp[slot_index].0 = group;
    //     //     self.gen_stat_group(slot_index + 1);
    //     // }
    //     for (group, ideal) in zip(self.group_arts[&slot].values(), self.ideal_arts[&slot].values()) {
    //         temp[slot_index] = (group, ideal);
    //         self.gen_stat_group(slot_index + 1, temp);
    //     }
    // }

    fn do_enumerate_recursive<'b>(
        &self,
        slot_index: usize,
        group_arts: &Vec<&HashMap<ArtifactSetName, (&Vec<&'b Artifact>, &Artifact)>>,
        arts: &mut Vec<&'b Artifact>,
        res_rec: &mut ResultRecorder,
    ) {
        if slot_index == 5 {
            res_rec.push_result(arts);
            return;
        }
        let mut upper_arts = arts.clone();
        for i in slot_index..5 {
            upper_arts.push(group_arts[i][&ArtifactSetName::Empty].1);
        }
        if !res_rec.check_hope(&upper_arts) {
            return;
        }
        let slot_group = group_arts[slot_index];
        for (set_arts, set_super_art) in slot_group.values() {
            if set_arts.is_empty() {
                continue;
            }
            upper_arts[slot_index] = set_super_art;
            if !res_rec.check_hope(&upper_arts) {
                continue;
            }
            for art in set_arts.iter() {
                arts.push(art);
                self.do_enumerate_recursive(slot_index + 1, group_arts, arts, res_rec);
                arts.pop();
            }
        }
    }

    fn do_enumerate(&self, group_arts: &Vec<&HashMap<ArtifactSetName, (&Vec<&Artifact>, &Artifact)>>, res_rec: &mut ResultRecorder) {
        let mut arts = Vec::with_capacity(5);
        self.do_enumerate_recursive(0, group_arts, &mut arts, res_rec);
    }

    fn do4(&self, art_sets: &Vec<ArtifactSetName>, res_rec: &mut ResultRecorder) {
        utils::log!("hello from do4");
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
                                    self.do_enumerate(&group_arts, res_rec);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn do22(&self, art_sets: &Vec<ArtifactSetName>, fixed_set: Option<ArtifactSetName>, res_rec: &mut ResultRecorder) {
        utils::log!("hello from do22");
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
                                                    self.do_enumerate(&group_arts, res_rec);
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
        utils::log!("hello from do2");
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
                                        self.do_enumerate(&group_arts, res_rec);
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
        utils::log!("hello from do_any");
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
                            self.do_enumerate(&group_arts, res_rec);
                        }
                    }
                }
            }
        }
    }

    fn optimize(&self, res_rec: &mut ResultRecorder) -> Vec<OptimizationResult> {
        let art_sets = HashSet::<_, RandomState>::from_iter(self.artifacts.iter().map(|&a| a.set_name));
        let art_sets = Vec::from_iter(art_sets.iter().map(|&a| a));
        match self.constraint.map_or(None, |c| c.set_mode.as_ref()).unwrap_or(&ConstraintSetMode::Any) {
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
    if use_optim {
        single_optimize::optimize_single(
            artifacts,
            artifact_config,
            character,
            weapon,
            target_function,
            constraint,
            buffs,
            count,
            use_optim,
        )
    } else {
        let single_optimizer = SingleOptimizer::new(artifacts, constraint);
        let mut res_rec = ResultRecorder::new(
            artifact_config,
            character,
            weapon,
            target_function,
            constraint,
            buffs,
            count,
        );
        single_optimizer.optimize(&mut res_rec)
    }
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
        5,
        input.use_optim
    );

    results
}
