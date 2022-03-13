use crate::artifacts::Artifact;
use crate::artifacts::eff::{ARTIFACT_EFF4, ARTIFACT_EFF5};
use crate::common::StatName;
use crate::potential_function::potential_function::PotentialFunction;

pub trait PotentialFunctionEngine {
    fn value(&self, potential_function: &Box<dyn PotentialFunction>, artifact: &Artifact) -> f64;
}

pub struct NaivePotentialFunctionEngine;

impl PotentialFunctionEngine for NaivePotentialFunctionEngine {
    fn value(&self, potential_function: &Box<dyn PotentialFunction>, artifact: &Artifact) -> f64 {
        potential_function.potential(&artifact)
    }
}


pub struct ExpectationPotentialFunctionEngine;

fn expectation_helper(
    artifact: *mut Artifact,
    pf: &Box<dyn PotentialFunction>,
    effective_stat_names: &[StatName]
) -> f64 {
    let artifact_ref = unsafe { &*artifact };

    if artifact_ref.is_max_level() {
        // println!("1");
        let v = pf.potential(&artifact_ref);
        // println!("{}", v);
        return v;
    }

    let mut result = 0.0;

    let sub_stat_count = artifact_ref.sub_stats.len();
    if sub_stat_count == 4 {
        for (index, stat) in artifact_ref.sub_stats.iter().enumerate() {
            let old_level = artifact_ref.level;
            let delta_level = (old_level / 4 * 4 + 4) - old_level;

            let is_effective_stat_name = effective_stat_names.contains(&stat.0);
            if is_effective_stat_name {
                let old_value = stat.1;

                for i in 0..4 {
                    let delta_value = if artifact_ref.star == 5 {
                        ARTIFACT_EFF5.get_value(stat.0, i)
                    } else {
                        ARTIFACT_EFF4.get_value(stat.0, i)
                    };
                    unsafe {
                        (*artifact).sub_stats[index].1 += delta_value;
                        (*artifact).level += delta_level;
                    }

                    result += expectation_helper(artifact, &pf, &effective_stat_names) * 0.0625;

                    unsafe {
                        (*artifact).sub_stats[index].1 = old_value;
                        (*artifact).level = old_level;
                    }
                }
            } else {
                // let v = pf.potential(&artifact_ref);
                unsafe {
                    (*artifact).level += delta_level;
                }
                result += 0.25 * expectation_helper(artifact, &pf, &effective_stat_names);
                unsafe {
                    (*artifact).level = old_level;
                }
            }
        }
    } else {
        let dist = artifact_ref.get_next_stat_name_dist().unwrap();
        // println!("{:?}", dist);

        let old_level = artifact_ref.level;
        let delta_level = (old_level / 4 * 4 + 4) - old_level;

        for dist_entry in dist.iter() {
            let stat_name = dist_entry.0;
            let prob = dist_entry.1;

            if !effective_stat_names.contains(&stat_name) {
                unsafe {
                    (*artifact).level += delta_level;
                    (*artifact).sub_stats.push((stat_name, 0.0));
                }
                result += prob * expectation_helper(artifact, &pf, &effective_stat_names);
                unsafe {
                    (*artifact).level = old_level;
                    (*artifact).sub_stats.pop();
                }
            } else {
                for i in 0..4 {
                    let delta_value = if artifact_ref.star == 5 {
                        ARTIFACT_EFF5.get_value(stat_name, i)
                    } else {
                        ARTIFACT_EFF4.get_value(stat_name, i)
                    };

                    unsafe {
                        (*artifact).sub_stats.push((stat_name, delta_value));
                        (*artifact).level += delta_level;
                    }

                    result += prob * 0.25 * expectation_helper(artifact, &pf, &effective_stat_names);

                    unsafe {
                        (*artifact).sub_stats.pop();
                        (*artifact).level = old_level;
                    }
                }
            }
        }
    }

    result
}

impl PotentialFunctionEngine for ExpectationPotentialFunctionEngine {
    fn value(&self, potential_function: &Box<dyn PotentialFunction>, artifact: &Artifact) -> f64 {
        let mut artifact = artifact.clone();
        let effective_stat_names = potential_function.get_effective_stats();

        expectation_helper(&mut artifact, &potential_function, &effective_stat_names)
    }
}

// pub enum PotentialFunctionEngineName {
//     Naive,
//     Expect,
// }
