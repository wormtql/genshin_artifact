use std::collections::{BinaryHeap, HashMap};
use std::cmp::{PartialOrd, Eq, PartialEq, Ord, Ordering, Reverse};

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

use crate::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName, ArtifactList};
use crate::character::{CharacterConfig, CharacterName, Character};
use crate::weapon::{WeaponName, WeaponConfig, Weapon};
use crate::target_functions::{TargetFunctionName, TargetFunctionConfig, TargetFunctionUtils};
use crate::buffs::{Buff, BuffConfig};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeNoReactive, AttributeName, AttributeUtils, AttributeCommon, Attribute, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use crate::enemies::Enemy;
use crate::{utils};
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::inter::OptimizeArtifactInterface;
use crate::applications::optimize_artifacts::single_optimize::optimize_single;

pub struct OptimizeSingleWasm;

#[wasm_bindgen]
impl OptimizeSingleWasm {
    pub fn optimize(val: &JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: OptimizeArtifactInterface = match val.into_serde() {
            Ok(x) => x,
            Err(e) => panic!("{}", e)
        };

        let character = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);
        let target_function = input.target_function.to_target_function(&character, &weapon);
        let artifacts_ref: Vec<&Artifact> = input.artifacts.iter().collect();
        let constraint_ref = input.constraint.as_ref();
        let buffs: Vec<Box<dyn Buff<SimpleAttributeGraph2>>> = input.buffs.iter().map(|x| x.to_buff()).collect();

        let results = optimize_single(
            &artifacts_ref,
            input.artifact_config,
            &character,
            &weapon,
            &target_function,
            constraint_ref,
            &buffs,
            100
        );

        JsValue::from_serde(&results).unwrap()
    }
}
