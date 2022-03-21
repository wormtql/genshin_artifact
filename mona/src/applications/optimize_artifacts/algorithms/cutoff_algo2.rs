use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet, BinaryHeap};
use rand::Rng;
use smallvec::SmallVec;
use strum::IntoEnumIterator;

use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult};
use crate::artifacts::{Artifact, ArtifactList, ArtifactSetName, ArtifactSlotName};
use ArtifactSlotName::*;
use crate::artifacts::eff::ARTIFACT_EFF5;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeUtils, SimpleAttributeGraph2};
use crate::buffs::Buff;
use crate::character::Character;
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::TargetFunction;
use crate::utils::artifact::{get_per_slot_artifacts, get_artifacts_by_id};
use crate::weapon::Weapon;


type SuperArtifactType = HashMap<(ArtifactSlotName, StatName, ArtifactSetName), Artifact>;
type SuperArtifactType2 = HashMap<(ArtifactSlotName, StatName), Artifact>;


#[derive(Debug)]
struct SuperArtifactStruct {
    super_artifacts: SuperArtifactType,
    super_artifacts2: SuperArtifactType2
}

impl SuperArtifactStruct {
    fn from_artifacts(artifacts: &[&Artifact]) -> Self {
        let mut result1 = HashMap::new();

        let main_stat_names = vec![
            get_slot_main_stat_names(artifacts, ArtifactSlotName::Flower),
            get_slot_main_stat_names(artifacts, ArtifactSlotName::Feather),
            get_slot_main_stat_names(artifacts, ArtifactSlotName::Sand),
            get_slot_main_stat_names(artifacts, ArtifactSlotName::Goblet),
            get_slot_main_stat_names(artifacts, ArtifactSlotName::Head),
        ];
        // println!("{:?}", main_stat_names);

        let set_names = vec![
            get_slot_set_names(artifacts, ArtifactSlotName::Flower),
            get_slot_set_names(artifacts, ArtifactSlotName::Feather),
            get_slot_set_names(artifacts, ArtifactSlotName::Sand),
            get_slot_set_names(artifacts, ArtifactSlotName::Goblet),
            get_slot_set_names(artifacts, ArtifactSlotName::Head),
        ];

        for slot_index in 0..5 {
            let slot_enum: ArtifactSlotName = num::FromPrimitive::from_usize(slot_index).unwrap();
            for &main_stat_name in main_stat_names[slot_index].iter() {
                for &set_name in set_names[slot_index].iter() {
                    if let Some(super_artifact) = merge_super_artifact(artifacts, slot_enum, set_name, main_stat_name, false) {
                        result1.insert((slot_enum, main_stat_name, set_name), super_artifact);
                    }
                }
            }
        }

        let mut result2 = HashMap::new();
        for slot_index in 0..5 {
            let slot_enum: ArtifactSlotName = num::FromPrimitive::from_usize(slot_index).unwrap();
            for &main_stat_name in main_stat_names[slot_index].iter() {
                if let Some(super_artifact) = merge_super_artifact(artifacts, slot_enum, ArtifactSetName::Adventurer, main_stat_name, true) {
                    result2.insert((slot_enum, main_stat_name), super_artifact);
                }
            }
        }

        SuperArtifactStruct {
            super_artifacts: result1,
            super_artifacts2: result2,
        }
    }

    fn get_with_set_name(&self, slot: ArtifactSlotName, main_stat: StatName, set_name: ArtifactSetName) -> Option<&Artifact> {
        self.super_artifacts.get(&(slot, main_stat, set_name))
    }

    fn get_without_set_name(&self, slot: ArtifactSlotName, main_stat: StatName) -> &Artifact {
        self.super_artifacts2.get(&(slot, main_stat)).unwrap()
    }
}

struct ArtifactStatistics<'a> {
    slot_main_stat_names: HashMap<ArtifactSlotName, HashSet<StatName>>,
    // slot_set_names: HashMap<ArtifactSlotName, HashSet<ArtifactSetName>>,
    slot_stat_set_names: HashMap<(ArtifactSlotName, StatName), HashSet<ArtifactSetName>>,
    all_set_names: HashSet<ArtifactSetName>,
    all_set_names_flat: Vec<ArtifactSetName>,
    artifact_group: HashMap<(ArtifactSlotName, StatName, ArtifactSetName), Vec<&'a Artifact>>,
    artifact_group2: HashMap<(ArtifactSlotName, StatName), Vec<&'a Artifact>>,
}

