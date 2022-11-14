use std::collections::{BinaryHeap, HashSet};
use mona::artifacts::{Artifact, ArtifactSetName};
use mona::artifacts::artifact_set_type::ArtifactSetType;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::SimpleAttributeGraph2;
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::upgrade_predicate::artifact_upgrader::ArtifactUpgrader;
use mona::weapon::Weapon;
use smallvec::SmallVec;
use crate::applications::artifact_best_set::update_item::UpdateItem;
use crate::applications::artifact_best_set::value_function::ValueFunction;

pub fn generate_artifacts(set_type: ArtifactSetType, main_stats: &[StatName], upgrader: &ArtifactUpgrader) -> (usize, SmallVec<[Artifact; 5]>) {
    let mut artifacts: SmallVec<[Artifact; 5]> = SmallVec::new();
    let mut tries = 0;

    let mut i = 0;
    while i < 5 {
        tries += 1;
        let mut a = Artifact::random0(ArtifactSetName::BlizzardStrayer, num::FromPrimitive::from_usize(i).unwrap(), main_stats[i]);
        if upgrader.upgrade(&mut a) {
            artifacts.push(a);
            i += 1;
        }
    }

    match set_type {
        ArtifactSetType::Set4(s) => {
            for i in 0..4 {
                artifacts[i].set_name = s;
            }
            artifacts[4].set_name = ArtifactSetName::Empty;
        },
        ArtifactSetType::Set22(s1, s2) => {
            for i in 0..2 {
                artifacts[i].set_name = s1;
                artifacts[i + 2].set_name = s2;
            }
            artifacts[4].set_name = ArtifactSetName::Empty;
        },
        ArtifactSetType::Set2(s) => {
            for i in 0..2 {
                artifacts[i].set_name = s;
            }
            for i in 2..5 {
                artifacts[i].set_name = ArtifactSetName::Empty;
            }
        },
        ArtifactSetType::Misc => ()
    }

    (tries, artifacts)
}

