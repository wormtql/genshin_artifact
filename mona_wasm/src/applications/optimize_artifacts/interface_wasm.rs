use std::collections::{BinaryHeap, HashMap};
use std::cmp::{PartialOrd, Eq, PartialEq, Ord, Ordering, Reverse};

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName, ArtifactList};
use mona::character::{CharacterConfig, CharacterName, Character};
use mona::weapon::{WeaponName, WeaponConfig, Weapon};
use mona::target_functions::{TargetFunctionName, TargetFunctionConfig, TargetFunctionUtils, TargetFunction};
use mona::buffs::{Buff, BuffConfig};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::{AttributeNoReactive, AttributeName, AttributeUtils, AttributeCommon, Attribute, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::enemies::Enemy;
use mona::{utils};
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::inter::OptimizeArtifactInterface;
use crate::target_function::dsl_tf::TargetFunctionDSL;

pub struct OptimizeSingleWasm;

#[wasm_bindgen]
impl OptimizeSingleWasm {
    pub fn optimize(val: JsValue, artifacts: JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: OptimizeArtifactInterface = match serde_wasm_bindgen::from_value(val) {
            Ok(x) => x,
            Err(e) => panic!("{}", e)
        };
        let artifacts: Vec<Artifact> = serde_wasm_bindgen::from_value(artifacts).unwrap();
        let artifacts_ref: Vec<_> = artifacts.iter().collect();

        let character = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);
        // let target_function = input.target_function.to_target_function(&character, &weapon);
        let target_function: Box<dyn TargetFunction> = if input.target_function.use_dsl {
            Box::new(TargetFunctionDSL::new(&input.target_function.dsl_source.unwrap()))
        } else {
            input.target_function.to_target_function(&character, &weapon)
        };
        let constraint = input.constraint.unwrap_or(Default::default());
        let buffs: Vec<Box<dyn Buff<SimpleAttributeGraph2>>> = input.buffs.iter().map(|x| x.to_buff()).collect();
        let artifact_config = input.artifact_config.as_ref().map(|x| x.clone().to_config());

        let filtered_artifacts = input.filter.as_ref().map(|x| x.filter_artifact(&artifacts_ref));
        let artifacts = match filtered_artifacts {
            Some(ref a) => a.as_slice(),
            None => &artifacts_ref
        };

        let algorithm = input.algorithm.get_algorithm();

        let result = algorithm.optimize(
            &artifacts,
            artifact_config,
            &character,
            &weapon,
            &target_function,
            &Default::default(),
            &buffs,
            &constraint,
            100
        );

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()
    }
}
