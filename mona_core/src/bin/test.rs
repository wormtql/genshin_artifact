use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use smallvec::SmallVec;
use mona::artifacts::{Artifact, ArtifactList, ArtifactSetName, ArtifactSlotName};
use mona::artifacts::artifact_set_type::ArtifactSetType;
use mona::attribute::{AttributeUtils, SimpleAttributeGraph2};
use mona::buffs::Buff;
use mona::character::{Character, CharacterConfig, CharacterName};
use mona::common::StatName;
use mona::target_functions::target_functions::{HuTaoDefaultTargetFunction, MonaDefaultTargetFunction, NilouDefaultTargetFunction};
use mona::target_functions::{TargetFunction, TargetFunctionConfig};
use mona::upgrade_predicate::artifact_upgrader::ArtifactUpgrader;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use strum::EnumCount;
use mona::artifacts::effect_config::ArtifactEffectConfig;

fn get_character() -> Character<SimpleAttributeGraph2> {
    Character::new(
        CharacterName::Nilou,
        90,
        false,
        0,
        9, 9, 9,
        &CharacterConfig::Nilou { golden_rate: 1.0 },
    )
}

fn get_weapon(c: &Character<SimpleAttributeGraph2>) -> Weapon<SimpleAttributeGraph2> {
    Weapon::new(
        WeaponName::KeyOfKhajNisut,
        90,
        false,
        1,
        &WeaponConfig::NoConfig,
        c
    )
}

fn get_tf() -> Box<dyn TargetFunction> {
    // Box::new(HuTaoDefaultTargetFunction::new(&TargetFunctionConfig::HuTaoDefault { vaporize_rate: 0.5, melt_rate: 0.0 }))
    Box::new(NilouDefaultTargetFunction {
        e_ratio: 0.0,
        q_ratio: 0.0,
        bloom_ratio: 1.0,
        other_em: 1000.0,
        other_bloom_ratio: 5.0
    })
}

struct ValueFunction {
    pub character: Character<SimpleAttributeGraph2>,
    pub weapon: Weapon<SimpleAttributeGraph2>,
    pub tf: Box<dyn TargetFunction>,
    pub artifact_config: ArtifactEffectConfig,
    pub buffs: Vec<Box<dyn Buff<SimpleAttributeGraph2>>>,
}

impl ValueFunction {
    fn call(&self, artifacts: &[&Artifact]) -> f64 {
        let artifact_list = ArtifactList {
            artifacts,
        };
        let attribute = AttributeUtils::create_attribute_from_big_config(
            &artifact_list,
            &self.artifact_config,
            &self.character,
            &self.weapon,
            &self.buffs
        );

        self.tf.target(
            &attribute,
            &self.character,
            &self.weapon,
            artifacts,
            &Default::default()
        )
    }

    fn call_with_stat(&self, stat_name: StatName, value: f64) -> f64 {
        let artifact = Artifact::new(
            ArtifactSetName::Empty,
            ArtifactSlotName::Flower,
            0,
            5,
            vec![],
            (stat_name, value)
        );

        self.call(&[&artifact])
    }

    fn call_with_set_type(&self, set_type: ArtifactSetType) -> f64 {
        let create_empty_artifact = |set_name: ArtifactSetName| {
            Artifact::new(
                set_name,
                ArtifactSlotName::Flower,
                0,
                5,
                vec![],
                (StatName::ATKFixed, 0.0)
            )
        };

        let set_names = match set_type {
            ArtifactSetType::Set4(s) => {
                vec![s, s, s, s, ArtifactSetName::Empty]
            },
            ArtifactSetType::Set22(s1, s2) => {
                vec![s1, s1, s2, s2, ArtifactSetName::Empty]
            },
            ArtifactSetType::Set2(s) => {
                vec![s, s, ArtifactSetName::Empty, ArtifactSetName::Empty, ArtifactSetName::Empty]
            },
            ArtifactSetType::Misc => {
                vec![ArtifactSetName::Empty; 5]
            }
        };

        let artifacts = set_names.iter().map(|x| create_empty_artifact(*x)).collect::<Vec<_>>();
        let a2 = artifacts.iter().map(|x| x).collect::<Vec<_>>();
        self.call(&a2)
    }

    fn get_effective_stat_name(&self) -> HashSet<StatName> {
        let mut result = HashSet::new();
        let base_value = self.call(&[]);

        for i in 0..StatName::COUNT {
            let stat_name: StatName = num::FromPrimitive::from_usize(i).unwrap();
            let value = self.call_with_stat(stat_name, 10000.0);
            if value > base_value {
                result.insert(stat_name);
            }
        }

        result
    }

