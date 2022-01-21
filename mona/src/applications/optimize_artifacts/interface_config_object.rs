use std::collections::{BinaryHeap, HashMap};
use std::cmp::{PartialOrd, Eq, PartialEq, Ord, Ordering, Reverse};

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use crate::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName, ArtifactList};
use crate::character::{CharacterConfig, CharacterName, Character};
use crate::weapon::{WeaponName, WeaponConfig, Weapon};
use crate::target_functions::{TargetFunctionName, TargetFunctionConfig, TargetFunctionUtils};
use crate::buffs::{Buff, BuffType};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeNoReactive, AttributeName, AttributeUtils, AttributeCommon, Attribute, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use crate::enemies::Enemy;
use crate::{utils};
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};


const TOO_LARGE_ITER_COUNT: usize = 1000000;

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

    pub hp_min: Option<f64>,
    pub atk_min: Option<f64>,
    pub def_min: Option<f64>,
    pub recharge_min: Option<f64>,
    pub em_min: Option<f64>,
    pub crit_min: Option<f64>,
    pub crit_dmg_min: Option<f64>,
}

impl Default for ConstraintConfig {
    fn default() -> Self {
        ConstraintConfig {
            set_mode: None,
            hp_min: None,
            atk_min: None,
            def_min: None,
            recharge_min: None,
            em_min: None,
            crit_min: None,
            crit_dmg_min: None
        }
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

#[derive(Serialize, Deserialize)]
pub struct SimpleOptimizeArtifactInterface {
    pub artifacts: Vec<Artifact>,
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub target_function: TargetFunctionInterface,
    pub constraint: Option<ConstraintConfig>,
    pub buffs: Vec<BuffType>
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

struct OptimizationIntermediateResult {
    flower_index: usize,
    feather_index: usize,
    sand_index: usize,
    goblet_index: usize,
    head_index: usize,
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

impl OptimizeArtifactInterface {
    pub fn optimize_internal(
        artifacts: &Vec<Artifact>,
        artifact_config: &ArtifactEffectConfig,
        character_interface: &CharacterInterface,
        weapon_interface: &WeaponInterface,
        target_function_interface: &TargetFunctionInterface,
        constraint: &ConstraintConfig,
        buffs_interface: &Vec<BuffType>
    ) -> Vec<OptimizationResult> {
        let character = character_interface.to_character();
        let weapon = weapon_interface.to_weapon(&character);
        let target_function = target_function_interface.to_target_function(&character, &weapon);
        let enemy = Enemy::default();
        let buffs: Vec<Box<dyn Buff<SimpleAttributeGraph2>>> = buffs_interface.iter().map(|bt| bt.into()).collect();

        let mut flowers: Vec<&Artifact> = vec![];
        let mut feathers: Vec<&Artifact> = vec![];
        let mut sands: Vec<&Artifact> = vec![];
        let mut goblets: Vec<&Artifact> = vec![];
        let mut heads: Vec<&Artifact> = vec![];

        let mut artifacts: Vec<&Artifact> = artifacts.iter().collect();
        if OptimizeArtifactInterface::get_iteration_count(&artifacts) > TOO_LARGE_ITER_COUNT {
            // utils::log!("need optimization");
            // if iteration count too large, use heuristics cutoff
            let target_function_opt_config = target_function.get_target_function_opt_config();
            artifacts = target_function_opt_config.filter(artifacts);
        }
        let iter_count = OptimizeArtifactInterface::get_iteration_count(&artifacts);
        utils::log!("iter count: {}", iter_count);

        // construct artifact array of each slot
        for &artifact in artifacts.iter() {
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
                if !OptimizeArtifactInterface::check_artifact_set(
                    &vec![
                        flowers.get(flower_i).cloned(),
                        feathers.get(feather_i).cloned()],
                    constraint
                ) {
                    continue;
                }
                for sand_i in 0..=sands.len() {
                    if !OptimizeArtifactInterface::check_artifact_set(
                        &vec![
                            flowers.get(flower_i).cloned(),
                            feathers.get(feather_i).cloned(),
                            sands.get(sand_i).cloned()
                        ],
                        constraint
                    ) {
                        continue;
                    }
                    for goblet_i in 0..=goblets.len() {
                        if !OptimizeArtifactInterface::check_artifact_set(
                            &vec![
                                flowers.get(flower_i).cloned(),
                                feathers.get(feather_i).cloned(),
                                sands.get(feather_i).cloned(),
                                goblets.get(goblet_i).cloned(),
                            ],
                            constraint
                        ) {
                            continue;
                        }
                        for head_i in 0..=heads.len() {
                            let list = vec![
                                flowers.get(flower_i).cloned(),
                                feathers.get(feather_i).cloned(),
                                sands.get(feather_i).cloned(),
                                goblets.get(goblet_i).cloned(),
                                heads.get(head_i).cloned(),
                            ];
                            if !OptimizeArtifactInterface::check_artifact_set(
                                &list, &constraint
                            ) {
                                continue;
                            }

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

                            let mut attribute = AttributeUtils::create_attribute_from_big_config(
                                &artifact_list,
                                artifact_config,
                                &character,
                                &weapon,
                                &buffs
                            );

                            if !OptimizeArtifactInterface::check_attribute(&attribute, constraint) {
                                continue;
                            }

                            let value = target_function.target(&attribute, &character, &weapon, &enemy);

                            let intermediate = OptimizationIntermediateResult {
                                flower_index: flower_i,
                                feather_index: feather_i,
                                sand_index: sand_i,
                                goblet_index: goblet_i,
                                head_index: head_i,
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

        let character_comp: Character<ComplicatedAttributeGraph> = Character::new(
            character_interface.name,
            character_interface.level,
            character_interface.ascend,
            character_interface.constellation,
            character_interface.skill1,
            character_interface.skill2,
            character_interface.skill3,
            &character_interface.params
        );
        let weapon_comp: Weapon<ComplicatedAttributeGraph> = Weapon::new(
            weapon_interface.name,
            weapon_interface.level,
            weapon_interface.ascend,
            weapon_interface.refine,
            &weapon_interface.params,
            &character_comp
        );
        let buffs_comp: Vec<Box<dyn Buff<ComplicatedAttributeGraph>>> = buffs_interface.iter().map(|bt| bt.into()).collect();
        let mut results: Vec<OptimizationResult> = vec![];
        for i in heap.iter() {
            let intermediate = &i.0;
            let flower: Option<Artifact> = flowers.get(intermediate.flower_index).cloned().cloned();
            let feather: Option<Artifact> = feathers.get(intermediate.feather_index).cloned().cloned();
            let sand: Option<Artifact> = sands.get(intermediate.sand_index).cloned().cloned();
            let goblet: Option<Artifact> = goblets.get(intermediate.goblet_index).cloned().cloned();
            let head: Option<Artifact> = heads.get(intermediate.head_index).cloned().cloned();

            let mut artifact_list: Vec<&Artifact> = Vec::new();
            if let Some(ref x) = flower {
                artifact_list.push(x);
            }
            if let Some(ref x) = feather {
                artifact_list.push(x);
            }
            if let Some(ref x) = sand {
                artifact_list.push(x);
            }
            if let Some(ref x) = goblet {
                artifact_list.push(x);
            }
            if let Some(ref x) = head {
                artifact_list.push(x);
            }

            let attribute_comp: ComplicatedAttributeGraph = AttributeUtils::create_attribute_from_big_config(
                &ArtifactList { artifacts: artifact_list },
                artifact_config,
                &character_comp,
                &weapon_comp,
                &buffs_comp
            );

            let attribute_no_reactive = AttributeNoReactive::from(&attribute_comp);

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

        results
    }

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

    fn get_iteration_count(artifacts: &[&Artifact]) -> usize {
        let mut map: HashMap<ArtifactSlotName, usize> = HashMap::new();

        for art in artifacts.iter() {
            *map.entry(art.slot).or_insert(0) += 1;
        }

        map.values().fold(0, |x, y| (x + 1) * (y + 1))
    }
}

#[wasm_bindgen]
impl OptimizeArtifactInterface {
    pub fn optimize(val: &JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: SimpleOptimizeArtifactInterface = val.into_serde().unwrap();

        let character = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);
        let target_function = input.target_function.to_target_function(&character, &weapon);

        let default_artifact_config = target_function.get_default_artifact_config(&Default::default());

        let constraint = match input.constraint {
            Some(x) => x,
            None => Default::default()
        };

        let results = OptimizeArtifactInterface::optimize_internal(
            &input.artifacts,
            &default_artifact_config,
            &input.character,
            &input.weapon,
            &input.target_function,
            &constraint,
            &input.buffs
        );

        JsValue::from_serde(&results).unwrap()
    }

    pub fn hello() -> String {
        return String::from("asd");
    }
}