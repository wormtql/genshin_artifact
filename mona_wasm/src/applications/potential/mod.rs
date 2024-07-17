use wasm_bindgen::prelude::*;
use crate::applications::common::PotentialFunctionInterface;
use mona::artifacts::Artifact;
use mona::potential_function::potential_function::calc_potential;
use mona::utils::{set_panic_hook};
use serde::Serialize;

pub struct PotentialInterface;

pub fn get_potential(artifacts: &[Artifact], pf_interface: &PotentialFunctionInterface) -> Vec<(u64, f64)> {
    let potential_function = pf_interface.to_pf();

    let mut results = Vec::new();
    for artifact in artifacts.iter() {
        let score = calc_potential(&potential_function, artifact);
        results.push((artifact.id, score));
    }

    results
}

#[wasm_bindgen]
impl PotentialInterface {
    pub fn get_potential(artifacts: JsValue, pf_interface: JsValue) -> JsValue {
        set_panic_hook();

        let artifacts: Vec<Artifact> = serde_wasm_bindgen::from_value(artifacts).unwrap();
        let pf_interface = serde_wasm_bindgen::from_value(pf_interface).unwrap();

        let mut results = get_potential(&artifacts, &pf_interface);
        results.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        results.serialize(&s).unwrap()
    }
}
