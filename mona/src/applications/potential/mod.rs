use wasm_bindgen::prelude::*;
use crate::applications::common::PotentialFunctionInterface;
use crate::artifacts::Artifact;
use crate::potential_function::potential_function::calc_potential;

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
    pub fn get_potential(artifacts: &JsValue, pf_interface: &JsValue) -> JsValue {
        let artifacts: Vec<Artifact> = artifacts.into_serde().unwrap();
        let pf_interface = pf_interface.into_serde().unwrap();

        let results = get_potential(&artifacts, &pf_interface);

        JsValue::from_serde(&results).unwrap()
    }
}
