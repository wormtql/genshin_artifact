use serde::{Serialize, Deserialize};

use crate::applications::optimize_artifacts::algorithms::cutoff_heuristic::CutoffAlgorithmHeuristic;
use crate::applications::optimize_artifacts::algorithms::CutoffAlgorithm2;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::buffs::Buff;
use crate::character::Character;
use crate::enemies::Enemy;
use crate::target_functions::TargetFunction;
use crate::weapon::Weapon;

pub trait SingleOptimizeAlgorithm {
    fn optimize(
        &self,
        artifacts: &[&Artifact],
        artifact_config: Option<ArtifactEffectConfig>,
        character: &Character<SimpleAttributeGraph2>,
        weapon: &Weapon<SimpleAttributeGraph2>,
        target_function: &Box<dyn TargetFunction>,
        enemy: &Enemy,
        buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>],
        constraint: &ConstraintConfig,
        count: usize
    ) -> Vec<OptimizationResult>;
}

#[derive(Serialize, Deserialize)]
pub enum SingleOptimizeAlgorithmName {
    AStar,
    Naive,
    Heuristic,
}

impl SingleOptimizeAlgorithmName {
    pub fn get_algorithm(&self) -> Box<dyn SingleOptimizeAlgorithm> {
        match *self {
            SingleOptimizeAlgorithmName::AStar => Box::new(CutoffAlgorithm2),
            SingleOptimizeAlgorithmName::Naive => Box::new(CutoffAlgorithmHeuristic { use_heuristic: false }),
            SingleOptimizeAlgorithmName::Heuristic => Box::new(CutoffAlgorithmHeuristic { use_heuristic: true })
        }
    }
}
