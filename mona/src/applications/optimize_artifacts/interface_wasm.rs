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
use crate::applications::optimize_artifacts::single_optimize::{optimize_single, optimize_single_interface_wasm};

pub struct OptimizeSingleWasm;

#[wasm_bindgen]
impl OptimizeSingleWasm {
    pub fn optimize(val: &JsValue, artifacts: &JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: OptimizeArtifactInterface = match val.into_serde() {
            Ok(x) => x,
            Err(e) => panic!("{}", e)
        };
        let artifacts: Vec<Artifact> = artifacts.into_serde().unwrap();
        let artifacts_ref: Vec<_> = artifacts.iter().map(|x| x).collect();

        let results = optimize_single_interface_wasm(&input, &artifacts_ref);

        JsValue::from_serde(&results).unwrap()
    }
}
