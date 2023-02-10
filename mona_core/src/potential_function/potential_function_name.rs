use serde::{Serialize, Deserialize};
use mona_derive::{PotentialFunctionData, EnumLen};
use strum_macros::{Display, EnumIter};
use num_derive::FromPrimitive;

#[derive(PotentialFunctionData, EnumLen, EnumIter)]
#[derive(Copy, Clone, Display, FromPrimitive)]
#[derive(Serialize, Deserialize)]
pub enum PotentialFunctionName {
    ArtifactEff
}