pub fn calc_artifact_best_set(
    character: &Character<SimpleAttributeGraph2>,
    weapon: &Weapon<SimpleAttributeGraph2>,
    target_function: &Box<dyn TargetFunction>,
    artifact_config: Option<&ArtifactEffectConfig>,
    buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>],
    enemy: &Enemy,
) -> BinaryHeap<UpdateItem> {
    let mut default_effect_config: ArtifactEffectConfig;
    let effect_config_ref = if let Some(x) = artifact_config {
        x
    } else {
        default_effect_config = target_function.get_default_artifact_config(&Default::default());
        &default_effect_config
    };

    let vf = ValueFunction {
        character,
        weapon,
        tf: &target_function,
        artifact_config: &effect_config_ref,
        buffs,
        enemy
    };

    use StatName::*;
    let upgrader = ArtifactUpgrader::default();

    let sim_count = 5000;
    let heap_size = 100;

    let slot_main_stats = StatName::get_slot_main_stats();
    let mut heap: BinaryHeap<UpdateItem> = BinaryHeap::new();
    // let mut max_heap: BinaryHeap<UpdateItem> = BinaryHeap::new();

    let effective_stat_name = vf.get_effective_stat_name();
    // println!("effective stat name: {:?}", effective_stat_name);
    let effective_set_type = vf.get_effective_artifact_set_type();
    // println!("effective set type: {:?}", effective_set_type);

    use ArtifactSetName::*;
    let exclude_list: HashSet<ArtifactSetName> = HashSet::from_iter(vec![
        Adventurer, LuckyDog, TravelingDoctor, ResolutionOfSojourner, TinyMiracle, Berserker, Instructor, TheExile,
        DefendersWill, BraveHeart, MartialArtist, Gambler, Scholar, PrayersForDestiny, PrayersForWisdom, PrayersForIllumination, PrayersToSpringtime,
    ]);

    // iterate set4
    for s in 0..ArtifactSetName::LEN {
        let set_name: ArtifactSetName = num::FromPrimitive::from_usize(s).unwrap();
        let set_type = ArtifactSetType::Set4(set_name);

        if !effective_set_type.contains(&set_type) || exclude_list.contains(&set_name) {
            continue;
        }

        for &s1 in slot_main_stats[2].iter() {
            if !effective_stat_name.contains(&s1) {
                continue;
            }
            for &s2 in slot_main_stats[3].iter() {
                if !effective_stat_name.contains(&s2) {
                    continue;
                }
                for &s3 in slot_main_stats[4].iter() {
                    if !effective_stat_name.contains(&s3) {
                        continue;
                    }
                    let mut total_value = 0.0;
                    // let mut max_value = 0.0;
                    let mut sim_iter = 0;
                    while sim_iter < sim_count {
                        let (tries, a) = generate_artifacts(set_type, &[HPFixed, ATKFixed, s1, s2, s3], &upgrader);
                        sim_iter += tries;
                        let a2: SmallVec<[&Artifact; 5]> = a.iter().map(|x| x).collect();
                        let value = vf.call(&a2);

                        total_value += value * (tries as f64);
                        // max_value = value.max(max_value);
                    }

                    let value = total_value / (sim_iter as f64);
                    heap.push(UpdateItem {
                        set_type,
                        stats: [s1, s2, s3],
                        value,
                    });
                    if heap.len() > heap_size {
                        heap.pop();
                    }

                    // max_heap.push(UpdateItem {
                    //     set_type,
                    //     stats: [s1, s2, s3],
                    //     value: max_value,
                    // });
                    // if max_heap.len() > heap_size {
                    //     max_heap.pop();
                    // }

                    // println!("{:?}, {:?}, {:?}: {}", s1, s2, s3, total_value / (sim_count as f64));
                }
            }
        }
    }

    for i in 0..ArtifactSetName::LEN {
        let set_name1: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();

        if !effective_set_type.contains(&ArtifactSetType::Set2(set_name1)) || exclude_list.contains(&set_name1) {
            continue;
        }

        for j in i + 1..ArtifactSetName::LEN {
            let set_name2: ArtifactSetName = num::FromPrimitive::from_usize(j).unwrap();
            if !effective_set_type.contains(&ArtifactSetType::Set2(set_name2)) || exclude_list.contains(&set_name2) {
                continue;
            }

            let set_type = ArtifactSetType::Set22(set_name1, set_name2);

            for &s1 in slot_main_stats[2].iter() {
                if !effective_stat_name.contains(&s1) {
                    continue;
                }
                for &s2 in slot_main_stats[3].iter() {
                    if !effective_stat_name.contains(&s2) {
                        continue;
                    }
                    for &s3 in slot_main_stats[4].iter() {
                        if !effective_stat_name.contains(&s3) {
                            continue;
                        }
                        let mut total_value = 0.0;
                        // let mut max_value = 0.0;
                        let mut sim_iter = 0;
                        while sim_iter < sim_count {
                            let (tries, a) = generate_artifacts(set_type, &[HPFixed, ATKFixed, s1, s2, s3], &upgrader);
                            sim_iter += tries;
                            let a2: SmallVec<[&Artifact; 5]> = a.iter().map(|x| x).collect();
                            let value = vf.call(&a2);

                            total_value += value * (tries as f64);
                            // max_value = value.max(max_value);
                        }

                        let value = total_value / (sim_iter as f64);
                        heap.push(UpdateItem {
                            set_type,
                            stats: [s1, s2, s3],
                            value,
                        });
                        if heap.len() > 100 {
                            heap.pop();
                        }

                        // max_heap.push(UpdateItem {
                        //     set_type,
                        //     stats: [s1, s2, s3],
                        //     value: max_value,
                        // });
                        // if max_heap.len() > 100 {
                        //     max_heap.pop();
                        // }

                        // println!("{:?}, {:?}, {:?}: {}", s1, s2, s3, total_value / (sim_count as f64));
                    }
                }
            }
        }
    }

    // for _ in 0..heap.len() {
    //     println!("{:?}", heap.pop().unwrap());
    // }

    heap
}
