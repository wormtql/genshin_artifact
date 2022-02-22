use crate::artifacts::Artifact;
use crate::artifacts::eff::ARTIFACT_EFF5;
use crate::common::StatName;

// fn potential_expect_helper(artifact: &Artifact, depth: usize, func: &Box<dyn PotentialFunction>, valid_list: &[StatName]) -> f64 {
//
// }

pub trait PotentialFunction {
    const EXPECT: bool;

    fn potential(&self, artifact: &Artifact) -> f64;

    fn get_effective_stats(&self) -> Vec<StatName>;

    fn do_potential(&self, artifact: &Artifact) -> f64 {
        if !Self::EXPECT {
            self.potential(artifact)
        } else {
            0.0

        }
    }
}
