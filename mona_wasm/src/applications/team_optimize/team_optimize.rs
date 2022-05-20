use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use smallvec::{SmallVec, smallvec};
use crate::applications::common::{CharacterInterface, WeaponInterface};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult, OptimizeArtifactInterface};
use crate::applications::team_optimize::inter::TeamInterface;
use crate::applications::optimize_artifacts::single_optimize::{optimize_single_interface_wasm};
use crate::applications::team_optimize::hyper_param::TeamOptimizeHyperParam;
use mona::artifacts::Artifact;
use mona::attribute::SimpleAttributeGraph2;
use mona::buffs::Buff;
use mona::character::{Character, CharacterName};
use mona::target_functions::TargetFunction;
use mona::team::team::Team;
// use mona::team_target::team_target_function::TeamTargetFunction;
use mona::weapon::Weapon;
use mona::log;


const MAX_TEAM_COUNT: usize = 8;

#[derive(Clone)]
pub struct ArtifactSet {
    pub items: HashSet<u64>,
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

    pub fn to_small_vec(&self) -> SmallVec<[u64; 5]> {
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
        let mut items = HashSet::new();
        for i in other.items.iter() {
            if self.items.contains(i) {
                items.insert(*i);
            }
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

    pub fn contains(&self, value: u64) -> bool {
        self.items.contains(&value)
    }
}

struct SearchStackContent {
    col_index: usize,
    row_index: usize,
    value_self: f64,
    current_value: f64,
    invalid_set: ArtifactSet,
    indices: SmallVec<[usize; MAX_TEAM_COUNT]>,
}

struct HeapItem {
    pub indices: SmallVec<[usize; MAX_TEAM_COUNT]>,
    pub value: f64,
}

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for HeapItem {}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

fn try_search(
    nodes: &[Vec<ArtifactSet>],
    max_iter: usize,
    // team_target_function: &Box<dyn TeamTargetFunction>,
    weights: &[f64],
    count: usize,
) -> Result<Vec<SmallVec<[usize; MAX_TEAM_COUNT]>>, HashMap<u64, usize>> {
    // let mut invalid_set = ArtifactSet::new();
    // let mut iter: usize = 0;
    // try_search_helper(nodes, 0, &invalid_set, &mut iter, max_iter)

    let mut stack: Vec<SearchStackContent> = Vec::new();
    let mut iter_count = 0_usize;
    // let mut max_value: f64 = 0.0;
    // let mut max_indices: SmallVec<[usize; MAX_TEAM_COUNT]> = SmallVec::new();
    let mut mva_map: HashMap<u64, usize> = HashMap::new(); // artifact id -> count
    for (row, node) in nodes[0].iter().enumerate().rev() {
        stack.push(SearchStackContent {
            col_index: 0,
            row_index: row,
            current_value: node.value * weights[0],
            value_self: node.value,
            invalid_set: node.clone(),
            indices: smallvec![row],
        });
    }

    let mut heap = BinaryHeap::with_capacity(count + 1);
    while !stack.is_empty() && iter_count < max_iter {
        let p = stack.pop().unwrap();
        if p.col_index == nodes.len() - 1 {
            iter_count += 1;

            // let mut value_map = HashMap::new();
            // for col in 0..p.indices.len() {
            //     let value = nodes[col][p.indices[col]].value;
            //     value_map.insert(character_names[col] as usize, value);
            // }
            // let team_target_value = team_target_function.target(&value_map);
            let value = p.current_value;


            let heap_item = HeapItem {
                indices: p.indices.clone(),
                value,
            };
            if heap.len() < count {
                heap.push(Reverse(heap_item));
            } else {
                heap.push(Reverse(heap_item));
                heap.pop();
            }

            // if team_target_value > max_value {
            //     max_value = team_target_value;
            //     max_indices = p.indices.clone();
            // }
        } else {
            for (row, node) in nodes[p.col_index + 1].iter().enumerate().rev() {
                let intersect = p.invalid_set.intersect(node);
                if intersect.is_empty() {
                    let mut ind = p.indices.clone();
                    ind.push(row);
                    stack.push(SearchStackContent {
                        col_index: p.col_index + 1,
                        row_index: row,
                        value_self: node.value,
                        current_value: p.current_value  + node.value * weights[p.col_index + 1],
                        invalid_set: p.invalid_set.union(node),
                        indices: ind,
                    });
                } else {
                    for &id in intersect.items.iter() {
                        *mva_map.entry(id).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    if heap.len() == 0 {
        Err(mva_map)
    } else {
        // Ok(max_indices)
        let mut results = Vec::new();
        for entry in heap.iter() {
            results.push(entry.0.indices.clone());
        }
        Ok(results)
    }
}

fn optimize_team_helper2(
    artifacts: &[&Artifact],
    single_interfaces: &[OptimizeArtifactInterface],
    weights: &[f64],
    hyper_param: &TeamOptimizeHyperParam,
    single_algo: &Box<dyn SingleOptimizeAlgorithm>
) -> Vec<SmallVec<[ArtifactSet; MAX_TEAM_COUNT]>> {
    let l = single_interfaces.len();

    // nodes[character index][individual optimization result index]
    let mut nodes: Vec<Vec<ArtifactSet>> = Vec::with_capacity(l);

    let optimize_index = |index: usize, arts: &[&Artifact]| -> Vec<ArtifactSet> {
        let algo = single_interfaces[index].algorithm.get_algorithm();
        let result = optimize_single_interface_wasm(&single_interfaces[index], &arts, &algo, hyper_param.work_space);
        result.iter().map(|x| ArtifactSet::from_optimization_result(x)).collect()
    };

    // calc L initial optimization results
    for i in 0..l {
        let column = optimize_index(i, artifacts);
        nodes.push(column);
    }

    // log!("finish initial individual");

    let mut result_indices: Vec<SmallVec<[usize; MAX_TEAM_COUNT]>> = Vec::new();
    let mut re_optimize_count: usize = 0;
    're_optimize: while re_optimize_count < hyper_param.max_re_optimize {
        re_optimize_count += 1;

        match try_search(&nodes, hyper_param.max_search, weights, hyper_param.count) {
            Ok(indices) => {
                // there is a result
                result_indices = indices;
                break 're_optimize;
            },
            Err(mva_map) => {
                // otherwise, re-optimize all columns except the first

                // mva_map(most valuable artifact): artifact id -> conflicting count
                let mut mva_map = mva_map;
                let mut temp: Vec<(u64, usize)> = mva_map.drain().collect();
                temp.sort_by(|x, y| (*y).1.cmp(&(*x).1));

                // exclude_list: artifact ids to be excluded for next re-optimization
                let exclude_list: HashSet<u64> = temp.iter().take(hyper_param.mva_step).map(|x| (*x).0).collect();
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
        let mut results = Vec::new();
        for entry in result_indices.iter() {
            let mut temp: SmallVec<[ArtifactSet; MAX_TEAM_COUNT]> = SmallVec::new();
            for (col, row_index) in entry.iter().enumerate() {
                temp.push(nodes[col][*row_index].clone());
            }
            results.push(temp);
        }

        results
    } else {
        // todo optimize algorithm
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
        let rest = optimize_team_helper2(
            &artifacts_new,
            &single_interfaces[1..],
            &weights[1..],
            &hyper_param,
            &single_algo
        );
        for i in rest[0].iter() {
            ret.push(i.clone());
        }

        vec![ret]
    }
}

// fn optimize_team_helper(
//     artifacts: &[&Artifact],
//     characters: &[Character<SimpleAttributeGraph2>],
//     weapons: &[Weapon<SimpleAttributeGraph2>],
//     buffs: &[Vec<Box<dyn Buff<SimpleAttributeGraph2>>>],
//     target_functions: &[Box<dyn TargetFunction>],
//     constraints: &[ConstraintConfig],
//     // team_target_function: &Box<dyn TeamTargetFunction>,
//     weights: &[f64],
//     hyper_param: &TeamOptimizeHyperParam
// ) -> Vec<SmallVec<[ArtifactSet; MAX_TEAM_COUNT]>> {
//     let l = characters.len();
//     let character_names: Vec<_> = characters.iter().map(|c| c.common_data.name).collect();
//
//     // nodes[character index][individual optimization result index]
//     let mut nodes: Vec<Vec<ArtifactSet>> = Vec::with_capacity(l);
//
//     let optimize_index = |index: usize, arts: &[&Artifact]| -> Vec<ArtifactSet> {
//         let result = optimize_single(
//             artifacts,
//             None,
//             &characters[index],
//             &weapons[index],
//             &target_functions[index],
//             Some(&constraints[index]),
//             &buffs[index],
//             hyper_param.work_space,
//             true
//         );
//         result.iter().map(|x| ArtifactSet::from_optimization_result(x)).collect()
//     };
//
//     // calc L initial optimization results
//     for i in 0..l {
//         let column = optimize_index(i, artifacts);
//         nodes.push(column);
//     }
//
//     utils::log!("finish initial individual");
//
//     let mut result_indices: Vec<SmallVec<[usize; MAX_TEAM_COUNT]>> = Vec::new();
//     let mut re_optimize_count: usize = 0;
//     're_optimize: while re_optimize_count < hyper_param.max_re_optimize {
//         re_optimize_count += 1;
//
//         match try_search(&nodes, hyper_param.max_search, weights, hyper_param.count) {
//             Ok(indices) => {
//                 // there is a result
//                 result_indices = indices;
//                 break 're_optimize;
//             },
//             Err(mva_map) => {
//                 // otherwise, re-optimize all columns except the first
//
//                 // mva_map(most valuable artifact): artifact id -> conflicting count
//                 let mut mva_map = mva_map;
//                 let mut temp: Vec<(u64, usize)> = mva_map.drain().collect();
//                 temp.sort_by(|x, y| (*y).1.cmp(&(*x).1));
//
//                 // exclude_list: artifact ids to be excluded for next re-optimization
//                 let exclude_list: HashSet<u64> = temp.iter().take(hyper_param.mva_step).map(|x| (*x).0).collect();
//                 let mut artifacts_new: Vec<&Artifact> = Vec::new();
//                 for &art in artifacts.iter() {
//                     if !exclude_list.contains(&art.id) {
//                         artifacts_new.push(art);
//                     }
//                 }
//
//                 for i in 1..l {
//                     let col = optimize_index(i, &artifacts_new);
//                     nodes[i] = col;
//                 }
//             }
//         }
//     }
//
//     if result_indices.len() > 0 {
//         // there is a result
//         let mut results = Vec::new();
//         for entry in result_indices.iter() {
//             let mut temp: SmallVec<[ArtifactSet; MAX_TEAM_COUNT]> = SmallVec::new();
//             for (col, row_index) in entry.iter().enumerate() {
//                 temp.push(nodes[col][*row_index].clone());
//             }
//             results.push(temp);
//         }
//
//         results
//     } else {
//         // todo optimize algorithm
//         // after re-optimization, still no result, fallback to naive greedy strategy
//         let mut artifacts_new: Vec<&Artifact> = Vec::new();
//         let need_remove = |art: &Artifact| -> bool {
//             nodes[0][0].contains(art.id)
//         };
//         for &art in artifacts.iter() {
//             if !need_remove(art) {
//                 artifacts_new.push(art);
//             }
//         }
//
//         let mut ret: SmallVec<[ArtifactSet; MAX_TEAM_COUNT]> = SmallVec::new();
//         ret.push(nodes[0][0].clone());
//         let rest = optimize_team_helper(
//             &artifacts_new,
//             &characters[1..],
//             &weapons[1..],
//             &buffs[1..],
//             &target_functions[1..],
//             // &team_target_function,
//             &constraints[1..],
//             &weights[1..],
//             &hyper_param
//         );
//         for i in rest[0].iter() {
//             ret.push(i.clone());
//         }
//
//         vec![ret]
//     }
// }

// pub fn optimize_team<'a>(
//     artifacts: &[&'a Artifact],
//     characters: &[Character<SimpleAttributeGraph2>],
//     weapons: &[Weapon<SimpleAttributeGraph2>],
//     buffs: &[Vec<Box<dyn Buff<SimpleAttributeGraph2>>>],
//     target_functions: &[Box<dyn TargetFunction>],
//     constraints: &[ConstraintConfig],
//     weights: &[f64],
//     // team_target_function: &Box<dyn TeamTargetFunction>,
//     hyper_param: &TeamOptimizeHyperParam
// ) -> Vec<SmallVec<[SmallVec<[u64; 5]>; MAX_TEAM_COUNT]>> {
//     let intermediate = optimize_team_helper(
//         artifacts,
//         characters,
//         weapons,
//         buffs,
//         target_functions,
//         constraints,
//         // team_target_function,
//         weights,
//         hyper_param
//     );
//
//     let mut results = Vec::new();
//     for entry in intermediate.iter() {
//         results.push(entry.iter().map(|x| x.to_small_vec()).collect());
//     }
//
//     results
// }

pub fn optimize_team_multi_single<'a>(
    artifacts: &[&'a Artifact],
    single_interfaces: &[OptimizeArtifactInterface],
    weights: &[f64],
    hyper_param: &TeamOptimizeHyperParam,
    single_algo: &Box<dyn SingleOptimizeAlgorithm>
) -> Vec<SmallVec<[SmallVec<[u64; 5]>; MAX_TEAM_COUNT]>> {
    let intermediate = optimize_team_helper2(&artifacts, &single_interfaces, &weights, &hyper_param, &single_algo);

    let mut results = Vec::new();
    for entry in intermediate.iter() {
        results.push(entry.iter().map(|x| x.to_small_vec()).collect());
    }

    results
}