fn group_artifact<'a>(artifacts: &[&'a Artifact]) -> HashMap<(ArtifactSlotName, StatName, ArtifactSetName), Vec<&'a Artifact>> {
    let mut result = HashMap::new();

    for artifact in artifacts.iter() {
        let key = (artifact.slot, artifact.main_stat.0, artifact.set_name);
        result.entry(key).or_insert(Vec::new()).push(*artifact);
    }

    result
}

fn group_artifact2<'a>(artifacts: &[&'a Artifact]) -> HashMap<(ArtifactSlotName, StatName), Vec<&'a Artifact>> {
    let mut result = HashMap::new();

    for artifact in artifacts.iter() {
        let key = (artifact.slot, artifact.main_stat.0);
        result.entry(key).or_insert(Vec::new()).push(*artifact);
    }

    result
}

impl<'a> ArtifactStatistics<'a> {
    fn from_artifacts(artifacts: &[&'a Artifact]) -> Self {
        let all_set_names = get_all_set_names(artifacts);

        let mut slot_main_stat_names = HashMap::new();
        let mut slot_stat_set_names = HashMap::new();

        for slot in ArtifactSlotName::iter() {
            slot_main_stat_names.insert(slot, get_slot_main_stat_names(artifacts, slot));
            // slot_set_names.insert(slot, get_slot_set_names(artifacts, slot));
        }

        for artifact in artifacts.iter() {
            let key = (artifact.slot, artifact.main_stat.0);
            slot_stat_set_names.entry(key).or_insert(HashSet::new()).insert(artifact.set_name);
        }

        ArtifactStatistics {
            slot_main_stat_names,
            // slot_set_names,
            slot_stat_set_names,
            all_set_names_flat: all_set_names.clone().drain().collect(),
            all_set_names,
            artifact_group: group_artifact(artifacts),
            artifact_group2: group_artifact2(artifacts),
        }
    }

    fn get_artifacts_with_set_name(&self, slot: ArtifactSlotName, stat_name: StatName, set_name: ArtifactSetName) -> &[&Artifact] {
        self.artifact_group.get(&(slot, stat_name, set_name)).unwrap()
    }

    fn get_artifacts_without_set_name(&self, slot: ArtifactSlotName, stat_name: StatName) -> &[&Artifact] {
        self.artifact_group2.get(&(slot, stat_name)).unwrap()
    }
}

struct BinaryHeapItem {
    artifacts: SmallVec<[u64; 5]>,
    score: f64,
}

impl PartialEq for BinaryHeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for BinaryHeapItem {}

impl PartialOrd for BinaryHeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for BinaryHeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct ResultRecorder<'a> {
    records: BinaryHeap<Reverse<BinaryHeapItem>>,

    capacity: usize,

    character: &'a Character<SimpleAttributeGraph2>,
    weapon: &'a Weapon<SimpleAttributeGraph2>,
    enemy: &'a Enemy,
    target_function: &'a Box<dyn TargetFunction>,
    buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
    artifact_config: &'a ArtifactEffectConfig,
}

impl<'a> ResultRecorder<'a> {
    fn value(&self, artifacts: &[&Artifact]) -> f64 {
        let artifact_list = ArtifactList {
            artifacts,
        };
        let attribute = AttributeUtils::create_attribute_from_big_config(
            &artifact_list,
            self.artifact_config,
            self.character,
            self.weapon,
            self.buffs
        );

        let v = self.target_function.target(&attribute, self.character, self.weapon, artifacts, self.enemy);
        v
    }

    fn max_value(&self) -> f64 {
        self.records.iter().map(|x| x.0.score).fold(0.0, f64::max)
    }

    fn is_better(&self, artifacts: &[&Artifact]) -> bool {
        if self.records.len() == 0 {
            true
        } else {
            let v = self.value(artifacts);

            v > self.records.peek().unwrap().0.score
        }

    }

