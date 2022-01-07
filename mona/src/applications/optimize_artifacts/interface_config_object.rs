use std::collections::{BinaryHeap, HashMap};
use std::cmp::{PartialOrd, Eq, PartialEq, Ord, Ordering, Reverse};

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName, ArtifactList};
use crate::character::{CharacterConfig, CharacterName, Character};
use crate::weapon::{WeaponName, WeaponConfig, Weapon};
use crate::target_functions::{TargetFunctionName, TargetFunctionConfig, TargetFunctionUtils, TargetFunction};
use crate::common::StatName;
use crate::buffs::BuffType;
use crate::attribute::attribute_utils::AttributeUtils;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeNoReactive, AttributeName};
use crate::enemies::Enemy;
use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct CharacterInterface {
    pub name: CharacterName,
    pub level: i32,
    pub ascend: bool,
    pub constellation: i32,
    pub params: CharacterConfig,
}

impl CharacterInterface {
    pub fn from_js(val: &JsValue) -> CharacterInterface {
        let interface: CharacterInterface = val.into_serde().unwrap();

        interface
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeaponInterface {
    pub name: WeaponName,
    pub level: i32,
    pub ascend: bool,
    pub refine: i32,
    pub params: WeaponConfig,
}

impl WeaponInterface {
    pub fn from_js(val: &JsValue) -> WeaponInterface {
        let interface: WeaponInterface = val.into_serde().unwrap();

        interface
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetFunctionInterface {
    pub name: TargetFunctionName,
    pub params: TargetFunctionConfig,
}

impl TargetFunctionInterface {
    pub fn from_js(val: &JsValue) -> TargetFunctionInterface {
        let interface: TargetFunctionInterface = val.into_serde().unwrap();

        interface
    }
}

#[derive(Serialize, Deserialize)]
pub enum ConstraintSetMode {
    Any,
    Set2(ArtifactSetName),
    Set4(ArtifactSetName),
    Set22(ArtifactSetName, ArtifactSetName),
}

#[derive(Serialize, Deserialize)]
pub struct ConstraintConfig {
    pub set_mode: Option<ConstraintSetMode>,

    // pub main_tag_sand: Option<StatName>,
    // pub main_tag_goblet: Option<StatName>,
    // pub main_tag_head: Option<StatName>,

    pub hp_min: Option<f64>,
    pub atk_min: Option<f64>,
    pub def_min: Option<f64>,
    pub recharge_min: Option<f64>,
    pub em_min: Option<f64>,
    pub crit_min: Option<f64>,
    pub crit_dmg_min: Option<f64>,

    // pub min_level: Option<i32>,
    // pub max_level: Option<i32>,
}

impl ConstraintConfig {
    pub fn from_js(val: &JsValue) -> ConstraintConfig {
        utils::set_panic_hook();
        let ret: ConstraintConfig = val.into_serde().unwrap();

        ret
    }
}

#[derive(Serialize, Deserialize)]
pub struct OptimizeArtifactInterface {
    pub artifacts: Vec<Artifact>,
    pub artifact_config: ArtifactEffectConfig,
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub target_function: TargetFunctionInterface,
    pub constraint: ConstraintConfig,
    pub buffs: Vec<BuffType>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PerStatBonus {
    pub atk_fixed: f64,
    pub atk_percentage: f64,
    pub def_fixed: f64,
    pub def_percentage: f64,
    pub hp_fixed: f64,
    pub hp_percentage: f64,
    pub elemental_mastery: f64,
    pub recharge: f64,
    pub critical: f64,
    pub critical_damage: f64,
}

#[derive(Serialize, Deserialize)]
pub struct OptimizationResult {
    flower: Option<Artifact>,
    feather: Option<Artifact>,
    sand: Option<Artifact>,
    goblet: Option<Artifact>,
    head: Option<Artifact>,
    attribute: AttributeNoReactive,
    per_stat_bonus: PerStatBonus,
    value: f64,
}

pub struct OptimizationIntermediateResult {
    flower_index: usize,
    feather_index: usize,
    sand_index: usize,
    goblet_index: usize,
    head_index: usize,
    attribute_graph: AttributeGraph,
    value: f64,
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

#[wasm_bindgen]
impl OptimizeArtifactInterface {
    pub fn from_js(val: &JsValue) -> String {

        let ret: OptimizeArtifactInterface = val.into_serde().unwrap();

        println!("{}", ret.artifacts.len());

        String::from("zxc")
    }

    fn check_attribute(attribute: &AttributeGraph, constraint: &ConstraintConfig) -> bool {
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
        let critical = attribute.get_value(AttributeName::CriticalBase).min(1.0).max(0.0);
        if critical < constraint.crit_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_value(AttributeName::CriticalDamageBase) < constraint.crit_dmg_min.unwrap_or(0.0) {
            return false;
        }

        true
    }

    fn check_artifact_set(list: &Vec<Option<&Artifact>>, constraint: &ConstraintConfig) -> bool {
        let mut set_name_count: HashMap<ArtifactSetName, i32> = HashMap::new();

        match constraint.set_mode {
            None => return true,
            Some(ref mode) => {
                match *mode {
                    ConstraintSetMode::Any => return true,
                    ConstraintSetMode::Set2(set_name) => {
                        set_name_count.insert(set_name, 2);
                    },
                    ConstraintSetMode::Set22(s1, s2) => {
                        set_name_count.insert(s1, 2);
                        set_name_count.insert(s2, 2);
                    },
                    ConstraintSetMode::Set4(s1) => {
                        set_name_count.insert(s1, 4);
                    }
                }
            }
        }

        for &maybe_artifact in list.iter() {
            if let Some(artifact) = maybe_artifact {
                *set_name_count.entry(artifact.set_name).or_insert(0) -= 1;
            }
        }

        let remain: i32 = set_name_count.iter().filter(|x| *x.1 > 0)
            .map(|x| *x.1).sum();

        if 5 - list.len() < remain as usize {
            false
        } else {
            true
        }
    }

    pub fn optimize(val: &JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: OptimizeArtifactInterface = val.into_serde().unwrap();

        let character = Character::new(
            input.character.name,
            input.character.level,
            input.character.ascend,
            input.character.constellation,
            &input.character.params
        );
        utils::log!("character created");
        let weapon = Weapon::new(
            input.weapon.name,
            input.weapon.level,
            input.weapon.ascend,
            input.weapon.refine,
            &input.weapon.params,
            &character
        );
        utils::log!("weapon created");
        let target_function = TargetFunctionUtils::new_target_function(
            input.target_function.name,
            &character,
            &weapon,
            &input.target_function.params
        );
        utils::log!("target function created");
        let enemy = Enemy::default();
        utils::log!("1234");

        let mut flowers: Vec<&Artifact> = vec![];
        let mut feathers: Vec<&Artifact> = vec![];
        let mut sands: Vec<&Artifact> = vec![];
        let mut goblets: Vec<&Artifact> = vec![];
        let mut heads: Vec<&Artifact> = vec![];

        for artifact in input.artifacts.iter() {
            match artifact.slot {
                ArtifactSlotName::Flower => flowers.push(artifact),
                ArtifactSlotName::Feather => feathers.push(artifact),
                ArtifactSlotName::Sand => sands.push(artifact),
                ArtifactSlotName::Goblet => goblets.push(artifact),
                ArtifactSlotName::Head => heads.push(artifact),
            }
        }

        const RECORDS_COUNT: usize = 5;
        let mut heap = BinaryHeap::with_capacity(RECORDS_COUNT + 1);
        for flower_i in 0..=flowers.len() {
            for feather_i in 0..=feathers.len() {
                // if !OptimizeArtifactInterface::check_artifact_set(
                //     &vec![
                //         flowers.get(flower_i).cloned(),
                //         feathers.get(feather_i).cloned()],
                //     &input.constraint
                // ) {
                //     continue;
                // }
                for sand_i in 0..=sands.len() {
                    // if !OptimizeArtifactInterface::check_artifact_set(
                    //     &vec![
                    //         flowers.get(flower_i).cloned(),
                    //         feathers.get(feather_i).cloned(),
                    //         sands.get(sand_i).cloned()
                    //     ],
                    //     &input.constraint
                    // ) {
                    //     continue;
                    // }
                    for goblet_i in 0..=goblets.len() {
                        // if !OptimizeArtifactInterface::check_artifact_set(
                        //     &vec![
                        //         flowers.get(flower_i).cloned(),
                        //         feathers.get(feather_i).cloned(),
                        //         sands.get(feather_i).cloned(),
                        //         goblets.get(goblet_i).cloned(),
                        //     ],
                        //     &input.constraint
                        // ) {
                        //     continue;
                        // }
                        for head_i in 0..=heads.len() {
                            // let list = vec![
                            //     flowers.get(flower_i).cloned(),
                            //     feathers.get(feather_i).cloned(),
                            //     sands.get(feather_i).cloned(),
                            //     goblets.get(goblet_i).cloned(),
                            //     heads.get(head_i).cloned(),
                            // ];
                            // if !OptimizeArtifactInterface::check_artifact_set(
                            //     &list, &input.constraint
                            // ) {
                            //     continue;
                            // }

                            let mut artifacts: Vec<&Artifact> = vec![];
                            if let Some(x) = flowers.get(flower_i) {
                                artifacts.push(*x);
                            }
                            if let Some(x) = feathers.get(feather_i) {
                                artifacts.push(*x);
                            }
                            if let Some(x) = sands.get(sand_i) {
                                artifacts.push(*x);
                            }
                            if let Some(x) = goblets.get(goblet_i) {
                                artifacts.push(*x);
                            }
                            if let Some(x) = heads.get(head_i) {
                                artifacts.push(*x);
                            }

                            let artifact_list = ArtifactList {
                                artifacts
                            };

                            let attribute = AttributeUtils::create_attribute_from_big_config(
                                &artifact_list,
                                &input.artifact_config,
                                &character,
                                &weapon,
                                // &input.buffs,
                                &vec![],
                            );

                            if !OptimizeArtifactInterface::check_attribute(&attribute, &input.constraint) {
                                continue;
                            }

                            let value = target_function.target(&attribute, &enemy);

                            let intermediate = OptimizationIntermediateResult {
                                flower_index: flower_i,
                                feather_index: feather_i,
                                sand_index: sand_i,
                                goblet_index: goblet_i,
                                head_index: head_i,
                                attribute_graph: attribute,
                                value,
                            };

                            if heap.len() < RECORDS_COUNT {
                                heap.push(Reverse(intermediate));
                            } else {
                                heap.push(Reverse(intermediate));
                                heap.pop();
                            }
                        }
                    }
                }
            }
        }

        let mut results: Vec<OptimizationResult> = vec![];
        for i in heap.iter() {
            let intermediate = &i.0;
            let flower: Option<Artifact> = flowers.get(intermediate.flower_index).cloned().cloned();
            let feather: Option<Artifact> = feathers.get(intermediate.feather_index).cloned().cloned();
            let sand: Option<Artifact> = sands.get(intermediate.sand_index).cloned().cloned();
            let goblet: Option<Artifact> = goblets.get(intermediate.goblet_index).cloned().cloned();
            let head: Option<Artifact> = heads.get(intermediate.head_index).cloned().cloned();

            let attribute_no_reactive = AttributeNoReactive::from(&intermediate.attribute_graph);

            let optimization_result = OptimizationResult {
                flower,
                feather,
                sand,
                goblet,
                head,
                attribute: attribute_no_reactive,
                per_stat_bonus: Default::default(),
                value: intermediate.value,
            };
            results.push(optimization_result);
        }

        JsValue::from_serde(&results).unwrap()
    }

    pub fn hello() -> String {
        return String::from("asd");
    }
}