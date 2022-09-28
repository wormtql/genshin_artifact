use crate::artifacts::Artifact;

pub trait UpgradePredicate {
    fn can_do_upgrade(&self, a: &Artifact) -> bool;
}