    fn is_better_map(&self, artifacts: &HashMap<ArtifactSlotName, &Artifact>) -> bool {
        let mut vec: SmallVec<[&Artifact; 5]> = SmallVec::new();
        for a in artifacts.iter() {
            vec.push(*a.1);
        }

        self.is_better(&vec)
    }

    fn update(&mut self, artifacts: &[&Artifact]) {
        let value = self.value(artifacts);

        let mut record = BinaryHeapItem {
            artifacts: SmallVec::new(),
            score: value,
        };
        for artifact in artifacts.iter() {
            record.artifacts.push(artifact.id);
        }

        if self.records.len() < self.capacity {
            self.records.push(Reverse(record));
        } else {
            let current_min = self.records.peek().unwrap().0.score;
            if value > current_min {
                self.records.pop();
                self.records.push(Reverse(record));
            }
        }
    }

    fn update_map(&mut self, artifacts: &HashMap<ArtifactSlotName, &Artifact>) {
        let mut vec: SmallVec<[&Artifact; 5]> = SmallVec::new();
        for a in artifacts.iter() {
            vec.push(*a.1);
        }

        self.update(&vec);
    }
}

fn merge_super_artifact(artifacts: &[&Artifact], slot: ArtifactSlotName, set_name: ArtifactSetName, main_stat_name: StatName, is_all: bool) -> Option<Artifact> {
    let mut value_map = HashMap::new();

    let artifacts: Vec<&Artifact> = artifacts.iter()
        .map(|x| *x)
        .filter(move |x| x.main_stat.0 == main_stat_name && x.slot == slot && ((!is_all && x.set_name == set_name) || is_all))
        .collect();

    if artifacts.len() == 0 {
        return None;
    }

    let mut max_main_stat_value: f64 = 0.0;
    for artifact in artifacts.iter() {
        for &(name, value) in artifact.sub_stats.iter() {
            if !value_map.contains_key(&name) {
                value_map.insert(name, 0.0);
            }
            let old_value: f64 = *value_map.get(&name).unwrap();
            value_map.insert(name, old_value.max(value));
        }

        max_main_stat_value = max_main_stat_value.max(artifact.main_stat.1);
    }

    let mut super_sub_stats = Vec::new();
    for (name, value) in value_map {
        super_sub_stats.push((name, value));
    }

    let artifact = Artifact {
        set_name,
        slot,
        level: 20,
        star: 5,
        sub_stats: super_sub_stats,
        main_stat: (main_stat_name, max_main_stat_value),
        id: rand::thread_rng().gen(),
    };

    Some(artifact)
}

fn get_set_names(artifacts: &[&Artifact]) -> HashMap<(ArtifactSlotName, StatName, ArtifactSetName), usize> {
    let mut result = HashMap::new();
    for artifact in artifacts.iter() {
        let key = (artifact.slot, artifact.main_stat.0, artifact.set_name);
        *result.entry(key).or_insert(0) += 1;
    }

    result
}

fn get_slot_main_stat_names(artifacts: &[&Artifact], slot: ArtifactSlotName) -> HashSet<StatName> {
    artifacts.iter()
        .filter(|a| a.slot == slot)
        .map(|a| a.main_stat.0)
        .collect()
}

fn get_slot_set_names(artifacts: &[&Artifact], slot: ArtifactSlotName) -> HashSet<ArtifactSetName> {
    artifacts.iter()
        .filter(|a| a.slot == slot)
        .map(|a| a.set_name)
        .collect()
}

fn get_all_set_names(artifacts: &[&Artifact]) -> HashSet<ArtifactSetName> {
    artifacts.iter().map(|a| a.set_name).collect()
}

fn select_main_stat_name(slot: ArtifactSlotName, n1: StatName, n2: StatName, n3: StatName) -> StatName {
    match slot {
        ArtifactSlotName::Flower => StatName::HPFixed,
        ArtifactSlotName::Feather => StatName::ATKFixed,
        ArtifactSlotName::Sand => n1,
        ArtifactSlotName::Goblet => n2,
        ArtifactSlotName::Head => n3
    }
}

