use serde::{Serialize, Deserialize};
use crate::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use crate::applications::optimize_artifacts::algorithms::cutoff_heu_plus_a_star::CutoffHeuristicPlusAStar;
use crate::applications::optimize_artifacts::algorithms::cutoff_heuristic::CutoffAlgorithmHeuristic;
use crate::applications::optimize_artifacts::algorithms::CutoffAlgorithm2;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use mona::artifacts::Artifact;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::SimpleAttributeGraph2;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;

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

impl Default for SingleOptimizeAlgorithmName {
    fn default() -> Self {
        SingleOptimizeAlgorithmName::AStar
    }
}

impl SingleOptimizeAlgorithmName {
    pub fn get_algorithm(&self) -> Box<dyn SingleOptimizeAlgorithm> {
        match *self {
            SingleOptimizeAlgorithmName::AStar => Box::new(AStarCutoff),
            SingleOptimizeAlgorithmName::Naive => Box::new(CutoffAlgorithmHeuristic { use_heuristic: false }),
            SingleOptimizeAlgorithmName::Heuristic => Box::new(CutoffHeuristicPlusAStar)
        }
    }
}
