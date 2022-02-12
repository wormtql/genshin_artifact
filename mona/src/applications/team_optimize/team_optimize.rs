use std::collections::{HashMap, HashSet};
use smallvec::{SmallVec, smallvec};
use crate::applications::common::{CharacterInterface, WeaponInterface};
use crate::applications::optimize_artifacts::inter::OptimizationResult;
use crate::applications::team_optimize::inter::TeamInterface;
use crate::applications::optimize_artifacts::single_optimize::optimize_single;
use crate::applications::team_optimize::hyper_param::TeamOptimizeHyperParam;
use crate::artifacts::Artifact;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::character::characters::jean::JeanRoleEnum::Default;
use crate::target_functions::TargetFunction;
use crate::team::team::Team;
use crate::weapon::Weapon;


const MAX_TEAM_COUNT: usize = 4;

#[derive(Clone)]
pub struct ArtifactSet {
    pub items: HashSet<usize>,
    pub value: f64,
}

impl ArtifactSet {
    pub fn new() -> ArtifactSet {
        ArtifactSet {
            items: HashSet::with_capacity(20),
            value: 0.0
        }
    }

    pub fn from_optimization_result(result: &OptimizationResult) -> ArtifactSet {
        let mut set = HashSet::new();
        if result.flower.is_some() {
            set.insert(result.flower.unwrap());
        }
        if result.feather.is_some() {
            set.insert(result.feather.unwrap());
        }
        if result.sand.is_some() {
            set.insert(result.sand.unwrap());
        }
        if result.goblet.is_some() {
            set.insert(result.goblet.unwrap());
        }
        if result.head.is_some() {
            set.insert(result.head.unwrap());
        }

        ArtifactSet {
            items: set,
            value: result.ratio
        }
    }

    pub fn to_small_vec(&self) -> SmallVec<[usize; 5]> {
        self.items.iter().cloned().collect()
    }

    pub fn union(&self, other: &ArtifactSet) -> ArtifactSet {
        let mut items = self.items.clone();
        for i in other.items.iter() {
            items.insert(*i);
        }

        ArtifactSet {
            items,
            value: 0.0
        }
    }

    pub fn intersect(&self, other: &ArtifactSet) -> ArtifactSet {
        let mut items = self.items.clone();
        for i in other.items.iter() {
            items.remove(i);
        }

        ArtifactSet {
            items,
            value: 0.0
        }
    }

