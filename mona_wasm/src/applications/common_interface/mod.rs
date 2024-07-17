use wasm_bindgen::prelude::*;
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::artifacts::Artifact;
use serde::Serialize;

pub mod get_attribute;

pub struct CommonInterface {}

#[wasm_bindgen]
impl CommonInterface {
    pub fn get_attribute(val: JsValue) -> JsValue {
        get_attribute::get_attribute(val)
    }

    pub fn get_artifacts_rank_by_character(character: JsValue, weapon: JsValue, tf: JsValue, artifacts: JsValue) -> JsValue {
        let character_interface: CharacterInterface = serde_wasm_bindgen::from_value(character).unwrap();
        let weapon_interface: WeaponInterface = serde_wasm_bindgen::from_value(weapon).unwrap();
        let tf_interface: TargetFunctionInterface = serde_wasm_bindgen::from_value(tf).unwrap();

        let artifacts: Vec<Artifact> = serde_wasm_bindgen::from_value(artifacts).unwrap();

        let character = character_interface.to_character();
        let weapon = weapon_interface.to_weapon(&character);
        let target_function = tf_interface.to_target_function(&character, &weapon);

        let optim_config = target_function.get_target_function_opt_config();

        let mut scores: Vec<(u64, f64)> = Vec::new();
        for artifact in artifacts.iter() {
            let id = artifact.id;
            let score = optim_config.score_normalized(artifact);

            scores.push((id, score));
        }

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        scores.serialize(&s).unwrap()
    }
}
