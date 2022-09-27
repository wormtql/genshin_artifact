use crate::artifacts::ArtifactSetName;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ArtifactSetType {
    Set4(ArtifactSetName),
    Set22(ArtifactSetName, ArtifactSetName),
    Set2(ArtifactSetName),
    Misc,
}
