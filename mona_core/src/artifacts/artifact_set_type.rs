use crate::artifacts::ArtifactSetName;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum ArtifactSetType {
    Set4(ArtifactSetName),
    Set22(ArtifactSetName, ArtifactSetName),
    Set2(ArtifactSetName),
    Misc,
}
