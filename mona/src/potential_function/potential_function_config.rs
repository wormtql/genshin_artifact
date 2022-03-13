use serde::{Serialize, Deserialize};
use crate::potential_function::potential_functions::artifact_eff::PotentialFunctionArtifactEffConfig;

#[derive(Serialize, Deserialize)]
pub enum PotentialFunctionConfig {
    NoConfig,
    ArtifactEff(PotentialFunctionArtifactEffConfig),
}
