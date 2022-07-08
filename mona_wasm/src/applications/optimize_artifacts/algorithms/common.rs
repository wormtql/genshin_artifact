use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use mona::artifacts::{Artifact, ArtifactList, ArtifactSetName};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{AttributeUtils, SimpleAttributeGraph2, AttributeCommon, Attribute, AttributeName};
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use rustc_hash::FxHashSet;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};

#[derive(Clone)]
pub struct OptimizationIntermediateResult {
    // arts must have same slot order
    pub arts: [u64; 5],
    pub score: f64,
}

impl OptimizationIntermediateResult {
    pub fn to_result(&self, max_score: f64) -> OptimizationResult {
        OptimizationResult {
            flower: Some(self.arts[0]),
            feather: Some(self.arts[1]),
            sand: Some(self.arts[2]),
            goblet: Some(self.arts[3]),
            head: Some(self.arts[4]),
            value: self.score,
            ratio: self.score / max_score
        }
    }
}

impl Eq for OptimizationIntermediateResult {}

impl PartialEq<Self> for OptimizationIntermediateResult {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl PartialOrd<Self> for OptimizationIntermediateResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for OptimizationIntermediateResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

pub struct ValueFunction<'a> {
    pub artifact_effect_config: &'a ArtifactEffectConfig,
    pub character: &'a Character<SimpleAttributeGraph2>,
    pub weapon: &'a Weapon<SimpleAttributeGraph2>,
    pub target_function: &'a Box<dyn TargetFunction>,
    pub buffs: &'a [Box<dyn Buff<SimpleAttributeGraph2>>],
    pub enemy: &'a Enemy,
    pub constraint: &'a ConstraintConfig,
}

impl<'a> ValueFunction<'a> {
    pub fn get_attribute(&self, arts: &[&Artifact]) -> SimpleAttributeGraph2 {
        let art_list = ArtifactList {
            artifacts: arts,
        };
        let attribute = AttributeUtils::create_attribute_from_big_config(
            &art_list,
            &self.artifact_effect_config,
            &self.character,
            &self.weapon,
            &self.buffs
        );
        attribute
    }

    pub fn score_attribute(&self, attribute: &SimpleAttributeGraph2, arts: &[&Artifact]) -> f64 {
        let score = self.target_function.target(
            &attribute,
            &self.character,
            &self.weapon,
            &arts,
            &self.enemy
        );
        score
    }

    pub fn score(&self, arts: &[&Artifact]) -> f64 {
        let attribute = self.get_attribute(arts);

        self.score_attribute(&attribute, arts)
    }

    pub fn check_attribute_attribute(&self, attribute: &SimpleAttributeGraph2) -> bool {
        if attribute.get_atk() < self.constraint.atk_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_def() < self.constraint.def_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_hp() < self.constraint.hp_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_value(AttributeName::ElementalMastery) < self.constraint.em_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_value(AttributeName::Recharge) < self.constraint.recharge_min.unwrap_or(0.0) {
            return false;
        }
        let critical = attribute.get_value(AttributeName::CriticalBase).clamp(0.0, 1.0);
        if critical < self.constraint.crit_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_value(AttributeName::CriticalDamageBase) < self.constraint.crit_dmg_min.unwrap_or(0.0) {
            return false;
        }

        true
    }

    pub fn check_attribute(&self, arts: &[&Artifact]) -> bool {
        let attribute = self.get_attribute(arts);

        self.check_attribute_attribute(&attribute)
    }
}

pub struct ResultRecorder {
    pub size: usize,
    pub results: BinaryHeap<Reverse<OptimizationIntermediateResult>>,
    pub result_hash: HashSet<[u64; 5]>,
    // pub result_hash: FxHashSet<[u64; 5]>,
}

impl ResultRecorder {
    pub fn new(size: usize) -> ResultRecorder {
        ResultRecorder {
            size,
            results: BinaryHeap::with_capacity(size + 1),
            result_hash: HashSet::new(),
            // result_hash: FxHashSet::default(),
        }
    }

    pub fn get_results_descend(&self) -> Vec<OptimizationIntermediateResult> {
        let mut result = Vec::new();

        for item in self.results.iter() {
            result.push(item.0.clone());
        }

        result.sort_by(|x, y| y.score.partial_cmp(&x.score).unwrap());
        result
    }

    pub fn push_result(&mut self, arts: [u64; 5], score: f64) {
        if self.result_hash.contains(&arts) {
            return;
        }

        self.results.push(Reverse(OptimizationIntermediateResult {
            arts: arts.clone(), score
        }));
        self.result_hash.insert(arts);

        if self.results.len() >= self.size + 1 {
            let pop = self.results.pop().unwrap();
            self.result_hash.remove(&pop.0.arts);
        }
    }

    pub fn current_least(&self) -> f64 {
        if self.results.len() == 0 {
            0.0
        } else {
            self.results.peek().unwrap().0.score
        }
    }
}

pub fn get_super_artifacts(arts: &[&Artifact]) -> HashMap<(ArtifactSetName, usize, StatName), Artifact> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let set_name = art.set_name;
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result.entry((set_name, slot as usize, main_stat)).or_insert(Artifact::new(
            set_name, slot, 20, 5, vec![], (main_stat, art.main_stat.1)
        ));
        for sub_slot in art.sub_stats.iter() {
            let mut flag = false;
            for i in entry.sub_stats.iter_mut() {
                if i.0 == sub_slot.0 {
                    i.1 = i.1.max(sub_slot.1);
                    flag = true;
                    break;
                }
            }
            if !flag {
                entry.sub_stats.push((sub_slot.0, sub_slot.1));
            }
        }
    }

    result
}

pub fn get_super_artifacts_without_set(arts: &[&Artifact]) -> HashMap<(usize, StatName), Artifact> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result.entry((slot as usize, main_stat)).or_insert(Artifact::new(
            ArtifactSetName::Empty, slot, 20, 5, vec![], (main_stat, art.main_stat.1)
        ));
        for sub_slot in art.sub_stats.iter() {
            let mut flag = false;
            for i in entry.sub_stats.iter_mut() {
                if i.0 == sub_slot.0 {
                    i.1 = i.1.max(sub_slot.1);
                    flag = true;
                    break;
                }
            }
            if !flag {
                entry.sub_stats.push((sub_slot.0, sub_slot.1));
            }
        }
    }

    result
}

pub fn get_artifacts_group<'a>(arts: &[&'a Artifact]) -> HashMap<(ArtifactSetName, usize, StatName), Vec<&'a Artifact>> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let set_name = art.set_name;
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result.entry((set_name, slot as usize, main_stat)).or_insert(Vec::new());
        entry.push(*art);
    }

    result
}

pub fn get_artifacts_group_without_set<'a>(arts: &[&'a Artifact]) -> HashMap<(usize, StatName), Vec<&'a Artifact>> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result.entry((slot as usize, main_stat)).or_insert(Vec::new());
        entry.push(*art);
    }

    result
}

pub fn get_set_names(arts: &[&Artifact]) -> Vec<ArtifactSetName> {
    let mut set = HashSet::new();
    for art in arts.iter() {
        set.insert(art.set_name);
    }

    set.into_iter().collect()
}