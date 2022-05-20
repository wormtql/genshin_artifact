use std::collections::HashMap;
use rand::Rng;
use crate::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use crate::artifacts::eff::ARTIFACT_EFF5;
use crate::common::StatName;

pub fn get_per_slot_artifacts<'a>(artifacts: &[&'a Artifact]) -> (Vec<&'a Artifact>, Vec<&'a Artifact>, Vec<&'a Artifact>, Vec<&'a Artifact>, Vec<&'a Artifact>) {
    let mut flowers = Vec::new();
    let mut feathers = Vec::new();
    let mut sands = Vec::new();
    let mut goblets = Vec::new();
    let mut heads = Vec::new();

    for &artifact in artifacts.iter() {
        match artifact.slot {
            ArtifactSlotName::Flower => flowers.push(artifact),
            ArtifactSlotName::Feather => feathers.push(artifact),
            ArtifactSlotName::Sand => sands.push(artifact),
            ArtifactSlotName::Goblet => goblets.push(artifact),
            ArtifactSlotName::Head => heads.push(artifact)
        }
    }

    (flowers, feathers, sands, goblets, heads)
}

pub fn get_artifacts_by_id<'a>(artifacts: &[&'a Artifact]) -> HashMap<u64, &'a Artifact> {
    let mut result: HashMap<u64, &'a Artifact> = HashMap::new();

    for artifact in artifacts.iter() {
        result.insert(artifact.id, *artifact);
    }

    result
}
