use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::applications::bonus_per_stat::bonus_per_stat::{BonusPerStatInput, BonusPerStatOutput};
use crate::applications::common::{BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use mona::artifacts::Artifact;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::target_functions::TargetFunction;
use mona::utils;
use crate::target_function::dsl_tf::TargetFunctionDSL;
use super::bonus_per_stat::bonus_per_stat;

#[derive(Serialize, Deserialize)]
pub struct WasmInput {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub artifacts: Vec<Artifact>,
    pub tf: TargetFunctionInterface,
    pub buffs: Vec<BuffInterface>,
    pub artifacts_config: Option<ArtifactEffectConfig>
}

// #[wasm_bindgen]
// pub struct WasmOutput {
//     pub atk_ptr: *const f64,
//     pub atk_len: usize,
//     pub atk_percentage_ptr: *const f64,
//     pub atk_percentage_len: usize,
//     pub def_ptr: *const f64,
//     pub def_len: usize,
//     pub def_percentage_ptr: *const f64,
//     pub def_percentage_len: usize,
//     pub hp_ptr: *const f64,
//     pub hp_len: usize,
//     pub hp_percentage_ptr: *const f64,
//     pub hp_percentage_len: usize,
//     pub critical_ptr: *const f64,
//     pub critical_len: usize,
//     pub critical_damage_ptr: *const f64,
//     pub critical_damage_len: usize,
//     pub recharge_ptr: *const f64,
//     pub recharge_len: usize,
//     pub elemental_mastery_ptr: *const f64,
//     pub elemental_mastery_len: usize,
// }

pub struct BonusPerStat;

#[wasm_bindgen]
impl BonusPerStat {
    pub fn bonus_per_stat(val: JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: WasmInput = serde_wasm_bindgen::from_value(val).unwrap();

        let character = input.character.to_character();
        let weapon = input.weapon.to_weapon(&character);
        let artifacts_ref: Vec<&Artifact> = input.artifacts.iter().collect();
        let tf: Box<dyn TargetFunction> = if input.tf.use_dsl {
            Box::new(TargetFunctionDSL::new(&input.tf.dsl_source.unwrap()))
        } else {
            input.tf.to_target_function(&character, &weapon)
        };
        let buffs: Vec<_> = input.buffs.iter().map(|b| b.to_buff()).collect();
        let config_ref = input.artifacts_config.as_ref();

        let result = bonus_per_stat(BonusPerStatInput {
            character: &character,
            weapon: &weapon,
            artifacts: &artifacts_ref,
            enemy: &Default::default(),
            tf: &tf,
            buffs: &buffs,
            artifacts_config: config_ref
        });

        // utils::log!("{:?}", result.atk);
        // utils::log!("{:?}", result.atk.as_ptr());

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()

        // WasmOutput {
        //     atk_ptr: result.atk.as_ptr(),
        //     atk_len: result.atk.len(),
        //     atk_percentage_ptr: result.atk_percentage.as_ptr(),
        //     atk_percentage_len: result.atk_percentage.len(),
        //     def_ptr: result.def.as_ptr(),
        //     def_len: result.def.len(),
        //     def_percentage_ptr: result.def_percentage.as_ptr(),
        //     def_percentage_len: result.def_percentage.len(),
        //     hp_ptr: result.hp.as_ptr(),
        //     hp_len: result.hp.len(),
        //     hp_percentage_ptr: result.hp_percentage.as_ptr(),
        //     hp_percentage_len: result.hp_percentage.len(),
        //     critical_ptr: result.critical_rate.as_ptr(),
        //     critical_len: result.critical_rate.len(),
        //     critical_damage_ptr: result.critical_damage.as_ptr(),
        //     critical_damage_len: result.critical_damage.len(),
        //     recharge_ptr: result.recharge.as_ptr(),
        //     recharge_len: result.recharge.len(),
        //     elemental_mastery_ptr: result.elemental_mastery.as_ptr(),
        //     elemental_mastery_len: result.elemental_mastery.len()
        // }
    }
}