    fn get_effective_artifact_set_type(&self) -> HashSet<ArtifactSetType> {
        let mut result = HashSet::new();
        let base_value = self.call(&[]);

        let len: usize = ArtifactSetName::LEN;

        for i in 0..len {
            let set_name: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();
            let t = ArtifactSetType::Set4(set_name);
            let value = self.call_with_set_type(t);
            if value > base_value {
                result.insert(t);
            }

            let t2 = ArtifactSetType::Set2(set_name);
            let value = self.call_with_set_type(t2);
            if value > base_value {
                result.insert(t2);
            }
        }

        result
    }
}

impl Default for ValueFunction {
    fn default() -> Self {
        let c = get_character();
        let w = get_weapon(&c);
        let tf = get_tf();
        ValueFunction {
            character: c,
            weapon: w,
            tf: get_tf(),
            buffs: vec![],
            // artifact_config: Default::default(),
            artifact_config: tf.get_default_artifact_config(&Default::default()),
        }
    }
}

fn generate_artifacts(set_type: ArtifactSetType, main_stats: &[StatName], upgrader: &ArtifactUpgrader) -> (usize, SmallVec<[Artifact; 5]>) {
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

#[derive(Debug)]
struct UpdateItem {
    pub set_type: ArtifactSetType,
    pub stats: [StatName; 3],
    pub value: f64,
}

impl PartialOrd for UpdateItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl Ord for UpdateItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for UpdateItem {}

impl PartialEq for UpdateItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn main() {
    use StatName::*;
    let upgrader = ArtifactUpgrader::default();


    // println!("{:?}", a);


    let vf = ValueFunction::default();

    let sim_count = 1000;

    let slot_main_stats = StatName::get_slot_main_stats();
    let mut heap: BinaryHeap<UpdateItem> = BinaryHeap::new();
    let mut max_heap: BinaryHeap<UpdateItem> = BinaryHeap::new();

    let effective_stat_name = vf.get_effective_stat_name();
    println!("effective stat name: {:?}", effective_stat_name);
    let effective_set_type = vf.get_effective_artifact_set_type();
    println!("effective set type: {:?}", effective_set_type);

    use ArtifactSetName::*;
    let exclude_list: HashSet<ArtifactSetName> = HashSet::from_iter(vec![
        Adventurer, LuckyDog, TravelingDoctor, ResolutionOfSojourner, TinyMiracle, Berserker, Instructor, TheExile,
        DefendersWill, BraveHeart, MartialArtist, Gambler, Scholar, PrayersForDestiny, PrayersForWisdom, PrayersForIllumination, PrayersToSpringtime,
    ]);

    // iterator set4
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
                    let mut max_value = 0.0;
                    for _ in 0..sim_count {
                        let (tries, a) = generate_artifacts(set_type, &[HPFixed, ATKFixed, s1, s2, s3], &upgrader);
                        let a2: SmallVec<[&Artifact; 5]> = a.iter().map(|x| x).collect();
                        let value = vf.call(&a2);

                        total_value += value;
                        max_value = value.max(max_value);
                    }

                    let value = total_value / (sim_count as f64);
                    heap.push(UpdateItem {
                        set_type,
                        stats: [s1, s2, s3],
                        value,
                    });
                    if heap.len() > 100 {
                        heap.pop();
                    }

                    max_heap.push(UpdateItem {
                        set_type,
                        stats: [s1, s2, s3],
                        value: max_value,
                    });
                    if max_heap.len() > 100 {
                        max_heap.pop();
                    }

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
                        let mut max_value = 0.0;
                        for _ in 0..sim_count {
                            let (tries, a) = generate_artifacts(set_type, &[HPFixed, ATKFixed, s1, s2, s3], &upgrader);
                            let a2: SmallVec<[&Artifact; 5]> = a.iter().map(|x| x).collect();
                            let value = vf.call(&a2);

                            total_value += value;
                            max_value = value.max(max_value);
                        }

                        let value = total_value / (sim_count as f64);
                        heap.push(UpdateItem {
                            set_type,
                            stats: [s1, s2, s3],
                            value,
                        });
                        if heap.len() > 100 {
                            heap.pop();
                        }

                        max_heap.push(UpdateItem {
                            set_type,
                            stats: [s1, s2, s3],
                            value: max_value,
                        });
                        if max_heap.len() > 100 {
                            max_heap.pop();
                        }

                        // println!("{:?}, {:?}, {:?}: {}", s1, s2, s3, total_value / (sim_count as f64));
                    }
                }
            }
        }
    }

    for _ in 0..heap.len() {
        println!("{:?}", heap.pop().unwrap());
    }

    // for _ in 0..max_heap.len() {
    //     println!("{:?}", max_heap.pop().unwrap());
    // }

    // println!("{}", total_value / (sim_count as f64));
}