fn do_iter(artifacts: &HashMap<ArtifactSlotName, &[&Artifact]>, super_artifacts: &SuperArtifactStruct, s: &ArtifactStatistics, recorder: &mut ResultRecorder, n1: StatName, n2: StatName, n3: StatName) {
    let mut slot_count: HashMap<ArtifactSlotName, usize> = HashMap::new();

    for slot in ArtifactSlotName::iter() {
        slot_count.insert(slot, artifacts.get(&slot).unwrap().len());
    }

    let mut temp: Vec<(ArtifactSlotName, usize)> = slot_count.drain().collect();
    temp.sort_by(|a, b| a.1.cmp(&b.1));

    let slot_order = temp.iter().map(|x| x.0).collect::<Vec<_>>();

    let s0 = slot_order[0];
    let s1 = slot_order[1];
    let s2 = slot_order[2];
    let s3 = slot_order[3];
    let s4 = slot_order[4];

    let sm0 = select_main_stat_name(s0, n1, n2, n3);
    let sm1 = select_main_stat_name(s1, n1, n2, n3);
    let sm2 = select_main_stat_name(s2, n1, n2, n3);
    let sm3 = select_main_stat_name(s3, n1, n2, n3);
    let sm4 = select_main_stat_name(s4, n1, n2, n3);

    let mut artifacts_buffer10 = HashMap::new();
    let mut artifacts_buffer11 = HashMap::new();
    let mut artifacts_buffer20 = HashMap::new();
    let mut artifacts_buffer21 = HashMap::new();
    let mut artifacts_buffer30 = HashMap::new();
    let mut artifacts_buffer31 = HashMap::new();
    let mut artifacts_buffer40 = HashMap::new();
    let mut artifacts_buffer41 = HashMap::new();
    let mut artifacts_buffer50 = HashMap::new();
    for &sn1 in s.slot_stat_set_names.get(&(s0, sm0)).unwrap() {
        artifacts_buffer10.clear();
        // fill super artifacts
        for slot in ArtifactSlotName::iter() {
            let stat_name = select_main_stat_name(slot, n1, n2, n3);
            if slot == s0 {
                // let result = super_artifacts.get_with_set_name(s0, stat_name, sn1);
                artifacts_buffer10.insert(slot, super_artifacts.get_with_set_name(slot, stat_name, sn1).unwrap());
            } else {
                artifacts_buffer10.insert(slot, super_artifacts.get_without_set_name(slot, stat_name));
            }
        }
        if !recorder.is_better_map(&artifacts_buffer10) {
            continue;
        }

        for &a1 in artifacts.get(&s0).unwrap().iter() {
            artifacts_buffer11.clone_from(&artifacts_buffer10);
            artifacts_buffer11.insert(s0, a1);
            if !recorder.is_better_map(&artifacts_buffer11) {
                continue;
            }

            for &sn2 in s.slot_stat_set_names.get(&(s1, sm1)).unwrap().iter() {
                artifacts_buffer20.clone_from(&artifacts_buffer11);
                artifacts_buffer20.insert(s1, super_artifacts.get_with_set_name(s1, sm1, sn2).unwrap());
                if !recorder.is_better_map(&artifacts_buffer20) {
                    continue;
                }

                for &a2 in artifacts.get(&s1).unwrap().iter() {
                    artifacts_buffer21.clone_from(&artifacts_buffer20);
                    artifacts_buffer21.insert(s1, a2);
                    if !recorder.is_better_map(&artifacts_buffer21) {
                        continue;
                    }

                    for &sn3 in s.slot_stat_set_names.get(&(s2, sm2)).unwrap().iter() {
                        artifacts_buffer30.clone_from(&artifacts_buffer21);
                        artifacts_buffer30.insert(s2, super_artifacts.get_with_set_name(s2, sm2, sn3).unwrap());
                        if !recorder.is_better_map(&artifacts_buffer30) {
                            continue;
                        }

                        for &a3 in artifacts.get(&s2).unwrap().iter() {
                            artifacts_buffer31.clone_from(&artifacts_buffer30);
                            artifacts_buffer31.insert(s2, a3);
                            if !recorder.is_better_map(&artifacts_buffer31) {
                                continue;
                            }

                            for &sn4 in s.slot_stat_set_names.get(&(s3, sm3)).unwrap().iter() {
                                artifacts_buffer40.clone_from(&artifacts_buffer31);
                                artifacts_buffer40.insert(s3, super_artifacts.get_with_set_name(s3, sm3, sn4).unwrap());
                                if !recorder.is_better_map(&artifacts_buffer40) {
                                    continue;
                                }

                                for &a4 in artifacts.get(&s3).unwrap().iter() {
                                    artifacts_buffer41.clone_from(&artifacts_buffer40);
                                    artifacts_buffer41.insert(s3, a4);
                                    if !recorder.is_better_map(&artifacts_buffer41) {
                                        continue;
                                    }

                                    for &sn5 in s.slot_stat_set_names.get(&(s4, sm4)).unwrap().iter() {
                                        artifacts_buffer50.clone_from(&artifacts_buffer41);
                                        artifacts_buffer50.insert(s4, super_artifacts.get_with_set_name(s4, sm4, sn5).unwrap());
                                        if !recorder.is_better_map(&artifacts_buffer50) {
                                            continue;
                                        }

                                        for &a5 in artifacts.get(&s4).unwrap().iter() {
                                            artifacts_buffer50.insert(s4, a5);
                                            recorder.update_map(&artifacts_buffer50);
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

fn iter_set4(super_artifacts: &SuperArtifactStruct, recorder: &mut ResultRecorder, s: &ArtifactStatistics, fixed_set: Option<ArtifactSetName>) {
    let mut artifacts_buffer: SmallVec<[&Artifact; 5]> = SmallVec::new();
    let mut filter_artifacts_buffer: HashMap<ArtifactSlotName, &[&Artifact]> = HashMap::with_capacity(5);
    for &sand_main_stat_name in s.slot_main_stat_names.get(&Sand).unwrap() {
        for &goblet_main_stat_name in s.slot_main_stat_names.get(&Goblet).unwrap() {
            for &head_main_stat_name in s.slot_main_stat_names.get(&Head).unwrap() {
                for &set_name in s.all_set_names.iter() {
                    if fixed_set.is_some() && set_name != fixed_set.unwrap() {
                        continue;
                    }

                    'any_slot: for slot in ArtifactSlotName::iter() {
                        // slot is any
                        artifacts_buffer.clear();

                        for slot2 in ArtifactSlotName::iter() {
                            let stat_name = select_main_stat_name(slot2, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            if slot == slot2 {
                                artifacts_buffer.push(super_artifacts.get_without_set_name(slot, stat_name));
                            } else {
                                let result = super_artifacts.get_with_set_name(slot2, stat_name, set_name);
                                if let Some(a) = result {
                                    artifacts_buffer.push(a);
                                } else {
                                    continue 'any_slot;
                                }
                            }
                        }

                        if !recorder.is_better(&artifacts_buffer) {
                            // do cutoff
                            continue;
                        }

                        for slot2 in ArtifactSlotName::iter() {
                            let stat_name = select_main_stat_name(slot2, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            if slot == slot2 {
                                filter_artifacts_buffer.insert(slot2, s.get_artifacts_without_set_name(slot2, stat_name));
                            } else {
                                filter_artifacts_buffer.insert(slot2, s.get_artifacts_with_set_name(slot2, stat_name, set_name));
                            }
                        }

                        do_iter(&filter_artifacts_buffer, super_artifacts, s, recorder, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                    }
                }
            }
        }
    }
}

fn iter_set22(super_artifacts: &SuperArtifactStruct, recorder: &mut ResultRecorder, s: &ArtifactStatistics, fixed1: Option<ArtifactSetName>, fixed2: Option<ArtifactSetName>) {
    let mut index_buffer: [usize; 4] = [0; 4];
    let mut index_buffer2: [usize; 4] = [0; 4];
    let mut artifacts_buffer: HashMap<ArtifactSlotName, &Artifact> = HashMap::with_capacity(5);
    let mut filter_artifacts_buffer: HashMap<ArtifactSlotName, &[&Artifact]> = HashMap::with_capacity(5);

    for &sand_main_stat_name in s.slot_main_stat_names.get(&Sand).unwrap() {
        for &goblet_main_stat_name in s.slot_main_stat_names.get(&Goblet).unwrap() {
            for &head_main_stat_name in s.slot_main_stat_names.get(&Head).unwrap() {
                for any_slot_index in 0..5 {
                    let any_slot: ArtifactSlotName = num::FromPrimitive::from_usize(any_slot_index).unwrap();
                    let any_slot_stat_name = select_main_stat_name(any_slot, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                    artifacts_buffer.clear();
                    artifacts_buffer.insert(any_slot, super_artifacts.get_without_set_name(any_slot, any_slot_stat_name));

                    let mut ii = 0;
                    for i in 0..5 {
                        if i != any_slot_index {
                            index_buffer[ii] = i;
                            ii += 1;
                        }
                    }

                    for a in 0..3 {
                        for b in a + 1..4 {
                            // a, b is set name 1
                            let aa = index_buffer[a];
                            let bb = index_buffer[b];

                            index_buffer2[0] = aa;
                            index_buffer2[1] = bb;

                            let mut it = 2;
                            for c in 0..4 {
                                let cc = index_buffer[c];
                                if cc != aa && cc != bb {
                                    index_buffer2[it] = cc;
                                    it += 1;
                                }
                            }

                            let s1 = num::FromPrimitive::from_usize(index_buffer2[0]).unwrap();
                            let s2 = num::FromPrimitive::from_usize(index_buffer2[1]).unwrap();
                            let s3 = num::FromPrimitive::from_usize(index_buffer2[2]).unwrap();
                            let s4 = num::FromPrimitive::from_usize(index_buffer2[3]).unwrap();

                            let ms1 = select_main_stat_name(s1, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let ms2 = select_main_stat_name(s2, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let ms3 = select_main_stat_name(s3, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let ms4 = select_main_stat_name(s4, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);

                            for sn1_index in 0_usize..s.all_set_names_flat.len() {
                                let sn1 = s.all_set_names_flat[sn1_index];
                                if fixed1.is_some() && fixed1.unwrap() != sn1 {
                                    continue;
                                }

                                if let Some(a) = super_artifacts.get_with_set_name(s1, ms1, sn1) {
                                    artifacts_buffer.insert(s1, a);
                                } else {
                                    continue;
                                }
                                if let Some(a) = super_artifacts.get_with_set_name(s2, ms2, sn1) {
                                    artifacts_buffer.insert(s2, a);
                                } else {
                                    continue;
                                }
                                artifacts_buffer.insert(s3, super_artifacts.get_without_set_name(s3, ms3));
                                artifacts_buffer.insert(s4, super_artifacts.get_without_set_name(s4, ms4));

                                if !recorder.is_better_map(&artifacts_buffer) {
                                    continue;
                                }

                                for sn2_index in sn1_index..s.all_set_names_flat.len() {
                                    let sn2 = s.all_set_names_flat[sn2_index];
                                    if fixed2.is_some() && fixed2.unwrap() != sn2 {
                                        continue;
                                    }

                                    if let Some(a) = super_artifacts.get_with_set_name(s3, ms3, sn2) {
                                        artifacts_buffer.insert(s3, a);
                                    } else {
                                        continue;
                                    }
                                    if let Some(a) = super_artifacts.get_with_set_name(s4, ms4, sn2) {
                                        artifacts_buffer.insert(s4, a);
                                    } else {
                                        continue;
                                    }

                                    if !recorder.is_better_map(&artifacts_buffer) {
                                        continue;
                                    }

                                    filter_artifacts_buffer.insert(any_slot, s.get_artifacts_without_set_name(any_slot, any_slot_stat_name));
                                    filter_artifacts_buffer.insert(s1, s.get_artifacts_with_set_name(s1, ms1, sn1));
                                    filter_artifacts_buffer.insert(s2, s.get_artifacts_with_set_name(s2, ms2, sn1));
                                    filter_artifacts_buffer.insert(s3, s.get_artifacts_with_set_name(s3, ms3, sn2));
                                    filter_artifacts_buffer.insert(s4, s.get_artifacts_with_set_name(s4, ms4, sn2));

                                    do_iter(&filter_artifacts_buffer, super_artifacts, s, recorder, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn iter_set2(super_artifacts: &SuperArtifactStruct, recorder: &mut ResultRecorder, s: &ArtifactStatistics, fixed: Option<ArtifactSetName>) {
    let mut artifacts_buffer = HashMap::with_capacity(5);
    let mut filter_artifacts_buffer: HashMap<ArtifactSlotName, &[&Artifact]> = HashMap::with_capacity(5);
    let mut index_buffer = [0_usize; 3];
    for &sand_main_stat_name in s.slot_main_stat_names.get(&Sand).unwrap() {
        for &goblet_main_stat_name in s.slot_main_stat_names.get(&Goblet).unwrap() {
            for &head_main_stat_name in s.slot_main_stat_names.get(&Head).unwrap() {
                for &set_name in s.all_set_names.iter() {
                    if fixed.is_some() && set_name != fixed.unwrap() {
                        continue;
                    }

                    'iter_sn1: for a in 0..4 {
                        'iter_sn2: for b in a + 1..5 {
                            let mut it = 0;
                            for i in 0..5 {
                                if i != a && i != b {
                                    index_buffer[it] = i;
                                    it += 1;
                                }
                            }

                            // 1, 2 are of set name
                            let s1: ArtifactSlotName = num::FromPrimitive::from_usize(a).unwrap();
                            let s2: ArtifactSlotName = num::FromPrimitive::from_usize(b).unwrap();
                            let s3: ArtifactSlotName = num::FromPrimitive::from_usize(index_buffer[0]).unwrap();
                            let s4: ArtifactSlotName = num::FromPrimitive::from_usize(index_buffer[1]).unwrap();
                            let s5: ArtifactSlotName = num::FromPrimitive::from_usize(index_buffer[2]).unwrap();

                            let sm1 = select_main_stat_name(s1, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let sm2 = select_main_stat_name(s2, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let sm3 = select_main_stat_name(s3, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let sm4 = select_main_stat_name(s4, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                            let sm5 = select_main_stat_name(s5, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);

                            if let Some(a) = super_artifacts.get_with_set_name(s1, sm1, set_name) {
                                artifacts_buffer.insert(s1, a);
                            } else {
                                continue 'iter_sn1;
                            }
                            if let Some(a) = super_artifacts.get_with_set_name(s2, sm2, set_name) {
                                artifacts_buffer.insert(s2, a);
                            } else {
                                continue;
                            }
                            artifacts_buffer.insert(s3, super_artifacts.get_without_set_name(s3, sm3));
                            artifacts_buffer.insert(s4, super_artifacts.get_without_set_name(s4, sm4));
                            artifacts_buffer.insert(s5, super_artifacts.get_without_set_name(s5, sm5));

                            if !recorder.is_better_map(&artifacts_buffer) {
                                continue;
                            }

                            filter_artifacts_buffer.insert(s1, s.get_artifacts_with_set_name(s1, sm1, set_name));
                            filter_artifacts_buffer.insert(s2, s.get_artifacts_with_set_name(s2, sm2, set_name));
                            filter_artifacts_buffer.insert(s3, s.get_artifacts_without_set_name(s3, sm3));
                            filter_artifacts_buffer.insert(s4, s.get_artifacts_without_set_name(s4, sm4));
                            filter_artifacts_buffer.insert(s5, s.get_artifacts_without_set_name(s5, sm5));

                            do_iter(&filter_artifacts_buffer, super_artifacts, s, recorder, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                        }
                    }
                }
            }
        }
    }
}

fn iter_any(super_artifacts: &SuperArtifactStruct, recorder: &mut ResultRecorder, s: &ArtifactStatistics) {
    let mut filter_artifacts_buffer: HashMap<ArtifactSlotName, &[&Artifact]> = HashMap::with_capacity(5);
    for &sand_main_stat_name in s.slot_main_stat_names.get(&Sand).unwrap() {
        for &goblet_main_stat_name in s.slot_main_stat_names.get(&Goblet).unwrap() {
            for &head_main_stat_name in s.slot_main_stat_names.get(&Head).unwrap() {
                let sm1 = select_main_stat_name(Sand, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                let sm2 = select_main_stat_name(Goblet, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
                let sm3 = select_main_stat_name(Head, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);

                filter_artifacts_buffer.insert(Flower, s.get_artifacts_without_set_name(Flower, StatName::HPFixed));
                filter_artifacts_buffer.insert(Feather, s.get_artifacts_without_set_name(Feather, StatName::ATKFixed));
                filter_artifacts_buffer.insert(Sand, s.get_artifacts_without_set_name(Sand, sm1));
                filter_artifacts_buffer.insert(Goblet, s.get_artifacts_without_set_name(Goblet, sm2));
                filter_artifacts_buffer.insert(Head, s.get_artifacts_without_set_name(Head, sm3));

                do_iter(&filter_artifacts_buffer, super_artifacts, s, recorder, sand_main_stat_name, goblet_main_stat_name, head_main_stat_name);
            }
        }
    }
}

fn entry_to_result(entry: &BinaryHeapItem, m: &HashMap<u64, &Artifact>, max_value: f64) -> OptimizationResult {
    let mut temp: HashMap<ArtifactSlotName, &Artifact> = HashMap::new();

    for &id in entry.artifacts.iter() {
        if let Some(a) = m.get(&id) {
            let slot = a.slot;
            temp.insert(slot, *a);
        }
    }

    OptimizationResult {
        flower: temp.get(&Flower).map(|x| x.id),
        feather: temp.get(&Feather).map(|x| x.id),
        sand: temp.get(&Sand).map(|x| x.id),
        goblet: temp.get(&Goblet).map(|x| x.id),
        head: temp.get(&Head).map(|x| x.id),
        value: entry.score,
        ratio: entry.score / max_value
    }
}

pub struct CutoffAlgorithm2;

impl SingleOptimizeAlgorithm for CutoffAlgorithm2 {
    fn optimize(
        &self,
        artifacts: &[&Artifact],
        artifact_config: Option<ArtifactEffectConfig>,
        character: &Character<SimpleAttributeGraph2>,
        weapon: &Weapon<SimpleAttributeGraph2>,
        target_function: &Box<dyn TargetFunction>,
        enemy: &Enemy,
        buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>],
        constraint: &ConstraintConfig,
        count: usize
    ) -> Vec<OptimizationResult> {
        // setup enemy
        let mut enemy = enemy.clone();
        for buff in buffs.iter() {
            buff.change_enemy(&mut enemy);
        }

        // get artifact config
        let artifact_config = if let Some(x) = artifact_config {
            x
        } else {
            target_function.get_default_artifact_config(&Default::default())
        };

        let super_artifacts = SuperArtifactStruct::from_artifacts(artifacts);
        let artifacts_statistics = ArtifactStatistics::from_artifacts(artifacts);

        let mut recorder = ResultRecorder {
            records: BinaryHeap::with_capacity(count + 1),
            capacity: count,
            character,
            weapon,
            enemy: &enemy,
            target_function,
            buffs,
            artifact_config: &artifact_config
        };

        let set_mode = constraint.set_mode.as_ref().unwrap_or(&ConstraintSetMode::Any);
        match *set_mode {
            ConstraintSetMode::Any => {
                iter_set4(&super_artifacts, &mut recorder, &artifacts_statistics, None);
                iter_set22(&super_artifacts, &mut recorder, &artifacts_statistics, None, None);
                iter_set2(&super_artifacts, &mut recorder, &artifacts_statistics, None);
                iter_any(&super_artifacts, &mut recorder, &artifacts_statistics);
            },
            ConstraintSetMode::Set4(set_name) => {
                iter_set4(&super_artifacts, &mut recorder, &artifacts_statistics, Some(set_name));
            },
            ConstraintSetMode::Set22(sn1, sn2) => {
                iter_set22(&super_artifacts, &mut recorder, &artifacts_statistics, Some(sn1), Some(sn2));
            },
            ConstraintSetMode::Set2(set_name) => {
                iter_set4(&super_artifacts, &mut recorder, &artifacts_statistics, Some(set_name));
                iter_set22(&super_artifacts, &mut recorder, &artifacts_statistics, Some(set_name), None);
                iter_set2(&super_artifacts, &mut recorder, &artifacts_statistics, Some(set_name));
            }
        }

        let artifacts_by_id = get_artifacts_by_id(artifacts);
        let max_score = recorder.max_value();

        let mut results = Vec::new();
        for item in recorder.records.iter() {
            results.push(entry_to_result(&item.0, &artifacts_by_id, max_score));
        }

        println!("{:?}", results);

        results
    }
}