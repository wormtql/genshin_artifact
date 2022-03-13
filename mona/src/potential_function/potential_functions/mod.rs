pub mod artifact_eff;

use crate::potential_function::potential_function::PotentialFunction;
use crate::potential_function::potential_function_config::PotentialFunctionConfig;
use crate::potential_function::potential_function_name::PotentialFunctionName;

pub use artifact_eff::PotentialFunctionArtifactEff;

pub fn create_potential_function(name: PotentialFunctionName, config: &PotentialFunctionConfig) -> Box<dyn PotentialFunction> {
    name.create(config)
}
