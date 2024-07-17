use serde::Serialize;
use wasm_bindgen::JsValue;
use crate::applications::artifact_best_set::artifact_best_set::calc_artifact_best_set;
use crate::applications::artifact_best_set::type_interface::CalcArtifactBestSetInterface;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use crate::utils;

pub struct CalcArtifactBestSet;

#[wasm_bindgen]
impl CalcArtifactBestSet {
    pub fn calc_artifact_best_set(args: JsValue) -> JsValue {
        set_panic_hook();

        let calc_best_set_interface: CalcArtifactBestSetInterface = serde_wasm_bindgen::from_value(args).unwrap();

        let character = calc_best_set_interface.character.to_character();
        let weapon = calc_best_set_interface.weapon.to_weapon(&character);
        let target_function = calc_best_set_interface.target_function.to_target_function(&character, &weapon);
        let enemy = match calc_best_set_interface.enemy {
            Some(ref x) => x.to_enemy(),
            None => Default::default()
        };
        let buffs = calc_best_set_interface.buffs.unwrap_or(vec![]).iter().map(|b| b.to_buff()).collect::<Vec<_>>();
        let artifact_config = calc_best_set_interface.artifact_config.clone().map(|x| x.to_config());

        let mut result = calc_artifact_best_set(
            &character, &weapon, &target_function,
            artifact_config.as_ref(),
            &buffs,
            &enemy
        );
        // utils::log!("{:?}", result);

        let mut arr = Vec::new();

        while !result.is_empty() {
            arr.push(result.pop().unwrap());
        }
        // utils::log!("{:?}", arr);

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        arr.serialize(&s).unwrap()
    }
}
