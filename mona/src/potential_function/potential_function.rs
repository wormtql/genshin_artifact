use crate::artifacts::Artifact;
use crate::artifacts::eff::ARTIFACT_EFF5;
use crate::common::item_config_type::ItemConfig;
use crate::common::StatName;
use crate::potential_function::potential_function_config::PotentialFunctionConfig;
use crate::potential_function::potential_function_engine::{ExpectationPotentialFunctionEngine, PotentialFunctionEngine};
use crate::potential_function::potential_function_name::PotentialFunctionName;

pub trait PotentialFunction {
    fn get_engine(&self) -> Box<dyn PotentialFunctionEngine> {
        Box::new(ExpectationPotentialFunctionEngine)
    }

    fn potential(&self, artifact: &Artifact) -> f64;

    fn get_effective_stats(&self) -> Vec<StatName>;
}

pub struct PotentialFunctionMetaData {
    pub name: PotentialFunctionName,
    pub chs: &'static str,
    pub description: &'static str,
    pub image: &'static str,
}

pub trait PotentialFunctionMeta {
    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = None;

    #[cfg(not(target_family = "wasm"))]
    const META: PotentialFunctionMetaData;

    fn create(config: &PotentialFunctionConfig) -> Box<dyn PotentialFunction>;
}

pub fn calc_potential(pf: &Box<dyn PotentialFunction>, artifact: &Artifact) -> f64 {
    let engine = pf.get_engine();
    engine.value(&pf, &artifact)
}
