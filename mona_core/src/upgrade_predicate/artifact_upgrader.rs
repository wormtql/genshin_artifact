use crate::artifacts::Artifact;
use crate::upgrade_predicate::predicates::all_true_predicate::AllTruePredicate;
use crate::upgrade_predicate::upgrade_predicate::UpgradePredicate;

pub struct ArtifactUpgrader {
    pub predicate: Box<dyn UpgradePredicate>,
}

impl Default for ArtifactUpgrader {
    fn default() -> Self {
        ArtifactUpgrader {
            predicate: Box::new(AllTruePredicate)
        }
    }
}

impl ArtifactUpgrader {
    pub fn upgrade(&self, a: &mut Artifact) -> bool {
        while !a.is_max_level() {
            if self.predicate.can_do_upgrade(&a) {
                a.upgrade();
            } else {
                return false;
            }
        }
        return true;
    }
}
