use std::cmp::Ordering;
use mona::artifacts::artifact_set_type::ArtifactSetType;
use mona::common::StatName;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub set_type: ArtifactSetType,
    pub stats: [StatName; 3],
    pub value: f64,
}

impl PartialOrd for UpdateItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl Ord for UpdateItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for UpdateItem {}

impl PartialEq for UpdateItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}