    pub fn is_intersect(&self, other: &ArtifactSet) -> bool {
        for item in other.items.iter() {
            if self.items.contains(item) {
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains(&self, value: usize) -> bool {
        self.items.contains(&value)
    }
}

struct SearchStackContent {
    col_index: usize,
    row_index: usize,
    current_value: f64,
    invalid_set: ArtifactSet,
    indices: SmallVec<[usize; 4]>,
}

fn try_search(nodes: &[Vec<ArtifactSet>], max_iter: usize) -> Result<SmallVec<[usize; MAX_TEAM_COUNT]>, HashMap<usize, usize>> {
    // let mut invalid_set = ArtifactSet::new();
    // let mut iter: usize = 0;
    // try_search_helper(nodes, 0, &invalid_set, &mut iter, max_iter)

    let mut stack: Vec<SearchStackContent> = Vec::new();
    let mut iter_count = 0_usize;
    let mut max_value: f64 = 0.0;
    let mut max_indices: SmallVec<[usize; MAX_TEAM_COUNT]> = SmallVec::new();
    let mut mva_map: HashMap<usize, usize> = HashMap::new(); // artifact id -> count
    for (row, node) in nodes[0].iter().enumerate().rev() {
        stack.push(SearchStackContent {
            col_index: 0,
            row_index: row,
            current_value: node.value,
            invalid_set: node.clone(),
            indices: smallvec![row],
        });
    }

    while !stack.is_empty() && iter_count < max_iter {
        let p = stack.pop().unwrap();
        if p.col_index == nodes.len() - 1 {
            iter_count += 1;
            if p.current_value > max_value {
                max_value = p.current_value;
                max_indices = p.indices.clone();
            }
        }
        for (row, node) in nodes[p.col_index + 1].iter().enumerate().rev() {
            let intersect = p.invalid_set.intersect(node);
            if intersect.is_empty() {
                let mut ind = p.indices.clone();
                ind.push(row);
                stack.push(SearchStackContent {
                    col_index: p.col_index + 1,
                    row_index: row,
                    current_value: p.current_value * node.value,
                    invalid_set: p.invalid_set.intersect(node),
                    indices: ind,
                });
            } else {
                for &id in intersect.items.iter() {
                    *mva_map.entry(id).or_insert(0) += 1;
                }
            }
        }
    }

    if max_indices.len() == 0 {
        Err(mva_map)
    } else {
        Ok(max_indices)
    }
}

fn optimize_team_helper(
    artifacts: &[&Artifact],
    characters: &[&Character<SimpleAttributeGraph2>],
    weapons: &[&Weapon<SimpleAttributeGraph2>],
    target_functions: &[&Box<dyn TargetFunction>],
    hyper_param: &TeamOptimizeHyperParam
) -> SmallVec<[ArtifactSet; MAX_TEAM_COUNT]> {
    let l = characters.len();
    let mut nodes: Vec<Vec<ArtifactSet>> = Vec::with_capacity(l);

    let optimize_index = |index: usize, arts: &[&Artifact]| -> Vec<ArtifactSet> {
        let result = optimize_single(
            artifacts, None, characters[index], weapons[index],
            target_functions[index], None, &[], hyper_param.work_space
        );
        result.iter().map(|x| ArtifactSet::from_optimization_result(x)).collect()
    };

    // calc L initial optimization results
    for i in 0..l {
        let column = optimize_index(i, artifacts);
        nodes.push(column);
    }

    let mut result_indices: SmallVec<[usize; MAX_TEAM_COUNT]> = SmallVec::new();
    let mut re_optimize_count: usize = 0;
    're_optimize: while re_optimize_count < hyper_param.max_re_optimize {
        re_optimize_count += 1;

        match try_search(&nodes, hyper_param.max_search) {
            Ok(indices) => {
                // there is a result
                result_indices = indices;
                break 're_optimize;
            },
            Err(mva_map) => {
                // otherwise, re-optimize all columns except the first
                let mut mva_map = mva_map;
                let mut temp: Vec<(usize, usize)> = mva_map.drain().collect();
                temp.sort_by(|x, y| (*y).1.cmp(&(*x).1));
                let exclude_list: HashSet<usize> = temp.iter().take(hyper_param.mva_step).map(|x| (*x).0).collect();
                let mut artifacts_new: Vec<&Artifact> = Vec::new();
                for &art in artifacts.iter() {
                    if !exclude_list.contains(&art.id) {
                        artifacts_new.push(art);
                    }
                }
                for i in 1..l {
                    let col = optimize_index(i, &artifacts_new);
                    nodes[i] = col;
                }
            }
        }
    }

    if result_indices.len() > 0 {
        // there is a result
        let mut ret: SmallVec<[ArtifactSet; MAX_TEAM_COUNT]> = SmallVec::new();
        for (index, row_index) in result_indices.iter().enumerate() {
            ret.push(nodes[index][*row_index].clone());
        }

        ret
    } else {
        // after re-optimization, still no result, fallback to naive greedy strategy
        let mut artifacts_new: Vec<&Artifact> = Vec::new();
        let need_remove = |art: &Artifact| -> bool {
            nodes[0][0].contains(art.id)
        };
        for &art in artifacts.iter() {
            if !need_remove(art) {
                artifacts_new.push(art);
            }
        }

        let mut ret: SmallVec<[ArtifactSet; MAX_TEAM_COUNT]> = SmallVec::new();
        ret.push(nodes[0][0].clone());
        let rest = optimize_team_helper(
            &artifacts_new, &characters[1..], &weapons[1..], &target_functions[1..],
            hyper_param
        );
        for i in rest {
            ret.push(i);
        }

        ret
    }
}

pub fn optimize_team<'a>(
    artifacts: &[&'a Artifact],
    characters: &[&Character<SimpleAttributeGraph2>],
    weapons: &[&Weapon<SimpleAttributeGraph2>],
    target_functions: &[&Box<dyn TargetFunction>],
    hyper_param: &TeamOptimizeHyperParam
) -> SmallVec<[SmallVec<[usize; 5]>; MAX_TEAM_COUNT]> {
    let intermediate = optimize_team_helper(
        artifacts,
        characters,
        weapons,
        target_functions,
        hyper_param
    );

    intermediate.iter().map(|x| x.to_small_vec()).collect()
}
