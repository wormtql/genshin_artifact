use crate::artifacts::Artifact;
use crate::upgrade_predicate::upgrade_predicate::UpgradePredicate;

pub struct AllTruePredicate;

impl UpgradePredicate for AllTruePredicate {
    fn can_do_upgrade(&self, a: &Artifact) -> bool {
        true
    }
}
