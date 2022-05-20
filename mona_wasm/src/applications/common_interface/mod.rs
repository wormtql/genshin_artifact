use wasm_bindgen::prelude::*;
use crate::applications::common::{CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::artifacts::Artifact;

pub mod get_attribute;

pub struct CommonInterface {}

#[wasm_bindgen]
impl CommonInterface {
    pub fn get_attribute(val: &JsValue) -> JsValue {
        get_attribute::get_attribute(val)
    }

    pub fn get_artifacts_rank_by_character(character: &JsValue, weapon: &JsValue, tf: &JsValue, artifacts: &JsValue) -> JsValue {
        let character_interface: CharacterInterface = character.into_serde().unwrap();
        let weapon_interface: WeaponInterface = weapon.into_serde().unwrap();
        let tf_interface: TargetFunctionInterface = tf.into_serde().unwrap();

        let artifacts: Vec<Artifact> = artifacts.into_serde().unwrap();

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

        JsValue::from_serde(&scores).unwrap()
    }
}
